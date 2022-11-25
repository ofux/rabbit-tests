use std::time::Duration;

use anyhow::Result;
use dotenv::dotenv;
use infrastructure::amqp::{
	destination::Destination, message::Message, subscriber::CallbackError, Bus,
};
use tokio::time::sleep;

const EXCHANGE_NAME: &str = "my-exchange";

#[tokio::main]
async fn main() -> Result<()> {
	dotenv().ok();
	env_logger::init();

	let publisher = Bus::default().await.unwrap();

	for i in 0..100 {
		publish(
			Message {
				payload: format!("Payload#{i}").to_string(),
			},
			&publisher,
		)
		.await
		.unwrap();
		sleep(Duration::from_millis(1000)).await
	}
	Ok(())
}

async fn publish(message: Message, publisher: &Bus) -> Result<(), CallbackError> {
	publisher
		.publish(Destination::exchange(EXCHANGE_NAME), &message)
		.await
		.map_err(|e| CallbackError::Fatal(e.into()))?;
	log::info!("Published message {}", message.payload);
	Ok(())
}
