#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Destination {
	Queue(String),
	Exchange(String),
}

impl Destination {
	pub fn queue(name: &str) -> Self {
		Self::Queue(name.into())
	}

	pub fn exchange(name: &str) -> Self {
		Self::Exchange(name.into())
	}
}
