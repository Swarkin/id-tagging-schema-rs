use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug)]
pub struct DeprecatedMapping {
	pub old: phf::Map<&'static str, &'static str>,
	pub replace: phf::Map<&'static str, &'static str>,
}

#[derive(Deserialize)]
pub struct RawDeprecatedMapping {
	pub old: HashMap<String, String>,
	#[serde(default)]
	pub replace: HashMap<String, String>,
}
