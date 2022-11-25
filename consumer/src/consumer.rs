use infrastructure::amqp::{Bus, BusError, ConsumableBus};
use lapin::options::QueueDeclareOptions;

const EXCHANGE_NAME: &str = "my-exchange";

pub async fn consumer(queue_name: &'static str) -> Result<ConsumableBus, BusError> {
	let bus = Bus::default()
		.await?
		.with_queue(
			queue_name,
			QueueDeclareOptions {
				exclusive: false, /* used by only one connection and the queue will be deleted
				                   * when that connection closes */
				durable: true, // the queue will survive a broker restart
				auto_delete: false, /* queue that has had at least one consumer is deleted when
				                * last consumer unsubscribes */
				..Default::default()
			},
		)
		.await?
		.with_exchange(EXCHANGE_NAME)
		.await?;

	log::info!("[{queue_name}] 🎧 Start listening to events");
	Ok(bus)
}
