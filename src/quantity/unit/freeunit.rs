use std::hash::Hash;

use crate::quantity::Scalar;
use crate::quantity::Quantity;
use super::WholeUnit;
use super::Prefix;
use super::Unit;
use super::unit_db;


#[derive(Debug)]
#[derive(Hash)]
#[derive(Copy, Clone)]
#[derive(PartialEq, Eq)]
pub struct FreeUnit {
	pub (in super) whole: WholeUnit,
	pub (in super) prefix: Prefix
}


macro_rules! unpack_string {
	(
		$u:expr, $s:expr,
		$( $_:expr ),*
	) => { $s };
}

impl ToString for FreeUnit {
	fn to_string(&self) -> String {

		let s = unit_db!(self.whole, unpack_string);
		let p = self.prefix.to_string();

		format!("{p}{s}")
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
					(FreeUnit::from_whole($u), Scalar::new_rational($p).unwrap()),
				)*
				(FreeUnit::from_whole($unit), Scalar::new_rational(-1f64).unwrap())
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
					(FreeUnit::from_whole($u), Scalar::new_rational($p).unwrap()),
				)*
				(FreeUnit::from_whole($unit), Scalar::new_rational(-1f64).unwrap())
			])
		})
	};

	(
		$unit:expr,
		$display_string:expr,
		rational_frac,
		($t:expr, $b:expr),
		$( ($u:expr, $p:expr) ),*
	) => {
		Some(Quantity {
			scalar: Scalar::new_rational_from_frac($t, $b).unwrap(),
			unit: Unit::from_array(&[
				$(
					(FreeUnit::from_whole($u), Scalar::new_rational($p).unwrap()),
				)*
				(FreeUnit::from_whole($unit), Scalar::new_rational(-1f64).unwrap())
			])
		})
	};
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

		let q = unit_db!(self.whole, unpack_base_factor);
		let mut q = q.unwrap_or(Quantity::new_rational_from_string("1").unwrap());

		let mut p = self.prefix.to_ratio();
		p.insert_unit(FreeUnit::from_whole(self.whole), Scalar::new_rational(1f64).unwrap());
		p.insert_unit(FreeUnit::from_whole_prefix(self.whole, self.prefix), Scalar::new_rational(-1f64).unwrap());
		q.mul_assign_no_convert(p);

		return q;
	}

	// Get this unit in terms of base units
	pub fn get_base(&self) -> Quantity {
		let q = unit_db!(self.whole, unpack_base_factor);
		let mut q = q.unwrap_or(Quantity::new_rational_from_string("1").unwrap());

		// Don't divide by self
		q.insert_unit(FreeUnit::from_whole_prefix(self.whole, self.prefix), Scalar::new_rational(1f64).unwrap());

		return q;
	}
}


