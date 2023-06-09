use crate::quantity::Quantity;


#[derive(Hash)]
#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(Eq, PartialEq)]
pub enum Prefix {
	None,

	Quetta,
	Ronna,
	Yotta,
	Zetta,
	Exa,
	Peta,
	Tera,
	Giga,
	Mega,
	Kilo,
	Hecto,
	Deka,

	Deci,
	Centi,
	Milli,
	Micro,
	Nano,
	Pico,
	Femto,
	Atto,
	Zepto,
	Yocto,
	Ronto,
	Quecto,


	BinExa,
	BinPeta,
	BinTera,
	BinGiga,
	BinMega,
	BinKilo
}


impl Prefix {
	pub fn to_ratio(&self) -> Quantity {
		let q = Quantity::new_rational_from_string(match self {
			Prefix::Quetta => "1e30",
			Prefix::Ronna => "1e27",
			Prefix::Yotta => "1e24",
			Prefix::Zetta => "1e21",
			Prefix::Exa => "1e18",
			Prefix::Peta => "1e15",
			Prefix::Tera => "1e12",
			Prefix::Giga => "1e9",
			Prefix::Mega => "1e6",
			Prefix::Kilo => "1e3",
			Prefix::Hecto => "1e2",
			Prefix::Deka => "1e1",

			Prefix::Deci => "1e-1",
			Prefix::Centi => "1e-2",
			Prefix::Milli => "1e-3",
			Prefix::Micro => "1e-6",
			Prefix::Nano => "1e-9",
			Prefix::Pico => "1e-12",
			Prefix::Femto => "1e-15",
			Prefix::Atto => "1e-18",
			Prefix::Zepto => "1e-21",
			Prefix::Yocto => "1e-24",
			Prefix::Ronto => "1e-27",
			Prefix::Quecto => "1e-30",


			Prefix::BinExa => "1152921504606846976", // 2^60
			Prefix::BinPeta => "1125899906842624", // 2^50
			Prefix::BinTera => "1099511627776", // 2^40
			Prefix::BinGiga => "1073741824", // 2^30
			Prefix::BinMega => "1048576", // 2^20
			Prefix::BinKilo => "1024", // 2^10

			Prefix::None => { "1" }
		}).unwrap();

		return q;

	}
}

macro_rules! str_to_prefix {
	("") => {Prefix::None};
	("Q") => {Prefix::Quetta};
	("R") => {Prefix::Ronna};
	("Y") => {Prefix::Yotta};
	("Z") => {Prefix::Zetta};
	("E") => {Prefix::Exa};
	("P") => {Prefix::Peta};
	("T") => {Prefix::Tera};
	("G") => {Prefix::Giga};
	("M") => {Prefix::Mega};
	("k") => {Prefix::Kilo};
	("h") => {Prefix::Hecto};
	("da") => {Prefix::Deka};
	("d") => {Prefix::Deci};
	("c") => {Prefix::Centi};
	("m") => {Prefix::Milli};
	("u") => {Prefix::Micro};
	("n") => {Prefix::Nano};
	("p") => {Prefix::Pico};
	("f") => {Prefix::Femto};
	("a") => {Prefix::Atto};
	("z") => {Prefix::Zepto};
	("y") => {Prefix::Yocto};
	("r") => {Prefix::Ronto};
	("q") => {Prefix::Quecto};
	("Ei") => {Prefix::BinExa};
	("Pi") => {Prefix::BinPeta};
	("Ti") => {Prefix::BinTera};
	("Gi") => {Prefix::BinGiga};
	("Mi") => {Prefix::BinMega};
	("Ki") => {Prefix::BinKilo};
}
pub (super) use str_to_prefix;


impl ToString for Prefix {
	fn to_string(&self) -> String {
		String::from(match self {
			Prefix::Quetta => "Q",
			Prefix::Ronna => "R",
			Prefix::Yotta => "Y",
			Prefix::Zetta => "Z",
			Prefix::Exa => "E",
			Prefix::Peta => "P",
			Prefix::Tera => "T",
			Prefix::Giga => "G",
			Prefix::Mega => "M",
			Prefix::Kilo => "k",
			Prefix::Hecto => "h",
			Prefix::Deka => "da",

			Prefix::Deci => "d",
			Prefix::Centi => "c",
			Prefix::Milli => "m",
			Prefix::Micro => "u",
			Prefix::Nano => "n",
			Prefix::Pico => "p",
			Prefix::Femto => "f",
			Prefix::Atto => "a",
			Prefix::Zepto => "z",
			Prefix::Yocto => "y",
			Prefix::Ronto => "r",
			Prefix::Quecto => "q",


			Prefix::BinExa => "Ei",
			Prefix::BinPeta => "Pi",
			Prefix::BinTera => "Ti",
			Prefix::BinGiga => "Gi",
			Prefix::BinMega => "Mi",
			Prefix::BinKilo => "Ki",

			Prefix::None => ""
		})
	}
}