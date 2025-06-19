pub mod deprecated;
pub mod discarded;

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
}
