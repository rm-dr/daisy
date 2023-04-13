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


