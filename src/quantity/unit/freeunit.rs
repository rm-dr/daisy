use std::hash::{Hash, Hasher};

use crate::quantity::Scalar;
use crate::quantity::Quantity;
use super::UnitBase;
use super::Prefix;
use super::Unit;
use super::unit_db;


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


macro_rules! unpack_base_factor {
	(
		$unit:expr,
		$display_string:expr,
		base
	) => { None };

	(
		$unit:expr,
		$display_string:expr,
		float,
		$value:expr,
		$( ($u:expr, $p:expr) ),*
	) => {
		Some(Quantity {
			scalar: Scalar::new_float_from_string($value).unwrap(),
			unit: Unit::from_array(&[
				$(
					(FreeUnit::from_base($u), Scalar::new_rational($p).unwrap()),
				)*
				(FreeUnit::from_base($unit), Scalar::new_rational(-1f64).unwrap())
			])
		})
	};

	(
		$unit:expr,
		$display_string:expr,
		rational,
		$value:expr,
		$( ($u:expr, $p:expr) ),*
	) => {
		Some(Quantity {
			scalar: Scalar::new_rational_from_string($value).unwrap(),
			unit: Unit::from_array(&[
				$(
					(FreeUnit::from_base($u), Scalar::new_rational($p).unwrap()),
				)*
				(FreeUnit::from_base($unit), Scalar::new_rational(-1f64).unwrap())
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

		let q = unit_db!(self.base, unpack_base_factor);
		let mut q = q.unwrap_or(Quantity::new_rational_from_string("1").unwrap());

		let mut p = self.prefix.to_ratio();
		p.insert_unit(FreeUnit::from_base(self.base), Scalar::new_rational(1f64).unwrap());
		p.insert_unit(FreeUnit::from_base_prefix(self.base, self.prefix), Scalar::new_rational(-1f64).unwrap());
		q *= p;

		return q;
	}
}



macro_rules! unpack_string {
	(
		$u:expr, $s:expr,
		$( $_:expr ),*
	) => { $s };
}

impl ToString for FreeUnit {
	fn to_string(&self) -> String {

		let s = unit_db!(self.base, unpack_string);
		let p = self.prefix.to_string();

		format!("{p}{s}")
	}
}
