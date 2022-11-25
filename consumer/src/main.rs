use anyhow::Result;
use dotenv::dotenv;
use futures::future::try_join_all;
use infrastructure::amqp::{message::Message, ConsumableBus};
use tokio::task::JoinHandle;

mod consumer;

#[tokio::main]
async fn main() -> Result<()> {
	dotenv().ok();
	let consumer_a = consumer::consumer("queue_a").await.unwrap();
	let consumer_b = consumer::consumer("queue_b").await.unwrap();

	let handles = [spawn("a", consumer_a), spawn("b", consumer_b)];

	try_join_all(handles).await?;

	Ok(())
}

fn spawn(name: &'static str, bus: ConsumableBus) -> JoinHandle<()> {
	tokio::spawn(async move {
		bus.subscribe(|message: Message| async move {
			log::info!("QUEUE[{name}] Consumed message {}", message.payload);
			Ok(())
		})
		.await
		.expect("Failed while trying to consume message");
	})
}
