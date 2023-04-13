use std::hash::Hash;

mod freeunit;
mod prefix;
mod unit;

pub use prefix::Prefix;
pub use unit::Unit;
pub use freeunit::FreeUnit;


#[derive(Hash)]
#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(Eq, PartialEq)]
pub enum UnitBase {
	// Base Units
	Second,
	Meter,
	Gram, // Technically kilogram, but that messes with our prefix system.
	Ampere,
	Kelvin,
	Mole,
	Candela,

	// Length units
	Angstrom,
	Thou,
	Point, // pt, typesetting unit
	Inch,
	Foot,
	Mile,
	Furlong,

	// Time units
	Minute,
	Hour,
	Day,
	Week,
	Month,
	Fortnight,
}


// SI prefix list:
// ("Q","R","Y","Z","E","P","T","G","M","k","h","da","d","c","m","u","n","p","f","a","z","y","r","q")

// X macro, used in Unit.from_string()
//
// Format is as follows:
// (Unit, string from, (prefixes_to_generate))
// Prefixes must be valid prefixes as defined in
// Prefix::str_to_prefix.
//
// Prefix array can be ommited to prevent prefix generation.
pub (self) use prefix::str_to_prefix;
macro_rules! fromstring_db {
	($X:ident) => {
		$X!(
			// Base units
			(UnitBase::Meter, "meter"),
			(UnitBase::Meter, "meters"),
			(UnitBase::Ampere, "ampere"),
			(UnitBase::Ampere, "amperes"),
			(UnitBase::Gram, "gram"),
			(UnitBase::Gram, "grams"),
			(UnitBase::Kelvin, "kelvin"),
			(UnitBase::Mole, "mole"),
			(UnitBase::Candela, "candela"),

			// Length
			(UnitBase::Angstrom, "angstrom"),
			(UnitBase::Angstrom, "Å"),
			(UnitBase::Thou, "thou"),
			(UnitBase::Point, "pt"),
			(UnitBase::Point, "point"),
			(UnitBase::Inch, "in"),
			(UnitBase::Inch, "inch"),
			(UnitBase::Foot, "ft"),
			(UnitBase::Foot, "foot"),
			(UnitBase::Foot, "feet"),
			(UnitBase::Mile, "mi"),
			(UnitBase::Mile, "mile"),
			(UnitBase::Mile, "miles"),

			// Time
			(UnitBase::Second, "sec"),
			(UnitBase::Second, "second"),
			(UnitBase::Second, "seconds"),
			(UnitBase::Minute, "min"),
			(UnitBase::Minute, "minute"),
			(UnitBase::Minute, "minutes"),
			(UnitBase::Hour, "h"),
			(UnitBase::Hour, "hour"),
			(UnitBase::Hour, "hours"),
			(UnitBase::Day, "d"),
			(UnitBase::Day, "day"),
			(UnitBase::Day, "days"),
			(UnitBase::Week, "w"),
			(UnitBase::Week, "week"),
			(UnitBase::Week, "weeks"),
			(UnitBase::Month, "month"),
			(UnitBase::Month, "months"),
			(UnitBase::Fortnight, "fortnight"),
			(UnitBase::Fortnight, "fortnights"),

			(UnitBase::Meter, "m",
				("Q","R","Y","Z","E","P","T","G","M","k","h","da","d","c","m","u","n","p","f","a","z","y","r","q")
			),

			(UnitBase::Second, "s",
				("Q","R","Y","Z","E","P","T","G","M","k","h","da","d","c","m","u","n","p","f","a","z","y","r","q")
			),

			(UnitBase::Gram, "g",
				("Q","R","Y","Z","E","P","T","G","M","k","h","da","d","c","m","u","n","p","f","a","z","y","r","q")
			),

			(UnitBase::Ampere, "A",
				("Q","R","Y","Z","E","P","T","G","M","k","h","da","d","c","m","u","n","p","f","a","z","y","r","q")
			),

			(UnitBase::Kelvin, "K",
				("Q","R","Y","Z","E","P","T","G","M","k","h","da","d","c","m","u","n","p","f","a","z","y","r","q")
			),

			(UnitBase::Mole, "mol",
				("Q","R","Y","Z","E","P","T","G","M","k","h","da","d","c","m","u","n","p","f","a","z","y","r","q")
			),

			(UnitBase::Candela, "cd",
				("Q","R","Y","Z","E","P","T","G","M","k","h","da","d","c","m","u","n","p","f","a","z","y","r","q")
			)
		)
	}
}
pub (self) use fromstring_db;


// X macro, used in the following functions:
// - FreeUnit.to_base_factor()
// - FreeUnit.to_string()
//
// Read below comments for explanation.
macro_rules! unit_db {
	($a:expr, $X:ident) => {
		match $a {

			UnitBase::Second => $X!(

				UnitBase::Second,  // Repeat the name of this base unit
				"s",               // String to display for this unit

				// "base", "float", or "rational."
				// if base, this is a base unit and has no conversion factor.
				// if float or rational, this is not a base unit. See below.
				base
			),
			UnitBase::Meter => $X!(
				UnitBase::Meter, "m",
				base
			),
			UnitBase::Gram => $X!(
				UnitBase::Gram, "g",
				base
			),
			UnitBase::Ampere => $X!(
				UnitBase::Ampere, "A",
				base
			),
			UnitBase::Kelvin => $X!(
				UnitBase::Kelvin, "K",
				base
			),
			UnitBase::Mole => $X!(
				UnitBase::Mole, "mol",
				base
			),
			UnitBase::Candela => $X!(
				UnitBase::Candela, "cd",
				base
			),


			UnitBase::Minute => $X!(
				UnitBase::Minute, "min",

				// "rational" and "float" determine what kind of Quantity
				// this unit's conversion factor will be. Use "rational"
				// if it is exact, and "float" if it is an approximation.
				rational,


				// The next two lines are interpreted as follows:
				// One Minute = 60 Seconds.

				// The value
				"60",
				// The unit. Can be repeated for compound units.
				// MUST BE BASE UNITS.
				(UnitBase::Second, 1f64)
			),

			UnitBase::Hour => $X!(
				UnitBase::Hour, "hour",
				rational, "3600",
				(UnitBase::Second, 1f64)
			),

			UnitBase::Day => $X!(
				UnitBase::Day, "day",
				rational, "86400",
				(UnitBase::Second, 1f64)
			),

			UnitBase::Week => $X!(
				UnitBase::Week, "week",
				rational, "604800",
				(UnitBase::Second, 1f64)
			),

			UnitBase::Month => $X!(
				UnitBase::Month, "month",
				rational, "2629746",
				(UnitBase::Second, 1f64)
			),

			UnitBase::Fortnight => $X!(
				UnitBase::Fortnight, "fortnight",
				rational, "1209600",
				(UnitBase::Second, 1f64)
			),

			UnitBase::Angstrom => $X!(
				UnitBase::Angstrom, "Å",
				rational, "1e-10",
				(UnitBase::Meter, 1f64)
			),

			UnitBase::Thou => $X!(
				UnitBase::Thou, "thou",
				float, "0.0000254",
				(UnitBase::Meter, 1f64)
			),

			UnitBase::Point => $X!(
				UnitBase::Point, "pt",
				float, "0.000352778",
				(UnitBase::Meter, 1f64)
			),

			UnitBase::Inch => $X!(
				UnitBase::Inch, "in",
				float, "0.0254",
				(UnitBase::Meter, 1f64)
			),

			UnitBase::Foot => $X!(
				UnitBase::Foot, "ft",
				float, "0.3048",
				(UnitBase::Meter, 1f64)
			),

			UnitBase::Mile => $X!(
				UnitBase::Mile, "mi",
				float, "1609",
				(UnitBase::Meter, 1f64)
			),

			UnitBase::Furlong => $X!(
				UnitBase::Furlong, "furlong",
				float, "201.168",
				(UnitBase::Meter, 1f64)
			),
		}


	}
}
pub (self) use unit_db;