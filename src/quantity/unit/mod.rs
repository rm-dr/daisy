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
pub enum WholeUnit {
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
			(WholeUnit::Meter, "meter"),
			(WholeUnit::Meter, "meters"),
			(WholeUnit::Ampere, "ampere"),
			(WholeUnit::Ampere, "amperes"),
			(WholeUnit::Gram, "gram"),
			(WholeUnit::Gram, "grams"),
			(WholeUnit::Gram, "gramme"),
			(WholeUnit::Gram, "grammes"),
			(WholeUnit::Kelvin, "kelvin"),
			(WholeUnit::Mole, "mole"),
			(WholeUnit::Candela, "candela"),

			(WholeUnit::Meter, "m",
				("Q","R","Y","Z","E","P","T","G","M","k","h","da","d","c","m","u","n","p","f","a","z","y","r","q")
			),

			(WholeUnit::Second, "s",
				("Q","R","Y","Z","E","P","T","G","M","k","h","da","d","c","m","u","n","p","f","a","z","y","r","q")
			),

			(WholeUnit::Gram, "g",
				("Q","R","Y","Z","E","P","T","G","M","k","h","da","d","c","m","u","n","p","f","a","z","y","r","q")
			),

			(WholeUnit::Ampere, "A",
				("Q","R","Y","Z","E","P","T","G","M","k","h","da","d","c","m","u","n","p","f","a","z","y","r","q")
			),

			(WholeUnit::Kelvin, "K",
				("Q","R","Y","Z","E","P","T","G","M","k","h","da","d","c","m","u","n","p","f","a","z","y","r","q")
			),

			(WholeUnit::Mole, "mol",
				("Q","R","Y","Z","E","P","T","G","M","k","h","da","d","c","m","u","n","p","f","a","z","y","r","q")
			),

			(WholeUnit::Candela, "cd",
				("Q","R","Y","Z","E","P","T","G","M","k","h","da","d","c","m","u","n","p","f","a","z","y","r","q")
			),

			// Length
			(WholeUnit::Angstrom, "angstrom"),
			(WholeUnit::Angstrom, "Å"),
			(WholeUnit::Thou, "thou"),
			(WholeUnit::Point, "pt"),
			(WholeUnit::Point, "point"),
			(WholeUnit::Inch, "in"),
			(WholeUnit::Inch, "inch"),
			(WholeUnit::Foot, "ft"),
			(WholeUnit::Foot, "foot"),
			(WholeUnit::Foot, "feet"),
			(WholeUnit::Yard, "yard"),
			(WholeUnit::Yard, "yd"),
			(WholeUnit::Yard, "yards"),
			(WholeUnit::Mile, "mi"),
			(WholeUnit::Mile, "mile"),
			(WholeUnit::Mile, "miles"),
			(WholeUnit::AstronomicalUnit, "au"),
			(WholeUnit::AstronomicalUnit, "AU"),
			(WholeUnit::AstronomicalUnit, "astronomicalUnit"),
			(WholeUnit::AstronomicalUnit, "astronomicalUnits"),
			(WholeUnit::Lightyear, "ly"),
			(WholeUnit::Lightyear, "lightyear"),
			(WholeUnit::Lightyear, "lightyears"),
			(WholeUnit::Parsec, "pc"),
			(WholeUnit::Parsec, "parsec"),
			(WholeUnit::Parsec, "parsecs"),

			// Time
			(WholeUnit::Second, "sec"),
			(WholeUnit::Second, "second"),
			(WholeUnit::Second, "seconds"),
			(WholeUnit::Minute, "min"),
			(WholeUnit::Minute, "minute"),
			(WholeUnit::Minute, "minutes"),
			(WholeUnit::Hour, "h"),
			(WholeUnit::Hour, "hour"),
			(WholeUnit::Hour, "hours"),
			(WholeUnit::Day, "d"),
			(WholeUnit::Day, "day"),
			(WholeUnit::Day, "days"),
			(WholeUnit::Week, "w"),
			(WholeUnit::Week, "week"),
			(WholeUnit::Week, "weeks"),
			(WholeUnit::Month, "month"),
			(WholeUnit::Month, "months"),
			(WholeUnit::Fortnight, "fortnight"),
			(WholeUnit::Fortnight, "fortnights"),
			(WholeUnit::GregorianYear, "year"),
			(WholeUnit::GregorianYear, "years"),
			(WholeUnit::JulianYear, "julianYear"),
			(WholeUnit::JulianYear, "julianYears"),

			// Misc
			(WholeUnit::Barn, "b"),
			(WholeUnit::Barn, "barn"),
			(WholeUnit::Hectare, "ha"),
			(WholeUnit::Hectare, "hectare"),
			(WholeUnit::Hectare, "hectares"),
			(WholeUnit::Acre, "acre"),
			(WholeUnit::Acre, "acres"),


			// Volume
			(WholeUnit::Liter, "liter"),
			(WholeUnit::Liter, "liters"),
			(WholeUnit::Liter, "litre"),
			(WholeUnit::Liter, "litres"),
			(WholeUnit::USGallon, "usgal"),
			(WholeUnit::USGallon, "gal"),
			(WholeUnit::USGallon, "gallon"),
			(WholeUnit::USGallon, "gallons"),
			(WholeUnit::Quart, "quart"),
			(WholeUnit::Quart, "quarts"),
			(WholeUnit::Quart, "qt"),
			(WholeUnit::ImperialGallon, "impgal"),
			(WholeUnit::ImperialGallon, "imperialGallon"),
			(WholeUnit::ImperialGallon, "imperialGallons"),
			(WholeUnit::Cup, "cup"),
			(WholeUnit::Floz, "floz"),
			(WholeUnit::Pint, "pint"),
			(WholeUnit::Pint, "pints"),
			(WholeUnit::Tablespoon, "tablespoon"),
			(WholeUnit::Tablespoon, "tablespoons"),
			(WholeUnit::Tablespoon, "tbsp"),
			(WholeUnit::Tablespoon, "Tbsp"),
			(WholeUnit::Teaspoon, "teaspoon"),
			(WholeUnit::Teaspoon, "teaspoons"),
			(WholeUnit::Teaspoon, "tsp"),
			(WholeUnit::Teaspoon, "Tsp"),



			(WholeUnit::Hogshead, "hogshead"),
			(WholeUnit::Hogshead, "hogsheads"),


			(WholeUnit::Liter, "l",
				("Q","R","Y","Z","E","P","T","G","M","k","h","da","d","c","m","u","n","p","f","a","z","y","r","q")
			),

			(WholeUnit::Liter, "L",
				("Q","R","Y","Z","E","P","T","G","M","k","h","da","d","c","m","u","n","p","f","a","z","y","r","q")
			),



			// Pressure
			(WholeUnit::Atmosphere, "atm"),
			(WholeUnit::Atmosphere, "atmosphere"),
			(WholeUnit::Atmosphere, "atmospheres"),
			(WholeUnit::Pascal, "pascal"),
			(WholeUnit::Barye, "Ba"),
			(WholeUnit::Psi, "psi"),
			(WholeUnit::MillimeterMercury, "mmhg"),
			(WholeUnit::MillimeterMercury, "mmHg"),
			(WholeUnit::Torr, "torr"),
			(WholeUnit::Torr, "Torr"),
			(WholeUnit::MeterSeaWater, "msw"),
			(WholeUnit::FootSeaWater, "fsw"),
			(WholeUnit::MeterSeaWater, "MSW"),
			(WholeUnit::FootSeaWater, "FSW"),

			(WholeUnit::Pascal, "Pa",
				("Q","R","Y","Z","E","P","T","G","M","k","h","da","d","c","m","u","n","p","f","a","z","y","r","q")
			),

			(WholeUnit::Bar, "bar",
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

			WholeUnit::Second => $X!(

				WholeUnit::Second,  // Repeat the name of this base unit
				"s",               // String to display for this unit

				// "base", "float", or "rational."
				// if base, this is a base unit and has no conversion factor.
				// if float or rational, this is not a base unit. See below.
				base
			),
			WholeUnit::Meter => $X!(
				WholeUnit::Meter, "m",
				base
			),
			WholeUnit::Gram => $X!(
				WholeUnit::Gram, "g",
				base
			),
			WholeUnit::Ampere => $X!(
				WholeUnit::Ampere, "A",
				base
			),
			WholeUnit::Kelvin => $X!(
				WholeUnit::Kelvin, "K",
				base
			),
			WholeUnit::Mole => $X!(
				WholeUnit::Mole, "mol",
				base
			),
			WholeUnit::Candela => $X!(
				WholeUnit::Candela, "cd",
				base
			),




			// Time

			WholeUnit::Minute => $X!(
				WholeUnit::Minute, "min",

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
				(WholeUnit::Second, 1f64)
			),

			WholeUnit::Hour => $X!(
				WholeUnit::Hour, "h",
				rational, "3600",
				(WholeUnit::Second, 1f64)
			),

			WholeUnit::Day => $X!(
				WholeUnit::Day, "day",
				rational, "86400",
				(WholeUnit::Second, 1f64)
			),

			WholeUnit::Week => $X!(
				WholeUnit::Week, "week",
				rational, "604800",
				(WholeUnit::Second, 1f64)
			),

			WholeUnit::Month => $X!(
				WholeUnit::Month, "month",
				rational, "2629746",
				(WholeUnit::Second, 1f64)
			),

			WholeUnit::Fortnight => $X!(
				WholeUnit::Fortnight, "fortnight",
				rational, "1209600",
				(WholeUnit::Second, 1f64)
			),

			WholeUnit::GregorianYear => $X!(
				WholeUnit::GregorianYear, "year",
				rational, "31557000",
				(WholeUnit::Second, 1f64)
			),

			WholeUnit::JulianYear => $X!(
				WholeUnit::JulianYear, "julianYear",
				rational, "31557600",
				(WholeUnit::Second, 1f64)
			),



			// Length

			WholeUnit::Angstrom => $X!(
				WholeUnit::Angstrom, "Å",
				rational, "1e-10",
				(WholeUnit::Meter, 1f64)
			),

			WholeUnit::Thou => $X!(
				WholeUnit::Thou, "thou",
				rational, "0.0000254",
				(WholeUnit::Meter, 1f64)
			),

			WholeUnit::Point => $X!(
				WholeUnit::Point, "pt",
				rational, "0.0003514598",
				(WholeUnit::Meter, 1f64)
			),

			WholeUnit::Inch => $X!(
				WholeUnit::Inch, "in",
				rational, "0.0254",
				(WholeUnit::Meter, 1f64)
			),

			WholeUnit::Foot => $X!(
				WholeUnit::Foot, "ft",
				rational, "0.3048",
				(WholeUnit::Meter, 1f64)
			),

			WholeUnit::Yard => $X!(
				WholeUnit::Yard, "yd",
				rational, "0.9144",
				(WholeUnit::Meter, 1f64)
			),

			WholeUnit::Furlong => $X!(
				WholeUnit::Furlong, "furlong",
				rational, "201.17",
				(WholeUnit::Meter, 1f64)
			),

			WholeUnit::Mile => $X!(
				WholeUnit::Mile, "mi",
				rational, "1609.344",
				(WholeUnit::Meter, 1f64)
			),

			WholeUnit::AstronomicalUnit => $X!(
				WholeUnit::AstronomicalUnit, "AU",
				rational, "149597870700",
				(WholeUnit::Meter, 1f64)
			),

			WholeUnit::Lightyear => $X!(
				WholeUnit::Lightyear, "ly",
				rational, "9460730472580800",
				(WholeUnit::Meter, 1f64)
			),

			WholeUnit::Parsec => $X!(
				WholeUnit::Parsec, "pc",
				float, "3.085677581e16",
				(WholeUnit::Meter, 1f64)
			),

			WholeUnit::Barn => $X!(
				WholeUnit::Barn, "b",
				rational, "1e-28",
				(WholeUnit::Meter, 2f64)
			),

			WholeUnit::Hectare => $X!(
				WholeUnit::Hectare, "ha",
				rational, "10000",
				(WholeUnit::Meter, 2f64)
			),

			WholeUnit::Acre => $X!( // 66 x 660 feet
				WholeUnit::Acre, "acre",
				rational, "4046.8564224",
				(WholeUnit::Meter, 2f64)
			),



			// Volume
			WholeUnit::Liter => $X!(
				WholeUnit::Liter, "l",
				rational, "0.001",
				(WholeUnit::Meter, 3f64)
			),

			WholeUnit::Hogshead => $X!(
				WholeUnit::Hogshead, "hogshead",
				rational, "0.2385", // 63 gallons
				(WholeUnit::Meter, 3f64)
			),

			WholeUnit::USGallon => $X!(
				WholeUnit::USGallon, "gal",
				rational, "0.003785411784",
				(WholeUnit::Meter, 3f64)
			),

			WholeUnit::Quart => $X!(
				WholeUnit::Quart, "qt",
				rational, "0.000946352946",
				(WholeUnit::Meter, 3f64)
			),

			WholeUnit::ImperialGallon => $X!(
				WholeUnit::ImperialGallon, "impgal",
				rational, "0.00454609",
				(WholeUnit::Meter, 3f64)
			),

			WholeUnit::Cup => $X!(
				WholeUnit::Cup, "cup",
				rational, "0.0002365882365",
				(WholeUnit::Meter, 3f64)
			),

			WholeUnit::Floz => $X!(
				WholeUnit::Floz, "floz",
				rational, "0.0000295735295625",
				(WholeUnit::Meter, 3f64)
			),

			WholeUnit::Pint => $X!(
				WholeUnit::Pint, "pint",
				rational, "0.00056826125",
				(WholeUnit::Meter, 3f64)
			),

			WholeUnit::Tablespoon => $X!(
				WholeUnit::Tablespoon, "tbsp",
				rational, "0.00001478676478125",
				(WholeUnit::Meter, 3f64)
			),

			WholeUnit::Teaspoon => $X!(
				WholeUnit::Teaspoon, "tsp",
				rational, "0.000005",
				(WholeUnit::Meter, 3f64)
			),



			// Pressure

			WholeUnit::Pascal => $X!(
				WholeUnit::Pascal, "Pa",
				rational, "1000",
				(WholeUnit::Gram, 1f64),
				(WholeUnit::Meter, -1f64),
				(WholeUnit::Second, -2f64)
			),

			WholeUnit::Bar => $X!(
				WholeUnit::Bar, "bar",
				rational, "100000000",
				(WholeUnit::Gram, 1f64),
				(WholeUnit::Meter, -1f64),
				(WholeUnit::Second, -2f64)
			),

			WholeUnit::Barye => $X!(
				WholeUnit::Barye, "Ba",
				rational, "100",
				(WholeUnit::Gram, 1f64),
				(WholeUnit::Meter, -1f64),
				(WholeUnit::Second, -2f64)
			),

			WholeUnit::Atmosphere => $X!(
				WholeUnit::Atmosphere, "atm",
				rational, "101325000",
				(WholeUnit::Gram, 1f64),
				(WholeUnit::Meter, -1f64),
				(WholeUnit::Second, -2f64)
			),

			WholeUnit::Psi => $X!(
				WholeUnit::Psi, "psi",
				rational, "6894757.2931783",
				(WholeUnit::Gram, 1f64),
				(WholeUnit::Meter, -1f64),
				(WholeUnit::Second, -2f64)
			),

			WholeUnit::MillimeterMercury => $X!(
				WholeUnit::MillimeterMercury, "mmHg",
				rational, "133322.387415",
				(WholeUnit::Gram, 1f64),
				(WholeUnit::Meter, -1f64),
				(WholeUnit::Second, -2f64)
			),

			WholeUnit::Torr => $X!(
				WholeUnit::Torr, "torr",
				rational_frac, (101325000, 760),
				(WholeUnit::Gram, 1f64),
				(WholeUnit::Meter, -1f64),
				(WholeUnit::Second, -2f64)
			),

			WholeUnit::MeterSeaWater => $X!(
				WholeUnit::MeterSeaWater, "MSW",
				rational, "10000000",
				(WholeUnit::Gram, 1f64),
				(WholeUnit::Meter, -1f64),
				(WholeUnit::Second, -2f64)
			),

			WholeUnit::FootSeaWater => $X!(
				WholeUnit::FootSeaWater, "FSW",
				rational, "3064330",
				(WholeUnit::Gram, 1f64),
				(WholeUnit::Meter, -1f64),
				(WholeUnit::Second, -2f64)
			),

		}


	}
}
pub (self) use unit_db;