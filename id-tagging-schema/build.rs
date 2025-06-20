use id_tagging_schema_types::deprecated::RawDeprecatedMapping;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::path::PathBuf;

fn main() -> std::io::Result<()> {
	/* discarded */ {
		let mut out = BufWriter::new(File::create(gen_path("discarded.rs"))?);

		let json_file = BufReader::new(File::open(osm_path("discarded.min.json"))?);
		let data: HashMap<String, bool> = serde_json::from_reader(json_file)?;

		let mut phf = phf_codegen::Set::<&str>::new();
		for key in data.keys() {
			phf.entry(key);
		}

		writeln!(&mut out, "pub static DISCARDED: phf::Set<&'static str> = {};", phf.build(),)?;
		out.flush()?;
	}

	/* deprecated */ {
		let mut out = BufWriter::new(File::create(gen_path("deprecated.rs"))?);

		let json_file = BufReader::new(File::open(osm_path("deprecated.min.json"))?);
		let data: Vec<RawDeprecatedMapping> = serde_json::from_reader(json_file)?;

		writeln!(&mut out, "use id_tagging_schema_types::deprecated::DeprecatedMapping;")?;
		write!(&mut out, "pub static DEPRECATED: [DeprecatedMapping; {}] = [", data.len())?;

		for mapping in data {
			let mut phf_old = phf_codegen::Map::<&'static str>::new();
			for (k, v) in &mapping.old {
				phf_old.entry(k, format!("\"{v}\""));
			}

			let mut phf_replace = phf_codegen::Map::<&'static str>::new();
			for (k, v) in &mapping.replace {
				phf_replace.entry(k, format!("\"{v}\""));
			}

			write!(&mut out, "DeprecatedMapping {{ old: {}, replace: {} }},", phf_old.build(), phf_replace.build())?;
		}

		write!(&mut out, "];")?;
		out.flush()?;
	}

	/* preset defaults */ {
		let mut out = BufWriter::new(File::create(gen_path("preset_defaults.rs"))?);

		let json_file = BufReader::new(File::open(osm_path("preset_defaults.min.json"))?);
		let data: HashMap<String, Vec<String>> = serde_json::from_reader(json_file)?;

		for (k, v) in &data {
			write!(&mut out, "pub static {}: [&'static str; {}] = [", k.to_uppercase(), v.len())?;
			for entry in v {
				write!(&mut out, "\"{entry}\", ")?;
			}
			writeln!(&mut out, "];")?;
		}
	}

	Ok(())
}

fn osm_path(path: &str) -> PathBuf {
	PathBuf::from(format!("id-tagging-schema/dist/{path}"))
}

fn gen_path(path: &str) -> PathBuf {
	PathBuf::from(format!("{}/{path}", std::env::var("OUT_DIR").unwrap()))
}
