use std::io::Write;
use std::fs::OpenOptions;
use std::path::Path;
use toml::Table;



pub fn write(target: &Path) {
	let constants = include_str!("constants.toml").parse::<Table>().unwrap();
	let toml::Value::Array(constants) = &constants["constant"] else {panic!()};

	let mut file = OpenOptions::new()
		.write(true)
		.create(true)
		.truncate(true)
		.open(target)
		.unwrap();


	writeln!(file,
		concat!(
			"#[derive(Debug)]\n",
			"#[derive(Copy, Clone)]\n",
			"pub enum Constant {{"
		)
	).unwrap();

	for c in constants {
		writeln!(file,
			"\t{},",
			c["enum_name"].as_str().unwrap()
		).unwrap();
	}

	writeln!(file, "}}\n").unwrap();

	// ToString
	writeln!(file,
		concat!(
			"impl ToString for Constant {{\n",
			"\tfn to_string(&self) -> String {{\n",
			"\t\tString::from(match self {{"
		)
	).unwrap();

	for c in constants {
		if c["strings"].is_array() {
			writeln!(file,
				"\t\t\tConstant::{e} => \"{s}\",",
				e = c["enum_name"].as_str().unwrap(),
				s = c["strings"].as_array().unwrap()[0].as_str().unwrap()
			).unwrap();
		} else {
			writeln!(file,
				"\t\t\tConstant::{e} => \"{s}\",",
				e = c["enum_name"].as_str().unwrap(),
				s = c["strings"].as_str().unwrap()
			).unwrap();
		}
	}

	writeln!(file, "\t\t}})\n\t}}\n}}\n").unwrap();


	writeln!(file,
		concat!(
			"impl Constant {{\n",
			"\tpub fn from_string(s: &str) -> Option<Constant> {{\n",
			"\t\tmatch s {{"
		)
	).unwrap();

	for c in constants {
		if c["strings"].is_array() {
			for s in c["strings"].as_array().unwrap() {
				writeln!(file,
					"\t\t\t\"{s}\" => Some(Constant::{e}),",
					e = c["enum_name"].as_str().unwrap(),
					s = s.as_str().unwrap()
				).unwrap();
			}
		} else {
			writeln!(file,
				"\t\t\t\"{s}\" => Some(Constant::{e}),",
				e = c["enum_name"].as_str().unwrap(),
				s = c["strings"].as_str().unwrap()
			).unwrap();
		}
	}

	writeln!(file, "\t\t\t_ => None\n\t\t}}\n\t}}\n").unwrap();





	writeln!(file,
		concat!(
			"\tpub fn all_consts() -> &'static [Constant] {{\n",
			"\t\treturn &["
		)
	).unwrap();

	for c in constants {
		writeln!(file,
			"\t\t\tConstant::{e},",
			e = c["enum_name"].as_str().unwrap(),
		).unwrap();
	}

	writeln!(file, "\t\t]\n\t}}\n").unwrap();






	writeln!(file,
		concat!(
			"\tpub fn source_strings(&self) -> &'static [&'static str] {{\n",
			"\t\tmatch self {{"
		)
	).unwrap();

	for c in constants {
		write!(file,
			"\t\t\tConstant::{e} => &[",
			e = c["enum_name"].as_str().unwrap(),
		).unwrap();

		if c["strings"].is_array() {
			for s in c["strings"].as_array().unwrap() {
				write!(file,
					"\"{s}\",",
					s = s.as_str().unwrap()
				).unwrap();
			}
			write!(file,
				"],",
			).unwrap();
		} else {
			write!(file,
				"\"{s}\"],",
				s = c["strings"].as_str().unwrap()
			).unwrap();
		}

		write!(file,
			"\n",
		).unwrap();
	}

	writeln!(file, "\t\t}}\n\t}}\n").unwrap();






	writeln!(file,
		concat!(
			"\tpub fn pretty_name(&self) -> Option<&'static str> {{\n",
			"\t\tmatch self {{"
		)
	).unwrap();

	for c in constants {
		if c.as_table().unwrap().contains_key("pretty_name") {
			writeln!(file,
				"\t\t\tConstant::{e} => Some(&\"{s}\"),",
				e = c["enum_name"].as_str().unwrap(),
				s = c["pretty_name"].as_str().unwrap()
			).unwrap();
		}
	}

	writeln!(file, "\t\t\t_ => None\n\t\t}}\n\t}}\n").unwrap();






	writeln!(file,
		concat!(
			"\tpub fn value(&self) -> Expression {{\n",
			"\t\tmatch self {{"
		)
	).unwrap();

	for c in constants {
		writeln!(file,
			"\t\t\tConstant::{e} => parse_no_context(&String::from(\"{s}\")).unwrap(),",
			e = c["enum_name"].as_str().unwrap(),
			s = c["value"].as_str().unwrap()
		).unwrap();
	}

	writeln!(file, "\t\t}}\n\t}}\n}}").unwrap();
}