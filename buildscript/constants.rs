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
			"\tpub fn value(&self) -> Token {{\n",
			"\t\tmatch self {{"
		)
	).unwrap();

	for c in constants {
		writeln!(file,
			"\t\t\tConstant::{e} => parse(&String::from(\"{s}\")).unwrap(),",
			e = c["enum_name"].as_str().unwrap(),
			s = c["value"].as_str().unwrap()
		).unwrap();
	}

	writeln!(file, "\t\t}}\n\t}}\n}}").unwrap();
}