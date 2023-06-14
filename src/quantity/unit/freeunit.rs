use std::hash::Hash;

use crate::quantity::Scalar;
use crate::quantity::Quantity;
use super::WholeUnit;
use super::Prefix;


#[derive(Debug)]
#[derive(Hash)]
#[derive(Copy, Clone)]
#[derive(PartialEq, Eq)]
pub struct FreeUnit {
	pub (in super) whole: WholeUnit,
	pub (in super) prefix: Prefix
}


impl ToString for FreeUnit {
	fn to_string(&self) -> String {

		let s = self.whole.to_string();
		let p = self.prefix.to_string();

		format!("{p}{s}")
	}
}

impl FreeUnit {
	pub fn from_whole(whole: WholeUnit) -> FreeUnit {
		return FreeUnit { whole, prefix: Prefix::None }
	}

	pub fn from_whole_prefix(whole: WholeUnit, prefix: Prefix) -> FreeUnit { FreeUnit {whole, prefix} }
	pub fn set_prefix(&mut self, prefix: Prefix) { self.prefix = prefix; }
	pub fn get_prefix(&self) -> Prefix { self.prefix }

	/// Returns a quantity q, so that self * q
	/// gives a quantity in base units.
	pub fn to_base_factor(&self) -> Quantity {

		let q = self.whole.base_factor();
		let mut q = q.unwrap_or(Quantity::new_rational_from_string("1").unwrap());

		let mut p = self.prefix.to_ratio();
		p.insert_unit(FreeUnit::from_whole(self.whole), Scalar::new_rational(1f64).unwrap());
		p.insert_unit(FreeUnit::from_whole_prefix(self.whole, self.prefix), Scalar::new_rational(-1f64).unwrap());
		q.mul_assign_no_convert(p);

		return q;
	}

	// Get this unit in terms of base units
	pub fn to_base(&self) -> Quantity {
		let q = self.whole.base_factor();
		let mut q = q.unwrap_or(Quantity::new_rational_from_string("1").unwrap());

		// Don't divide by self
		q.insert_unit(FreeUnit::from_whole_prefix(self.whole, self.prefix), Scalar::new_rational(1f64).unwrap());

		return q;
	}
}


