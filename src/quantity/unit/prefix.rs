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
	Quecto
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

			Prefix::None => { "1" }
		}).unwrap();

		return q;

	}
}


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

			Prefix::None => ""
		})
	}
}