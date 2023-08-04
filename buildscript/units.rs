use std::path::Path;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use toml::Table;
use toml::Value;



/// Create WholeUnit enum with
/// basic impls. Should only be run once.
fn write_wholeunit_main(mut file: &File, units: &Vec<Value>) {
	writeln!(file,
		concat!(
			"#[derive(Hash)]\n",
			"#[derive(Debug)]\n",
			"#[derive(Copy, Clone)]\n",
			"#[derive(Eq, PartialEq)]\n",
			"pub enum WholeUnit {{"
		)
	).unwrap();

	for u in units {
		writeln!(file,
			"\t{},",
			u["enum_name"].as_str().unwrap()
		).unwrap();
	}

	writeln!(file, "}}\n").unwrap();

	// ToString
	writeln!(file,
		concat!(
			"impl ToString for WholeUnit {{\n",
			"\tfn to_string(&self) -> String {{\n",
			"\t\tString::from(match self {{"
		)
	).unwrap();

	for u in units {
		writeln!(file,
			"\t\t\tWholeUnit::{e} => \"{s}\",",
			s = u["strings"].as_array().unwrap()[0][0].as_str().unwrap(),
			e = u["enum_name"].as_str().unwrap()
		).unwrap();
	}

	writeln!(file, "\t\t}})\n\t}}\n}}\n").unwrap();


	// Properties
	writeln!(file,
		concat!(
			"impl WholeUnit {{\n",
			"\tfn no_space(&self) -> bool {{\n",
			"\t\tmatch self {{"
		)
	).unwrap();

	for u in units {
		if u.as_table().unwrap().contains_key("no_space") {
			if u.as_table().unwrap()["no_space"].as_bool().unwrap() {
				writeln!(file,
					"\t\t\tWholeUnit::{} => true,",
					u["enum_name"].as_str().unwrap()
				).unwrap();
			}
		}
	}

	writeln!(file, "\t\t\t_ => false\n\t\t}}\n\t}}\n}}").unwrap();
}


/// Create WholeUnit::base_factor().
/// Should only be run once.
fn write_wholeunit_base_factor(mut file: &File, units: &Vec<Value>) {
	writeln!(file,
		concat!(
			"impl WholeUnit {{\n",
			"\tfn base_factor(&self) -> Option<Quantity> {{\n",
			"\t\tmatch self {{"
		)
	).unwrap();

	for u in units {


		if { // Base units should return None
			u.as_table().unwrap().contains_key("base") &&
			u["base"].as_bool().unwrap()
		} {
			writeln!(file,
				"\t\t\tWholeUnit::{} => None,",
				u["enum_name"].as_str().unwrap()
			).unwrap();
			continue
		}


		writeln!(file,
			"\t\t\tWholeUnit::{} => Some(Quantity{{",
			u["enum_name"].as_str().unwrap()
		).unwrap();

		match u["base_value_type"].as_str().unwrap() {
			"exact" => {
				writeln!(file,
					"\t\t\t\tscalar: Scalar::new_rational_from_string(\"{}\").unwrap(),",
					u["base_value"].as_str().unwrap(),
				).unwrap();
			},

			"fract" => {
				writeln!(file,
					"\t\t\t\tscalar: Scalar::new_rational_from_frac({}, {}).unwrap(),",
					u["base_value"].as_array().unwrap()[0].as_integer().unwrap(),
					u["base_value"].as_array().unwrap()[1].as_integer().unwrap(),
				).unwrap();
			},

			"approx" => {
				writeln!(file,
					"\t\t\t\tscalar: Scalar::new_float_from_string(\"{}\").unwrap(),",
					u["base_value"].as_str().unwrap(),
				).unwrap();
			},

			_ => panic!()
		};

		writeln!(file,
			concat!(
				"\t\t\t\tunit: Unit::from_array(&[\n",
				"\t\t\t\t\t(FreeUnit{{whole: WholeUnit::{}, prefix: Prefix::None}}, Scalar::new_rational(-1f64).unwrap()),",
			),
			u["enum_name"].as_str().unwrap()
		).unwrap();

		for b in u["base_units"].as_array().unwrap() {
			writeln!(file,
				"\t\t\t\t\t(FreeUnit{{whole: WholeUnit::{u}, prefix: Prefix::None}}, Scalar::new_rational({p}f64).unwrap()),",
				u = b.as_table().unwrap()["u"].as_str().unwrap(),
				p = b.as_table().unwrap()["p"].as_integer().unwrap(),
			).unwrap();
		}

		writeln!(file,
			concat!(
				"\t\t\t\t])\n",
				"\t\t\t}}),"
			),
		).unwrap();
	}

	writeln!(file, "\t\t}}\n\t}}\n}}").unwrap();
}


/// Create freeunit_from_string().
/// Should only be run once.
fn write_freeunit_from_string(mut file: &File, units: &Vec<Value>) {
	writeln!(file,
		concat!(
			"#[inline(always)]\n",
			"pub fn freeunit_from_string(s: &str) -> Option<FreeUnit> {{\n",
			"\tmatch s {{"
		),
	).unwrap();

	for u in units {

		for s in u["strings"].as_array().unwrap() {

			if s.as_array().unwrap().len() == 1 {
				writeln!(file,
					"\t\t\"{}\" => Some(FreeUnit{{whole: WholeUnit::{}, prefix: Prefix::None}}),",
					s.as_array().unwrap()[0].as_str().unwrap(),
					u["enum_name"].as_str().unwrap()
				).unwrap();
			} else {
				for p in &s.as_array().unwrap()[1..] {
					writeln!(file,
						"\t\t\"{p}{u}\" => Some(FreeUnit{{whole: WholeUnit::{e}, prefix: str_to_prefix!(\"{p}\")}}),",
						u = s.as_array().unwrap()[0].as_str().unwrap(),
						p = p.as_str().unwrap(),
						e = u["enum_name"].as_str().unwrap()
					).unwrap();
				}
			}


		}

		writeln!(file, "").unwrap();
	}

	writeln!(file, "\t\t_ => None\n\t}}\n}}").unwrap();
}

pub fn write(target: &Path) {
	let units = include_str!("units.toml").parse::<Table>().unwrap();
	let toml::Value::Array(units) = &units["unit"] else {panic!()};

	let mut file = OpenOptions::new()
		.write(true)
		.create(true)
		.truncate(true)
		.open(target)
		.unwrap();


	write_wholeunit_main(&file, units);
	writeln!(file, "\n\n").unwrap();

	write_wholeunit_base_factor(&file, units);
	writeln!(file, "\n\n").unwrap();

	write_freeunit_from_string(&file, units);
}
