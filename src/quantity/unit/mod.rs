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
	Inch,
	Foot,
	Mile,

	// Time units
	Minute,
	Hour,
	Day,
	//Week,
	//Month,
}


// SI prefix list:
// ("Q","R","Y","Z","E","P","T","G","M","k","h","da","d","c","m","u","n","p","f","a","z","y","r","q")

// X macro, used in Unit.from_string()
//
// Format is as follows:
// (Unit, string from, (prefixes_to_generate))
// Prefixes must be valid prefixes as defined in
// Prefix::str_to_prefix.
pub (self) use prefix::str_to_prefix;
macro_rules! fromstring_db {
	($X:ident) => {
		$X!(
			// No prefix
			(UnitBase::Meter, "meter"),
			(UnitBase::Foot, "ft"),
			(UnitBase::Mile, "mile"),
			(UnitBase::Hour, "hour"),
			(UnitBase::Minute, "min"),
			(UnitBase::Day, "day"),
			(UnitBase::Second, "sec"),


			(UnitBase::Meter, "m",
				("Q","R","Y","Z","E","P","T","G","M","k","h","da","d","c","m","u","n","p","f","a","z","y","r","q")
			),

			(UnitBase::Second, "s",
				("Q","R","Y","Z","E","P","T","G","M","k","h","da","d","c","m","u","n","p","f","a","z","y","r","q")
			),

			(UnitBase::Gram, "g",
				("Q","R","Y","Z","E","P","T","G","M","k","h","da","d","c","m","u","n","p","f","a","z","y","r","q")
			),

			(UnitBase::Ampere, "a",
				("Q","R","Y","Z","E","P","T","G","M","k","h","da","d","c","m","u","n","p","f","a","z","y","r","q")
			),

			(UnitBase::Kelvin, "k",
				("Q","R","Y","Z","E","P","T","G","M","k","h","da","d","c","m","u","n","p","f","a","z","y","r","q")
			),

			(UnitBase::Mole, "mol",
				("Q","R","Y","Z","E","P","T","G","M","k","h","da","d","c","m","u","n","p","f","a","z","y","r","q")
			),

			(UnitBase::Candela, "c",
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
				UnitBase::Ampere, "a",
				base
			),
			UnitBase::Kelvin => $X!(
				UnitBase::Kelvin, "k",
				base
			),
			UnitBase::Mole => $X!(
				UnitBase::Mole, "mol",
				base
			),
			UnitBase::Candela => $X!(
				UnitBase::Candela, "c",
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

			UnitBase::Foot => $X!(
				UnitBase::Foot, "ft",
				float, "0.3048",
				(UnitBase::Meter, 1f64)
			),

			UnitBase::Inch => $X!(
				UnitBase::Inch, "in",
				float, "0.0254",
				(UnitBase::Meter, 1f64)
			),

			UnitBase::Mile => $X!(
				UnitBase::Mile, "mile",
				float, "1609",
				(UnitBase::Meter, 1f64)
			),
		}


	}
}
pub (self) use unit_db;