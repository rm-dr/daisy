use std::hash::{Hash, Hasher};

use crate::quantity::Scalar;
use crate::quantity::Quantity;
use super::UnitBase;
use super::Prefix;
use super::Unit;


#[derive(Debug)]
#[derive(Copy, Clone)]
pub struct FreeUnit {
	pub (in super) base: UnitBase,
	pub (in super) prefix: Prefix
}

impl Hash for FreeUnit {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.base.hash(state);
	}
}

impl Eq for FreeUnit {}
impl PartialEq for FreeUnit {
	fn eq(&self, other: &Self) -> bool {
		self.base.eq(&other.base)
	}
}


macro_rules! quick_base_factor {
	(float, $u:expr, $s:expr, $( ($x:expr, $p:expr) ),* ) => {
		Some(Quantity {
			scalar: Scalar::new_float_from_string($s).unwrap(),
			unit: Unit::from_array(&[
				$(
					(FreeUnit::from_base($x), Scalar::new_rational($p).unwrap()),
				)*
				(FreeUnit::from_base($u), Scalar::new_rational(-1f64).unwrap())
			])
		})
	};

	(rational, $u:expr, $s:expr, $( ($x:expr, $p:expr) ),* ) => {
		Some(Quantity {
			scalar: Scalar::new_float_from_string($s).unwrap(),
			unit: Unit::from_array(&[
				$(
					(FreeUnit::from_base($x), Scalar::new_rational($p).unwrap()),
				)*
				(FreeUnit::from_base($u), Scalar::new_rational(-1f64).unwrap())
			])
		})
	};
}


impl FreeUnit {
	pub fn from_base(base: UnitBase) -> FreeUnit {
		return FreeUnit { base, prefix: Prefix::None }
	}

	pub fn from_base_prefix(base: UnitBase, prefix: Prefix) -> FreeUnit { FreeUnit {base, prefix} }
	pub fn set_prefix(&mut self, prefix: Prefix) { self.prefix = prefix; }
	pub fn get_prefix(&self) -> Prefix { self.prefix }

	pub fn same_with_prefix(&self, other: &FreeUnit) -> bool {
		self.base.eq(&other.base) && self.prefix.eq(&other.prefix)
	}


	pub fn to_base_factor(&self) -> Quantity {
		let q = match self.base {
			// Returns the unit we need to multiply by to get a base
			// unit, or `None` if this is already a base unit.
			//
			// Example:
			// 1 foot  = 0.3048 m,
			// so 1 ft * (0.3084 m / ft) will give meters.
			//
			// The units here MUST be in terms of base units.
			// If they aren't, things will break.

			UnitBase::Foot => quick_base_factor!(float,
				UnitBase::Foot,
				"0.3048",
				(UnitBase::Meter, 1f64)
			),

			UnitBase::Inch => quick_base_factor!(float,
				UnitBase::Inch,
				"0.0254",
				(UnitBase::Meter, 1f64)
			),

			UnitBase::Mile => quick_base_factor!(rational,
				UnitBase::Mile,
				"1609",
				(UnitBase::Meter, 1f64)
			),

			UnitBase::Minute => quick_base_factor!(rational,
				UnitBase::Minute,
				"60",
				(UnitBase::Second, 1f64)
			),

			UnitBase::Hour => quick_base_factor!(rational,
				UnitBase::Hour,
				"3600",
				(UnitBase::Second, 1f64)
			),

			UnitBase::Day => quick_base_factor!(rational,
				UnitBase::Day,
				"86400",
				(UnitBase::Second, 1f64)
			),

			// Only base units should be missing a conversion factor.
			_ => None
		};

		let mut q = q.unwrap_or(Quantity::new_rational_from_string("1").unwrap());

		let mut p = self.prefix.to_ratio();
		p.insert_unit(FreeUnit::from_base(self.base), Scalar::new_rational(1f64).unwrap());
		p.insert_unit(FreeUnit::from_base_prefix(self.base, self.prefix), Scalar::new_rational(-1f64).unwrap());
		q *= p;

		return q;
	}
}


impl ToString for FreeUnit {
	fn to_string(&self) -> String {
		let s = match self.base {
			UnitBase::Second => "s",
			UnitBase::Meter => "m",
			UnitBase::Gram => "g",
			UnitBase::Ampere => "a",
			UnitBase::Kelvin => "k",
			UnitBase::Mole => "mol",
			UnitBase::Candela => "c",

			UnitBase::Foot => "ft",
			UnitBase::Inch => "in",
			UnitBase::Mile => "mile",

			UnitBase::Hour => "hour",
			UnitBase::Minute => "min",
			UnitBase::Day => "day",
		};

		let p = self.prefix.to_string();

		format!("{p}{s}")
	}
}
