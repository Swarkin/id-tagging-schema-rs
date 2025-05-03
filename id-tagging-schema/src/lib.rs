pub mod category;
pub mod discarded;
pub mod preset;
pub mod preset_defaults;

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn category() {
		dbg!(category::BARRIER);
	}

	#[test]
	fn preset() {
		dbg!(preset::BARRIER);
	}

	#[test]
	fn preset_defaults() {
		dbg!(preset_defaults::PRESET_DEFAULTS);
	}

	#[test]
	fn discarded() {
		dbg!(discarded::DISCARDED);
	}
}
