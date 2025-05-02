mod category;
mod preset;

use serde::Deserialize;

pub use category::Category;
pub use preset::Preset;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Geometry {
	Point,
	Line,
	Area,
	Relation,
	Vertex,
}
