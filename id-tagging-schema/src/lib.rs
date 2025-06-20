pub mod deprecated;
pub mod discarded;
pub mod preset_defaults;

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn discarded() {
		dbg!(&discarded::DISCARDED);
	}

	#[test]
	fn deprecated() {
		dbg!(&deprecated::DEPRECATED);
	}

	#[test]
	fn preset_defaults() {
		dbg!(&preset_defaults::AREA);
		dbg!(&preset_defaults::LINE);
		dbg!(&preset_defaults::POINT);
		dbg!(&preset_defaults::VERTEX);
		dbg!(&preset_defaults::RELATION);
	}
}
