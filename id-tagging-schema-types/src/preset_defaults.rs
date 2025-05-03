use crate::Geometry;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct RawPresetDefaults {
	pub area: Vec<String>,
	pub line: Vec<String>,
	pub point: Vec<String>,
	pub vertex: Vec<String>,
	pub relation: Vec<String>,
}

#[derive(Debug)]
pub struct PresetDefaults {
	pub area: &'static [&'static str],
	pub line: &'static [&'static str],
	pub point: &'static [&'static str],
	pub vertex: &'static [&'static str],
	pub relation: &'static [&'static str],
}

impl PresetDefaults {
	pub fn get(&self, g: Geometry) -> &'static [&'static str] {
		match g {
			Geometry::Point => self.point,
			Geometry::Line => self.line,
			Geometry::Area => self.area,
			Geometry::Relation => self.relation,
			Geometry::Vertex => self.vertex,
		}
	}
}
