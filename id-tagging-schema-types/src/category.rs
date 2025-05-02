use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Category {
	pub icon: String,
	pub name: String,
	pub members: Vec<String>,
}
