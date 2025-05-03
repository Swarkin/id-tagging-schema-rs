use id_tagging_schema_types::*;
use std::fs::read_dir;
use std::path::PathBuf;

fn path(path: &str) -> String {
	format!("id-tagging-schema/data/{path}")
}

fn main() -> std::io::Result<()> {
	// Categories
	let category_dir = PathBuf::from(path("preset_categories"));
	let out_dir = std::env::var("OUT_DIR").unwrap();
	
	let mut generated = concat!("use std::sync::LazyLock;\n", "use id_tagging_schema_types::*;\n").to_string();

	for entry in read_dir(category_dir)?.map(|x| x.unwrap()) {
		let path = entry.path();

		if entry.file_type()?.is_file() && path.extension().map(|x| x == "json").unwrap_or(false) {
			let json_str = std::fs::read_to_string(&path)?;
			let category = serde_json::from_str::<Category>(&json_str)?;

			let ident = path
				.file_stem()
				.unwrap()
				.to_str()
				.unwrap()
				.replace(['-', '.'], "_")
				.to_uppercase();

			generated.push_str(&format!(
				concat!(
					"\npub static {}: LazyLock<Category> = LazyLock::new(|| Category {{\n",
					"\ticon: {:?}.into(),\n",
					"\tname: {:?}.into(),\n",
					"\tmembers: vec![{}],\n",
					"}});\n"
				),
				ident,
				category.icon,
				category.name,
				category
					.members
					.into_iter()
					.map(|x| format!("{x:?}.into(), "))
					.collect::<String>()
			));
		}
	}

	let dest_path = PathBuf::from(&out_dir).join("category_data.rs");
	std::fs::write(dest_path, generated)?;

	// Presets
	let mut generated = concat!(
		"use std::sync::LazyLock;\n",
		"use std::collections::HashMap;\n",
		"use id_tagging_schema_types::{*, Geometry::*};\n",
	)
	.to_string();

	let preset_dir = PathBuf::from(path("presets"));
	for entry in read_dir(preset_dir)?.map(|x| x.unwrap()) {
		let path = entry.path();
		if entry.file_type()?.is_file() && path.extension().map(|x| x == "json").unwrap() && !path.starts_with("_") {
			println!("{path:?}");
			let json_str = std::fs::read_to_string(&path)?;
			let preset = serde_json::from_str::<Preset>(&json_str)?;

			let ident = path
				.file_stem()
				.unwrap()
				.to_str()
				.unwrap()
				.replace(['-', '.'], "_")
				.to_uppercase();

			generated.push_str(&format!(
				concat!(
					"\npub static {}: LazyLock<Preset> = LazyLock::new(|| Preset {{\n",
					"\ticon: {},\n",
					"\tname: {:?}.into(),\n",
					"\tfields: vec![{}],\n",
					"\tmore_fields: vec![{}],\n",
					"\tgeometry: vec!{:?},\n",
					"\ttags: {{\n\t\tlet mut x = HashMap::new();\n\t\t{}\n\t\tx\n\t}},\n",
					"\tterms: vec![{}],\n",
					"\tmatch_score: {},\n",
					"}});\n"
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
							string.push_str("\".into())");
						}
					}
					string
				},
				preset.name,
				preset
					.fields
					.into_iter()
					.map(|x| format!("{x:?}.into(), "))
					.collect::<String>(),
				preset
					.more_fields
					.into_iter()
					.map(|x| format!("{x:?}.into(), "))
					.collect::<String>(),
				preset.geometry,
				{
					let mut string = String::new();
					for (k, v) in preset.tags {
						string.push_str(&format!("x.insert({k:?}.into(), {v:?}.into()); "));
					}
					string
				},
				preset
					.terms
					.into_iter()
					.map(|x| format!("{x:?}.into(), "))
					.collect::<String>(),
				{
					match preset.match_score {
						None => String::from("None"),
						Some(score) => format!("Some(\"{score}\".into())"),
					}
				},
			));
		}
	}

	let dest_path = PathBuf::from(&out_dir).join("preset_data.rs");
	std::fs::write(dest_path, generated)?;

	Ok(())
}
