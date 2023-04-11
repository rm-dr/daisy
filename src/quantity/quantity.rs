use std::ops::{
	Add, Sub, Mul, Div,
	Neg, Rem,

	AddAssign, SubAssign,
	MulAssign, DivAssign
};
use std::cmp::Ordering;

use crate::quantity::Unit;
use crate::quantity::BaseUnit;

use crate::quantity::Scalar;

#[derive(Debug)]
#[derive(Clone)]
pub struct Quantity {
	pub v: Scalar,
	pub u: Unit
}



impl ToString for Quantity {
	fn to_string(&self) -> String {
		let n = self.v.to_string();
		if self.unitless() { return n; }

		let u = self.u.to_string();
		if self.is_one() { return u; };

		return format!("{n} {u}");
	}
}

impl Quantity {
	pub fn to_string_outer(&self) -> String {
		let n = self.v.to_string();
		if self.unitless() { return n; }

		let u = self.u.to_string();
		return format!("{n} {u}");
	}

	pub fn new_float(f: f64) -> Option<Quantity> {
		let v = Scalar::new_float(f);
		if v.is_none() { return None; }

		return Some(Quantity{
			v: v.unwrap(),
			u: Unit::new()
		});
	}

	pub fn new_rational(f: f64) -> Option<Quantity> {
		let v = Scalar::new_rational(f);
		if v.is_none() { return None; }

		return Some(Quantity{
			v: v.unwrap(),
			u: Unit::new()
		});
	}

	pub fn new_float_from_string(s: &str) -> Option<Quantity> {
		let v = Scalar::new_float_from_string(s);
		if v.is_none() { return None; }

		return Some(Quantity{
			v: v.unwrap(),
			u: Unit::new()
		});
	}

	pub fn new_rational_from_string(s: &str) -> Option<Quantity> {
		let v = Scalar::new_rational_from_string(s);
		if v.is_none() { return None; }

		return Some(Quantity{
			v: v.unwrap(),
			u: Unit::new()
		});
	}

	pub fn from_scalar(s: Scalar) -> Quantity {
		return Quantity{
			v: s,
			u: Unit::new()
		};
	}

	pub fn insert_unit(&mut self, ui: BaseUnit, pi: Scalar) { self.u.insert(ui, pi) }
	pub fn set_unit(&mut self, u: Unit) { self.u = u; }

	pub fn from_unit_string(s: &str) -> Option<Quantity> {
		// Base Units
		let b = match s {
			"m" => Some(BaseUnit::Meter),
			"s" => Some(BaseUnit::Second),
			"kg" => Some(BaseUnit::Kilogram),
			"a" => Some(BaseUnit::Ampere),
			"k" => Some(BaseUnit::Kelvin),
			"mol" => Some(BaseUnit::Mole),
			"c" => Some(BaseUnit::Candela),
			"ft" => Some(BaseUnit::Foot),
			"mile" => Some(BaseUnit::Mile),
			"hour" => Some(BaseUnit::Hour),
			"min" => Some(BaseUnit::Minute),
			_ => { None }
		};

		if b.is_some() {
			let mut u = Unit::new();
			u.insert(b.unwrap(), Scalar::new_rational(1f64).unwrap());

			let mut q = Quantity::new_rational(1f64).unwrap();
			q.set_unit(u);

			return Some(q);
		};

		return None;
	}

	pub fn convert_to(self, other: Quantity) -> Option<Quantity> {
		let fa = self.u.to_base_factor();
		let fb = other.u.to_base_factor();
		let r = self * fa / fb;

		// If this didn't work, units are incompatible
		if r.u != other.u { return None; };

		return Some(r);
	}

}


macro_rules! quant_foward {
	( $x:ident ) => {
		pub fn $x(&self) -> Quantity {
			if !self.unitless() { panic!() }
			Quantity {
				v: self.v.$x(),
				u: self.u.clone()
			}
		}
	}
}

impl Quantity {

	pub fn is_zero(&self) -> bool { self.v.is_zero() }
	pub fn is_one(&self) -> bool { self.v.is_one() }
	pub fn is_nan(&self) -> bool { self.v.is_nan() }
	pub fn is_negative(&self) -> bool { self.v.is_negative() }
	pub fn is_positive(&self) -> bool { self.v.is_positive() }
	pub fn unitless(&self) -> bool { self.u.unitless() }
	pub fn unit(&self) -> &Unit { &self.u }

	quant_foward!(fract);
	quant_foward!(abs);
	quant_foward!(floor);
	quant_foward!(ceil);
	quant_foward!(round);
	quant_foward!(sin);
	quant_foward!(cos);
	quant_foward!(tan);
	quant_foward!(asin);
	quant_foward!(acos);
	quant_foward!(atan);
	quant_foward!(sinh);
	quant_foward!(cosh);
	quant_foward!(tanh);
	quant_foward!(asinh);
	quant_foward!(acosh);
	quant_foward!(atanh);
	quant_foward!(exp);
	quant_foward!(ln);
	quant_foward!(log10);
	quant_foward!(log2);

	pub fn log(&self, base: Quantity) -> Quantity {
		if !self.unitless() { panic!() }
		Quantity {
			v: self.v.log(base.v),
			u: self.u.clone()
		}
	}

	pub fn pow(&self, pwr: Quantity) -> Quantity {
		Quantity {
			v: self.v.pow(pwr.v.clone()),
			u: self.u.pow(pwr.v)
		}
	}
}


impl Neg for Quantity where {
	type Output = Self;

	fn neg(self) -> Self::Output {
		Quantity {
			v: -self.v,
			u: self.u
		}
	}
}

impl Add for Quantity {
	type Output = Self;

	fn add(self, other: Self) -> Self::Output {
		if self.u != other.u { panic!() }

		Quantity {
			v: self.v + other.v,
			u: self.u
		}
	}
}

impl AddAssign for Quantity where {
	fn add_assign(&mut self, other: Self) {
		if self.u != other.u { panic!() }
		self.v += other.v
	}
}

impl Sub for Quantity {
	type Output = Self;

	fn sub(self, other: Self) -> Self::Output {
		if self.u != other.u { panic!() }

		Quantity {
			v: self.v - other.v,
			u: self.u
		}
	}
}

impl SubAssign for Quantity where {
	fn sub_assign(&mut self, other: Self) {
		if self.u != other.u { panic!() }
		self.v -= other.v
	}
}

impl Mul for Quantity {
	type Output = Self;

	fn mul(self, other: Self) -> Self::Output {
		Quantity {
			v: self.v * other.v,
			u: self.u * other.u
		}
	}
}

impl MulAssign for Quantity where {
	fn mul_assign(&mut self, other: Self) {
		self.v *= other.v;
		self.u *= other.u;
	}
}

impl Div for Quantity {
	type Output = Self;

	fn div(self, other: Self) -> Self::Output {
		Quantity {
			v: self.v / other.v,
			u: self.u / other.u
		}
	}
}

impl DivAssign for Quantity where {
	fn div_assign(&mut self, other: Self) {
		self.v /= other.v;
		self.u /= other.u;
	}
}

impl Rem<Quantity> for Quantity {
	type Output = Self;

	fn rem(self, other: Quantity) -> Self::Output {
		if !self.u.unitless() { panic!() }
		if !other.u.unitless() { panic!() }

		Quantity {
			v: self.v % other.v,
			u: self.u
		}
	}
}

impl PartialEq for Quantity {
	fn eq(&self, other: &Self) -> bool {
		if self.u != other.u {false} else {
			self.v == other.v
		}
	}
}

impl PartialOrd for Quantity {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		if self.u != other.u { panic!() }
		self.v.partial_cmp(&other.v)
	}
}