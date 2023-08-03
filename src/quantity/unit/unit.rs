use std::collections::HashMap;
use std::ops::{
	Mul, Div,
	MulAssign, DivAssign
};

use crate::quantity::Scalar;
use crate::quantity::Quantity;
use super::FreeUnit;
use super::freeunit_from_string;

#[derive(Debug)]
#[derive(Clone)]
pub struct Unit {
	// Unit, power.
	pub val: HashMap<FreeUnit, Scalar>
}

impl ToString for Unit {
	fn to_string(&self) -> String {

		if self.unitless() { return String::new(); };


		// Sort units by power
		let mut v: Vec<(&FreeUnit, &Scalar)> = self.get_val().iter().collect();
		v.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());

		let mut i = v.iter();
		let Some((mut u, mut p)) = i.next() else { panic!() };
		let mut done = false;

		// Positive powers
		let mut t = String::new();
		while p.is_positive() && !done {
			let c = u.to_string();

			if *p == Scalar::new_rational(1f64).unwrap() {
				t.push_str(&format!("{c}·"));
			} else if p.is_int() && !p.to_string().contains("e"){
				t.push_str(&c);
				for c in p.to_string().chars() {
					t.push( match c {
						//'-' => '⁻',
						'0' => '⁰',
						'1' => '¹',
						'2' => '²',
						'3' => '³',
						'4' => '⁴',
						'5' => '⁵',
						'6' => '⁶',
						'7' => '⁷',
						'8' => '⁸',
						'9' => '⁹',
						_ => unreachable!()
					});
				}
				t.push('·');
			} else {
				t.push_str(&format!("{c}^{}·", p.to_string()));
			}

			if let Some((a, b)) = i.next() {
				u = a; p = b;
			} else {done = true}
		};

		// Negative powers
		let mut b = String::new();
		let mut bottom_count = 0;
		while !done {
			let c = u.to_string();

			bottom_count += 1;
			if t.len() != 0 && *p == Scalar::new_rational(-1f64).unwrap() {
				b.push_str(&format!("{c}·"));
			} else if p.is_int() && !p.to_string().contains("e") {
				b.push_str(&c);
				for c in p.to_string().chars() {
					if c == '-' && t.len() != 0 { continue; }
					b.push( match c {
						'-' => '⁻',
						'0' => '⁰',
						'1' => '¹',
						'2' => '²',
						'3' => '³',
						'4' => '⁴',
						'5' => '⁵',
						'6' => '⁶',
						'7' => '⁷',
						'8' => '⁸',
						'9' => '⁹',
						_ => unreachable!()
					});
				}
				b.push('·');
			} else {
				b.push_str(&format!("{c}^{}·", p.to_string()));
			}

			if let Some((a, b)) = i.next() {
				u = a; p = b;
			} else {done = true}
		};

		// Slice cuts off the last `·` (2 bytes)
		if t.len() == 0 {
			return format!("{}", &b[..b.len() - 2]);
		} else if b.len() == 0 {
			return String::from(&t[..t.len() - 2]);
		} else {
			if bottom_count > 1 {
				return format!("{}/({})", &t[..t.len() - 2], &b[..b.len() - 2] );
			} else {
				return format!("{}/{}", &t[..t.len() - 2], &b[..b.len() - 2] );
			}
		}

	}
}

impl Unit {
	pub fn new() -> Unit {
		return Unit {
			val: HashMap::new()
		}
	}

	pub fn from_free(f: FreeUnit) -> Unit {
		let mut u = Unit {
			val: HashMap::new()
		};

		u.insert(f, Scalar::new_rational(1f64).unwrap());
		return u;
	}

	pub fn get_val(&self) -> &HashMap<FreeUnit, Scalar> { &self.val }
	pub fn get_val_mut(&mut self) -> &mut HashMap<FreeUnit, Scalar> { &mut self.val }
	pub fn unitless(&self) -> bool { self.get_val().len() == 0 }

	pub fn no_space(&self) -> bool {
		if self.get_val().len() == 1 {
			return self.get_val().keys().next().unwrap().whole.no_space();
		} else { return false; }
	}

	pub fn from_array(a: &[(FreeUnit, Scalar)]) -> Unit {
		let mut n = Unit::new();
		for (u, p) in a.iter() {
			n.insert(*u, p.clone());
		}
		return n;
	}

	// True if base units are the same
	// compatible <=> can be converted to
	pub fn compatible_with(&self, other: &Unit) -> bool {
		let s = self.clone() * self.to_base_factor().unit;
		let o = other.clone() * other.to_base_factor().unit;

		return o == s;
	}


	// True if all base units are the same AND there is a constant factor between their powers.
	// This is a generalization of `compatible_with`. `compatible_with` is true iff
	// `compatible_with_power` is one.
	pub fn compatible_with_power(&self, other: &Unit) -> Option<Scalar> {
		let mut flag;
		let mut pow_factor: Option<Scalar> = None;

		let sbu = self.to_base().unit;
		let obu = other.to_base().unit;

		for (us, ps) in sbu.get_val() {
			flag = false;
			for (uo, po) in obu.get_val() {
				if uo.whole == us.whole {
					if pow_factor.is_none() {
						pow_factor = Some(po.clone() / ps.clone());
					} else if let Some(ref f) = pow_factor {
						if *f != po.clone() / ps.clone() { return None; }
					}

					flag = true;
					break;
				}
			}
			if !flag { return None; }
		}

		pow_factor = None;
		for (uo, po) in obu.get_val() {
			flag = false;
			for (us, ps) in sbu.get_val() {
				if uo.whole == us.whole {
					if pow_factor.is_none() {
						pow_factor = Some(po.clone() / ps.clone());
					} else if let Some(ref f) = pow_factor {
						if *f != po.clone() / ps.clone() { return None; }
					}

					flag = true;
					break;
				}
			}
			if !flag { return None; }
		}

		return pow_factor;
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

	// Returns a unit `u` so that `self * u` contains only base units.
	pub fn to_base_factor(&self) -> Quantity {
		let mut q = Quantity::new_rational(1f64).unwrap();

		for (u, p) in self.get_val().iter() {
			let b = u.to_base_factor();
			q.mul_assign_no_convert(b.pow(Quantity::from_scalar(p.clone())));
		}

		return q;
	}

	// Returns a unit `u` equivalent to `self` that contains only base units.
	pub fn to_base(&self) -> Quantity {
		let mut q = Quantity::new_rational(1f64).unwrap();

		for (u, p) in self.get_val().iter() {
			let b = u.to_base();
			q.mul_assign_no_convert(b.pow(Quantity::from_scalar(p.clone())));
		}

		return q;
	}
}

impl Unit {
	pub fn from_string(s: &str) -> Option<Quantity> {
		let b = freeunit_from_string(s);
		if b.is_none() { return None; }
		let b = Unit::from_free(b.unwrap());
		let mut q = Quantity::new_rational(1f64).unwrap();
		q.set_unit(b);
		return Some(q);

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

		let v = other.get_val();
		for (u, p) in self.get_val() {
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
