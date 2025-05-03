use serde::Deserialize;

#[derive(Deserialize)]
pub struct RawCategory {
	pub icon: String,
	pub name: String,
	pub members: Vec<String>,
}

#[derive(Debug)]
pub struct Category {
	pub icon: &'static str,
	pub name: &'static str,
	pub members: &'static [&'static str],
}
