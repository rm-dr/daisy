use std::collections::HashMap;
use std::ops::{
	Mul, Div,
	MulAssign, DivAssign
};

use crate::quantity::Scalar;
use crate::quantity::Quantity;
use super::UnitBase;
use super::Prefix;
use super::FreeUnit;

#[derive(Debug)]
#[derive(Clone)]
pub struct Unit {
	// Unit, power.
	pub val: HashMap<FreeUnit, Scalar>
}

impl ToString for Unit {
	fn to_string(&self) -> String {
		if self.unitless() { return String::new(); };

		let mut top_empty = true;
		let mut bottom_empty = true;

		for (_, p) in self.get_val() {
			if p.is_positive() {
				top_empty = false;
			} else {
				bottom_empty = false;
			}
		};

		let mut t = String::new();
		let mut b = String::new();

		for (u, p) in self.get_val() {
			let c = u.to_string();

			if *p == Scalar::new_rational(1f64).unwrap() {
				t.push_str(&format!("{c}·"));
			} else if *p == Scalar::new_rational(-1f64).unwrap() {
				if top_empty {
					b.push_str(&format!("{c}⁻¹·"));
				} else {
					b.push_str(&format!("{c}·"));
				}
			} else if p.is_positive() {
				t.push_str(&format!("{c}^{}·", p.to_string()));
			} else {
				if top_empty {
					b.push_str(&format!("{c}^{}·", p.to_string()));
				} else {
					b.push_str(&format!("{c}^{}·", (-p.clone()).to_string()));
				}
			}
		};

		if top_empty {
			format!("{}", &b[..b.len()-2]) // Slice cuts off the last `·` (2 bytes)
		} else if bottom_empty {
			format!("{}", &t[..t.len()-2])
		} else {
			format!("{}/{}", &t[..t.len()-2], &b[..b.len()-2])
		}
	}
}

impl Unit {
	pub fn new() -> Unit {
		return Unit {
			val: HashMap::new()
		}
	}

	pub fn get_val(&self) -> &HashMap<FreeUnit, Scalar> { &self.val }
	pub fn get_val_mut(&mut self) -> &mut HashMap<FreeUnit, Scalar> { &mut self.val }
	pub fn unitless(&self) -> bool { self.get_val().len() == 0 }

	pub fn from_array(a: &[(FreeUnit, Scalar)]) -> Unit {
		let mut n = Unit::new();
		for (u, p) in a.iter() {
			n.insert(*u, p.clone());
		}
		return n;
	}

	pub fn prefixes_match(&self, other: &Unit) -> bool {
		let v = self.get_val();
		for (u, _) in other.get_val() {
			let k = v.get_key_value(u);

			if k.is_some() {
				let k = k.unwrap().0;
				if !u.same_with_prefix(k) { return false; }
			}
		}
		return true;
	}

	pub fn match_prefix_factor(&self, other: &Unit) -> Quantity {
		let mut f = Quantity::new_rational(1f64).unwrap();

		let v = self.get_val();
		for (ou, op) in other.get_val() {
			let k = v.get_key_value(ou);

			if k.is_some() {
				let (su, _) = k.unwrap();

				// Conversion factor ou -> basic
				let mut p = ou.prefix.to_ratio();
				p.insert_unit(FreeUnit::from_base(ou.base), Scalar::new_rational(1f64).unwrap());
				p.insert_unit(FreeUnit::from_base_prefix(ou.base, ou.prefix), Scalar::new_rational(-1f64).unwrap());

				// Conversion factor su -> basic
				let mut q = su.prefix.to_ratio();
				q.insert_unit(FreeUnit::from_base(su.base), Scalar::new_rational(1f64).unwrap());
				q.insert_unit(FreeUnit::from_base_prefix(su.base, su.prefix), Scalar::new_rational(-1f64).unwrap());

				f = f * (p / q).pow(Quantity::from_scalar(op.clone()));
			}
		}

		return f;
	}


	pub fn insert(&mut self, u: FreeUnit, p: Scalar) {
		let v = self.get_val_mut();
		match v.get_mut(&u) {
			Some(i) => {
				let n = i.clone() + p;

				if n.is_zero() {
					v.remove(&u);
				} else { *i = n; }
			},
			None => { v.insert(u, p); }
		};
	}

	pub fn pow(&self, pwr: Scalar) -> Unit {
		let mut u = self.clone();
		for (_, p) in u.get_val_mut() {
			*p *= pwr.clone();
		};
		return u;
	}


	pub fn from_string(s: &str) -> Option<Quantity> {
		// Base Units
		let b = match s {
			"m" => Some(UnitBase::Meter),
			"s" => Some(UnitBase::Second),
			"sec" => Some(UnitBase::Second),
			"g" => Some(UnitBase::Gram),
			"a" => Some(UnitBase::Ampere),
			"k" => Some(UnitBase::Kelvin),
			"mol" => Some(UnitBase::Mole),
			"c" => Some(UnitBase::Candela),
			"ft" => Some(UnitBase::Foot),
			"mile" => Some(UnitBase::Mile),
			"hour" => Some(UnitBase::Hour),
			"min" => Some(UnitBase::Minute),
			"day" => Some(UnitBase::Day),
			_ => { None }
		};

		if b.is_some() {
			let mut u = Unit::new();
			let b = FreeUnit::from_base(b.unwrap());

			u.insert(b, Scalar::new_rational(1f64).unwrap());

			let mut q = Quantity::new_rational(1f64).unwrap();
			q.set_unit(u);

			return Some(q);
		};

		if b.is_none() {
			if s == "kg" {
				let mut u = Unit::new();
				let b = FreeUnit::from_base_prefix(UnitBase::Gram, Prefix::Kilo);

				u.insert(b, Scalar::new_rational(1f64).unwrap());

				let mut q = Quantity::new_rational(1f64).unwrap();
				q.set_unit(u);

				return Some(q);
			}

			if s == "km" {
				let mut u = Unit::new();
				let b = FreeUnit::from_base_prefix(UnitBase::Meter, Prefix::Kilo);

				u.insert(b, Scalar::new_rational(1f64).unwrap());

				let mut q = Quantity::new_rational(1f64).unwrap();
				q.set_unit(u);

				return Some(q);
			}
		}

		return None;
	}


	pub fn to_base_factor(&self) -> Quantity {
		let mut q = Quantity::new_rational(1f64).unwrap();

		for (u, p) in self.get_val().iter() {
			let b = u.to_base_factor();
			q *= b.pow(Quantity::from_scalar(p.clone()));
		}

		return q;
	}
}


impl PartialEq for Unit {
	fn eq(&self, other: &Self) -> bool {
		let v = self.get_val();
		for (u, p) in other.get_val() {
			match v.get(u) {
				Some(i) => { if i != p { return false; } },
				None => { return false; }
			};
		}
		return true;
	}
}

impl Mul for Unit {
	type Output = Self;

	fn mul(self, other: Self) -> Self::Output {
		let mut o = self.clone();
		for (u, p) in other.get_val() { o.insert(*u, p.clone()); }
		return o;
	}
}

impl MulAssign for Unit where {
	fn mul_assign(&mut self, other: Self) {
		for (u, p) in other.get_val() { self.insert(*u, p.clone()); }
	}
}

impl Div for Unit {
	type Output = Self;

	fn div(self, other: Self) -> Self::Output {
		let mut o = self.clone();
		for (u, p) in other.get_val() { o.insert(*u, -p.clone()); }
		return o;
	}
}

impl DivAssign for Unit where {
	fn div_assign(&mut self, other: Self) {
		for (u, p) in other.get_val() { self.insert(*u, -p.clone()); }
	}
}
