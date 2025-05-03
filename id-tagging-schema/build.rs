use id_tagging_schema_types::*;
use std::collections::HashMap;
use std::fs::read_dir;
use std::path::PathBuf;

fn path(path: &str) -> String {
	format!("id-tagging-schema/data/{path}")
}

fn main() -> std::io::Result<()> {
	let out_dir = std::env::var("OUT_DIR").unwrap();

	// Categories
	let mut generated = "use id_tagging_schema_types::*;\n".to_string();

	let category_dir = PathBuf::from(path("preset_categories"));
	for entry in read_dir(category_dir)?.map(|x| x.unwrap()) {
		let path = entry.path();

		if entry.file_type()?.is_file() && path.extension().map(|x| x == "json").unwrap_or(false) {
			let json_str = std::fs::read_to_string(&path)?;
			let category = serde_json::from_str::<RawCategory>(&json_str)?;

			let ident = path
				.file_stem()
				.unwrap()
				.to_str()
				.unwrap()
				.replace(['-', '.'], "_")
				.to_uppercase();

			generated.push_str(&format!(
				concat!(
					"\npub const {}: Category = Category {{\n",
					"\ticon: {:?},\n",
					"\tname: {:?},\n",
					"\tmembers: &[{}],\n",
					"}};\n"
				),
				ident,
				category.icon,
				category.name,
				category
					.members
					.into_iter()
					.map(|x| format!("{x:?}, "))
					.collect::<String>()
			));
		}
	}

	let dest_path = PathBuf::from(&out_dir).join("category_data.rs");
	std::fs::write(dest_path, generated)?;

	// Presets
	let mut generated = concat!(
		"use phf::phf_map;\n",
		"use id_tagging_schema_types::{*, Geometry::*};\n",
	)
	.to_string();

	let preset_dir = PathBuf::from(path("presets"));
	for entry in read_dir(preset_dir)?.map(|x| x.unwrap()) {
		let path = entry.path();
		if entry.file_type()?.is_file() && path.extension().map(|x| x == "json").unwrap() && !path.starts_with("_") {
			let json_str = std::fs::read_to_string(&path)?;
			let preset = serde_json::from_str::<RawPreset>(&json_str)?;

			let ident = path
				.file_stem()
				.unwrap()
				.to_str()
				.unwrap()
				.replace(['-', '.'], "_")
				.to_uppercase();

			generated.push_str(&format!(
				concat!(
					"\npub const {}: Preset = Preset {{\n",
					"\ticon: {},\n",
					"\tname: {:?},\n",
					"\tfields: &[{}],\n",
					"\tmore_fields: &[{}],\n",
					"\tgeometry: &{:?},\n",
					"\ttags: phf_map! {{{}}},\n",
					"\tterms: &[{}],\n",
					"\tmatch_score: {},\n",
					"}};\n"
				),
				ident,
				{
					let mut string = String::new();
					match preset.icon {
						None => {
							string.push_str("None");
						}
						Some(icon) => {
							string.push_str("Some(\"");
							string.push_str(&icon);
							string.push_str("\")");
						}
					}
					string
				},
				preset.name,
				vec_to_str(preset.fields),
				vec_to_str(preset.more_fields),
				preset.geometry,
				{
					let mut string = String::new();
					for (k, v) in preset.tags {
						string.push_str(&format!("\t\"{k}\" => \"{v}\",\n"));
					}
					string
				},
				vec_to_str(preset.terms),
				{
					match preset.match_score {
						None => String::from("None"),
						Some(score) => format!("Some(\"{score}\")"),
					}
				},
			));
		}
	}

	let dest_path = PathBuf::from(&out_dir).join("preset_data.rs");
	std::fs::write(dest_path, generated)?;

	// preset_defaults.json
	let mut generated = concat!("use id_tagging_schema_types::*;\n").to_string();

	let preset_defaults_file = PathBuf::from(path("preset_defaults.json"));
	let json_str = std::fs::read_to_string(preset_defaults_file)?;
	let preset = serde_json::from_str::<RawPresetDefaults>(&json_str)?;

	generated.push_str(&format!(
		concat!(
			"\npub const PRESET_DEFAULTS: PresetDefaults = PresetDefaults {{\n",
			"\tarea: &[{}],\n",
			"\tline: &[{}],\n",
			"\tpoint: &[{}],\n",
			"\tvertex: &[{}],\n",
			"\trelation: &[{}],\n",
			"}};\n"
		),
		vec_to_str(preset.area),
		vec_to_str(preset.line),
		vec_to_str(preset.point),
		vec_to_str(preset.vertex),
		vec_to_str(preset.relation),
	));

	let dest_path = PathBuf::from(&out_dir).join("preset_defaults.rs");
	std::fs::write(dest_path, generated)?;

	// discarded.json
	let mut generated = String::new();

	let preset_defaults_file = PathBuf::from(path("discarded.json"));
	let json_str = std::fs::read_to_string(preset_defaults_file)?;
	let preset = serde_json::from_str::<HashMap<String, bool>>(&json_str)?;

	generated.push_str("\npub const DISCARDED: &'static [&'static str] = &[");
	generated.push_str(&vec_to_str(preset.into_keys()));
	generated.push_str("];\n");

	let dest_path = PathBuf::from(&out_dir).join("discarded.rs");
	std::fs::write(dest_path, generated)?;

	Ok(())
}

fn vec_to_str(vec: impl IntoIterator<Item = impl std::fmt::Debug>) -> String {
	vec.into_iter()
		.map(|x| format!("{x:?}, "))
		.collect::<String>()
}
