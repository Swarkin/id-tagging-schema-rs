pub mod category;
pub mod preset;

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn category() {
		let _ = *category::BARRIER;
		dbg!(&category::BARRIER);
	}

	#[test]
	fn preset() {
		let _ = *preset::BARRIER;
		dbg!(&preset::BARRIER);
	}
}
