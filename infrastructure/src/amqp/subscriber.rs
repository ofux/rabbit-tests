use super::{message::Message, ConsumableBus};
use lapin::{message::Delivery, options::BasicNackOptions};
use serde_json::Error;
use std::future::Future;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CallbackError {
	// Returning a Discard error will discard the current message and process the next one. It
	// won't be requeued.
	#[error("Ignoring message")]
	Discard(#[source] anyhow::Error),

	// Returning an Fatal error will stop the consuming of messages. The current message
	// must be automatically requeued by the message broker.
	#[error("Fatal error while processing the message")]
	Fatal(#[from] anyhow::Error),
}

impl ConsumableBus {
	pub async fn subscribe<C, F>(&self, callback: C) -> anyhow::Result<()>
	where
		C: Fn(Message) -> F + Send + Sync,
		F: Future<Output = Result<(), CallbackError>> + Send,
	{
		while let Some(delivery) = self.consume().await? {
			let message: Result<Message, Error> = serde_json::from_slice(&delivery.data);
			let message = match message {
				Ok(message) => message,
				Err(error) => {
					log::error!("Failed to deserialize message: {error}",);
					Self::discard_message(&delivery).await?;
					continue;
				},
			};

			match callback(message).await {
				Ok(_) => delivery.ack(Default::default()).await?,

				Err(error) => match error {
					CallbackError::Discard(error) => {
						log::error!("Ignoring message: {error}",);
						Self::discard_message(&delivery).await?;
						continue;
					},
					CallbackError::Fatal(error) => {
						log::error!("Fatal error while processing the message: {error}",);
						return Err(error.into());
					},
				},
			}
		}

		Ok(())
	}
}

impl ConsumableBus {
	async fn discard_message(delivery: &Delivery) -> anyhow::Result<()> {
		delivery
			.nack(BasicNackOptions {
				requeue: false,
				..Default::default()
			})
			.await
			.map_err(|e| e.into())
	}
}
