mod category;
mod preset;
mod preset_defaults;

pub use category::*;
pub use preset::*;
pub use preset_defaults::*;

#[derive(Debug, Copy, Clone, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Geometry {
	Point,
	Line,
	Area,
	Relation,
	Vertex,
}
