use super::{destination::Destination, message::Message, Bus};
use anyhow::anyhow;

impl Bus {
	pub async fn publish(&self, destination: Destination, message: &Message) -> anyhow::Result<()> {
		let (exchange_name, routing_key) = match destination {
			Destination::Queue(name) => (String::new(), name),
			Destination::Exchange(name) => (name, String::new()),
		};

		let confirmation = self
			.publish_internal(&exchange_name, &routing_key, &serde_json::to_vec(message)?)
			.await?;

		match confirmation.is_nack() {
			true => Err(anyhow!("nack")),
			false => Ok(()),
		}
	}
}
