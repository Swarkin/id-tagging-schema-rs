use crate::Geometry;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct RawPreset {
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

#[derive(Debug)]
pub struct Preset {
	pub icon: Option<&'static str>,
	pub name: &'static str,
	pub fields: &'static [&'static str],
	pub more_fields: &'static [&'static str],
	pub geometry: &'static [Geometry],
	pub tags: phf::Map<&'static str, &'static str>,
	pub terms: &'static [&'static str],
	pub match_score: Option<f32>,
}
