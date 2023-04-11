use std::collections::HashMap;
use std::ops::{
	Mul, Div,
	MulAssign, DivAssign
};

use super::Scalar;
use super::Quantity;

#[derive(Debug)]
#[derive(Hash)]
#[derive(Eq, PartialEq)]
#[derive(Copy, Clone)]
pub enum BaseUnit {
	Second,
	Meter,
	Kilogram,
	Ampere,
	Kelvin,
	Mole,
	Candela,

	Foot,
}

impl BaseUnit {
	pub fn is_base(&self) -> bool {
		match self {
			BaseUnit::Second
			| BaseUnit::Meter
			| BaseUnit::Kilogram
			| BaseUnit::Ampere
			| BaseUnit::Kelvin
			| BaseUnit::Mole
			| BaseUnit::Candela
			=> true,

			_ => false
		}
	}

	pub fn to_base(&self) -> Option<Quantity> {
		match self {
			BaseUnit::Foot => Some(Quantity {
				v: Scalar::new_float_from_string("0.3048").unwrap(),
				u: Unit::from_array(&[(BaseUnit::Meter, Scalar::new_rational(1f64).unwrap())])
			}),
			_ => None
		}
	}
}


#[derive(Debug)]
#[derive(Clone)]
pub struct Unit {
	// Unit, power.
	pub val: HashMap<BaseUnit, Scalar>
}


impl ToString for Unit {
	fn to_string(&self) -> String {
		if self.unitless() { return String::new(); };

		let mut top_empty = true;
		let mut bottom_empty = true;

		for (_, p) in &self.val {
			if p.is_positive() {
				top_empty = false;
			} else {
				bottom_empty = false;
			}
		};

		let mut t = String::new();
		let mut b = String::new();

		for (u, p) in &self.val {
			let c = match u {
				BaseUnit::Second => "s",
				BaseUnit::Meter => "m",
				BaseUnit::Kilogram => "kg",
				BaseUnit::Ampere => "a",
				BaseUnit::Kelvin => "k",
				BaseUnit::Mole => "mol",
				BaseUnit::Candela => "c",

				BaseUnit::Foot => "ft",
			};

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
		return Unit{
			val: HashMap::new()
		}
	}

	pub fn from_array(a: &[(BaseUnit, Scalar)]) -> Unit {
		let mut n = Unit::new();
		for (u, p) in a.iter() {
			n.insert(*u, p.clone());
		}
		return n;
	}

	pub fn unitless(&self) -> bool { self.val.len() == 0 }

	pub fn insert(&mut self, u: BaseUnit, p: Scalar) {
		match self.val.get_mut(&u) {
			Some(i) => {
				let n = i.clone() + p;

				if n.is_zero() {
					self.val.remove(&u);
				} else { *i = n; }
			},
			None => { self.val.insert(u, p); }
		};
	}

	pub fn pow(&self, pwr: Scalar) -> Unit {
		let mut u = self.clone();
		for (_, p) in &mut u.val {
			*p *= pwr.clone();
		};
		return u;
	}
}


impl PartialEq for Unit {
	fn eq(&self, other: &Self) -> bool {
		for (u, p) in &other.val {
			match self.val.get(u) {
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
		for (u, p) in &other.val { o.insert(*u, p.clone()); }
		return o;
	}
}

impl MulAssign for Unit where {
	fn mul_assign(&mut self, other: Self) {
		for (u, p) in &other.val { self.insert(*u, p.clone()); }
	}
}

impl Div for Unit {
	type Output = Self;

	fn div(self, other: Self) -> Self::Output {
		let mut o = self.clone();
		for (u, p) in &other.val { o.insert(*u, -p.clone()); }
		return o;
	}
}

impl DivAssign for Unit where {
	fn div_assign(&mut self, other: Self) {
		for (u, p) in &other.val { self.insert(*u, -p.clone()); }
	}
}
