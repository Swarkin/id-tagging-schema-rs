use crate::Geometry;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct Preset {
	pub icon: Option<String>,
	pub name: String,
	#[serde(skip_serializing_if = "Vec::is_empty", default)]
	pub fields: Vec<String>,
	#[serde(skip_serializing_if = "Vec::is_empty", default)]
	pub more_fields: Vec<String>,
	pub geometry: Vec<Geometry>,
	pub tags: HashMap<String, String>,
	#[serde(skip_serializing_if = "Vec::is_empty", default)]
	pub terms: Vec<String>,
	pub match_score: Option<f32>,
}
