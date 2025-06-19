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

		writeln!(&mut out, "use id_tagging_schema_types::deprecated::DeprecatedMapping;\n")?;
		writeln!(&mut out, "pub static DEPRECATED: [DeprecatedMapping; {}] = [", data.len())?;

		for mapping in data {
			let mut phf_old = phf_codegen::Map::<&'static str>::new();
			for (k, v) in &mapping.old {
				phf_old.entry(k, format!("\"{v}\""));
			}

			let mut phf_replace = phf_codegen::Map::<&'static str>::new();
			for (k, v) in &mapping.replace {
				phf_replace.entry(k, format!("\"{v}\""));
			}

			writeln!(&mut out, "\tDeprecatedMapping {{ old: {}, replace: {} }},", phf_old.build(), phf_replace.build())?;
		}

		writeln!(&mut out, "];")?;
		out.flush()?;
	}

	Ok(())
}

fn osm_path(path: &str) -> PathBuf {
	PathBuf::from(format!("id-tagging-schema/dist/{path}"))
}

fn gen_path(path: &str) -> PathBuf {
	PathBuf::from(format!("{}/{path}", std::env::var("OUT_DIR").unwrap()))
}
