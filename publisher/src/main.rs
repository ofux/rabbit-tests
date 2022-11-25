use anyhow::Result;
use dotenv::dotenv;
use infrastructure::amqp::{
	destination::Destination, message::Message, subscriber::CallbackError, Bus,
};

const EXCHANGE_NAME: &str = "my-exchange";

#[tokio::main]
async fn main() -> Result<()> {
	dotenv().ok();

	let publisher = Bus::default().await.unwrap();

	for i in 0..1000 {
		publish(
			Message {
				payload: format!("Payload#{i}").to_string(),
			},
			&publisher,
		)
		.await
		.unwrap();
	}
	Ok(())
}

async fn publish(message: Message, publisher: &Bus) -> Result<(), CallbackError> {
	publisher
		.publish(Destination::exchange(EXCHANGE_NAME), &message)
		.await
		.map_err(|e| CallbackError::Fatal(e.into()))?;
	Ok(())
}
