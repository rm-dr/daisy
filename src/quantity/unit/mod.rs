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
	Yard,
	Furlong,
	Mile,
	AstronomicalUnit,
	Lightyear,
	Parsec,

	// Area
	Barn,
	Hectare,
	Acre,



	// Time units
	Minute,
	Hour,
	Day,
	Week,
	Month,
	Fortnight,
	GregorianYear,
	JulianYear,

	// Volume
	Liter,
	USGallon,
	Quart,
	ImperialGallon,
	Hogshead,
	Cup,
	Floz,
	Pint,
	Tablespoon,
	Teaspoon,

	// Pressure
	Pascal,
	Atmosphere,
	Bar,
	Barye,
	Psi,
	MillimeterMercury,
	Torr,
	MeterSeaWater,
	FootSeaWater
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
			(UnitBase::Gram, "gramme"),
			(UnitBase::Gram, "grammes"),
			(UnitBase::Kelvin, "kelvin"),
			(UnitBase::Mole, "mole"),
			(UnitBase::Candela, "candela"),

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
			),

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
			(UnitBase::Yard, "yard"),
			(UnitBase::Yard, "yd"),
			(UnitBase::Yard, "yards"),
			(UnitBase::Mile, "mi"),
			(UnitBase::Mile, "mile"),
			(UnitBase::Mile, "miles"),
			(UnitBase::AstronomicalUnit, "au"),
			(UnitBase::AstronomicalUnit, "AU"),
			(UnitBase::AstronomicalUnit, "astronomicalUnit"),
			(UnitBase::AstronomicalUnit, "astronomicalUnits"),
			(UnitBase::Lightyear, "ly"),
			(UnitBase::Lightyear, "lightyear"),
			(UnitBase::Lightyear, "lightyears"),
			(UnitBase::Parsec, "pc"),
			(UnitBase::Parsec, "parsec"),
			(UnitBase::Parsec, "parsecs"),

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
			(UnitBase::GregorianYear, "year"),
			(UnitBase::GregorianYear, "years"),
			(UnitBase::JulianYear, "julianYear"),
			(UnitBase::JulianYear, "julianYears"),

			// Misc
			(UnitBase::Barn, "b"),
			(UnitBase::Barn, "barn"),
			(UnitBase::Hectare, "ha"),
			(UnitBase::Hectare, "hectare"),
			(UnitBase::Hectare, "hectares"),
			(UnitBase::Acre, "acre"),
			(UnitBase::Acre, "acres"),


			// Volume
			(UnitBase::Liter, "liter"),
			(UnitBase::Liter, "liters"),
			(UnitBase::Liter, "litre"),
			(UnitBase::Liter, "litres"),
			(UnitBase::USGallon, "usgal"),
			(UnitBase::USGallon, "gal"),
			(UnitBase::USGallon, "gallon"),
			(UnitBase::USGallon, "gallons"),
			(UnitBase::Quart, "quart"),
			(UnitBase::Quart, "quarts"),
			(UnitBase::Quart, "qt"),
			(UnitBase::ImperialGallon, "impgal"),
			(UnitBase::ImperialGallon, "imperialGallon"),
			(UnitBase::ImperialGallon, "imperialGallons"),
			(UnitBase::Cup, "cup"),
			(UnitBase::Floz, "floz"),
			(UnitBase::Pint, "pint"),
			(UnitBase::Pint, "pints"),
			(UnitBase::Tablespoon, "tablespoon"),
			(UnitBase::Tablespoon, "tablespoons"),
			(UnitBase::Tablespoon, "tbsp"),
			(UnitBase::Tablespoon, "Tbsp"),
			(UnitBase::Teaspoon, "teaspoon"),
			(UnitBase::Teaspoon, "teaspoons"),
			(UnitBase::Teaspoon, "tsp"),
			(UnitBase::Teaspoon, "Tsp"),



			(UnitBase::Hogshead, "hogshead"),
			(UnitBase::Hogshead, "hogsheads"),


			(UnitBase::Liter, "l",
				("Q","R","Y","Z","E","P","T","G","M","k","h","da","d","c","m","u","n","p","f","a","z","y","r","q")
			),

			(UnitBase::Liter, "L",
				("Q","R","Y","Z","E","P","T","G","M","k","h","da","d","c","m","u","n","p","f","a","z","y","r","q")
			),



			// Pressure
			(UnitBase::Atmosphere, "atm"),
			(UnitBase::Atmosphere, "atmosphere"),
			(UnitBase::Atmosphere, "atmospheres"),
			(UnitBase::Pascal, "pascal"),
			(UnitBase::Barye, "Ba"),
			(UnitBase::Psi, "psi"),
			(UnitBase::MillimeterMercury, "mmhg"),
			(UnitBase::MillimeterMercury, "mmHg"),
			(UnitBase::Torr, "torr"),
			(UnitBase::Torr, "Torr"),
			(UnitBase::MeterSeaWater, "msw"),
			(UnitBase::FootSeaWater, "fsw"),
			(UnitBase::MeterSeaWater, "MSW"),
			(UnitBase::FootSeaWater, "FSW"),

			(UnitBase::Pascal, "Pa",
				("Q","R","Y","Z","E","P","T","G","M","k","h","da","d","c","m","u","n","p","f","a","z","y","r","q")
			),

			(UnitBase::Bar, "bar",
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


			// Base

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




			// Time

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
				UnitBase::Hour, "h",
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

			UnitBase::GregorianYear => $X!(
				UnitBase::GregorianYear, "year",
				rational, "31557000",
				(UnitBase::Second, 1f64)
			),

			UnitBase::JulianYear => $X!(
				UnitBase::JulianYear, "julianYear",
				rational, "31557600",
				(UnitBase::Second, 1f64)
			),



			// Length

			UnitBase::Angstrom => $X!(
				UnitBase::Angstrom, "Å",
				rational, "1e-10",
				(UnitBase::Meter, 1f64)
			),

			UnitBase::Thou => $X!(
				UnitBase::Thou, "thou",
				rational, "0.0000254",
				(UnitBase::Meter, 1f64)
			),

			UnitBase::Point => $X!(
				UnitBase::Point, "pt",
				rational, "0.0003514598",
				(UnitBase::Meter, 1f64)
			),

			UnitBase::Inch => $X!(
				UnitBase::Inch, "in",
				rational, "0.0254",
				(UnitBase::Meter, 1f64)
			),

			UnitBase::Foot => $X!(
				UnitBase::Foot, "ft",
				rational, "0.3048",
				(UnitBase::Meter, 1f64)
			),

			UnitBase::Yard => $X!(
				UnitBase::Yard, "yd",
				rational, "0.9144",
				(UnitBase::Meter, 1f64)
			),

			UnitBase::Furlong => $X!(
				UnitBase::Furlong, "furlong",
				rational, "201.17",
				(UnitBase::Meter, 1f64)
			),

			UnitBase::Mile => $X!(
				UnitBase::Mile, "mi",
				rational, "1609.344",
				(UnitBase::Meter, 1f64)
			),

			UnitBase::AstronomicalUnit => $X!(
				UnitBase::AstronomicalUnit, "AU",
				rational, "149597870700",
				(UnitBase::Meter, 1f64)
			),

			UnitBase::Lightyear => $X!(
				UnitBase::Lightyear, "ly",
				rational, "9460730472580800",
				(UnitBase::Meter, 1f64)
			),

			UnitBase::Parsec => $X!(
				UnitBase::Parsec, "pc",
				float, "3.085677581e16",
				(UnitBase::Meter, 1f64)
			),

			UnitBase::Barn => $X!(
				UnitBase::Barn, "b",
				rational, "1e-28",
				(UnitBase::Meter, 2f64)
			),

			UnitBase::Hectare => $X!(
				UnitBase::Hectare, "ha",
				rational, "10000",
				(UnitBase::Meter, 2f64)
			),

			UnitBase::Acre => $X!( // 66 x 660 feet
				UnitBase::Acre, "acre",
				rational, "4046.8564224",
				(UnitBase::Meter, 2f64)
			),



			// Volume
			UnitBase::Liter => $X!(
				UnitBase::Liter, "l",
				rational, "0.001",
				(UnitBase::Meter, 3f64)
			),

			UnitBase::Hogshead => $X!(
				UnitBase::Hogshead, "hogshead",
				rational, "0.2385", // 63 gallons
				(UnitBase::Meter, 3f64)
			),

			UnitBase::USGallon => $X!(
				UnitBase::USGallon, "gal",
				rational, "0.003785411784",
				(UnitBase::Meter, 3f64)
			),

			UnitBase::Quart => $X!(
				UnitBase::Quart, "qt",
				rational, "0.000946352946",
				(UnitBase::Meter, 3f64)
			),

			UnitBase::ImperialGallon => $X!(
				UnitBase::ImperialGallon, "impgal",
				rational, "0.00454609",
				(UnitBase::Meter, 3f64)
			),

			UnitBase::Cup => $X!(
				UnitBase::Cup, "cup",
				rational, "0.0002365882365",
				(UnitBase::Meter, 3f64)
			),

			UnitBase::Floz => $X!(
				UnitBase::Floz, "floz",
				rational, "0.0000295735295625",
				(UnitBase::Meter, 3f64)
			),

			UnitBase::Pint => $X!(
				UnitBase::Pint, "pint",
				rational, "0.00056826125",
				(UnitBase::Meter, 3f64)
			),

			UnitBase::Tablespoon => $X!(
				UnitBase::Tablespoon, "tbsp",
				rational, "0.00001478676478125",
				(UnitBase::Meter, 3f64)
			),

			UnitBase::Teaspoon => $X!(
				UnitBase::Teaspoon, "tsp",
				rational, "0.000005",
				(UnitBase::Meter, 3f64)
			),



			// Pressure

			UnitBase::Pascal => $X!(
				UnitBase::Pascal, "Pa",
				rational, "1000",
				(UnitBase::Gram, 1f64),
				(UnitBase::Meter, -1f64),
				(UnitBase::Second, -2f64)
			),

			UnitBase::Bar => $X!(
				UnitBase::Bar, "bar",
				rational, "100000000",
				(UnitBase::Gram, 1f64),
				(UnitBase::Meter, -1f64),
				(UnitBase::Second, -2f64)
			),

			UnitBase::Barye => $X!(
				UnitBase::Barye, "Ba",
				rational, "100",
				(UnitBase::Gram, 1f64),
				(UnitBase::Meter, -1f64),
				(UnitBase::Second, -2f64)
			),

			UnitBase::Atmosphere => $X!(
				UnitBase::Atmosphere, "atm",
				rational, "101325000",
				(UnitBase::Gram, 1f64),
				(UnitBase::Meter, -1f64),
				(UnitBase::Second, -2f64)
			),

			UnitBase::Psi => $X!(
				UnitBase::Psi, "psi",
				rational, "6894757.2931783",
				(UnitBase::Gram, 1f64),
				(UnitBase::Meter, -1f64),
				(UnitBase::Second, -2f64)
			),

			UnitBase::MillimeterMercury => $X!(
				UnitBase::MillimeterMercury, "mmHg",
				rational, "133322.387415",
				(UnitBase::Gram, 1f64),
				(UnitBase::Meter, -1f64),
				(UnitBase::Second, -2f64)
			),

			UnitBase::Torr => $X!(
				UnitBase::Torr, "torr",
				rational_frac, (101325000, 760),
				(UnitBase::Gram, 1f64),
				(UnitBase::Meter, -1f64),
				(UnitBase::Second, -2f64)
			),

			UnitBase::MeterSeaWater => $X!(
				UnitBase::MeterSeaWater, "MSW",
				rational, "10000000",
				(UnitBase::Gram, 1f64),
				(UnitBase::Meter, -1f64),
				(UnitBase::Second, -2f64)
			),

			UnitBase::FootSeaWater => $X!(
				UnitBase::FootSeaWater, "FSW",
				rational, "3064330",
				(UnitBase::Gram, 1f64),
				(UnitBase::Meter, -1f64),
				(UnitBase::Second, -2f64)
			),

		}


	}
}
pub (self) use unit_db;