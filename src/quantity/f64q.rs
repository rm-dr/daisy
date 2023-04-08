use std::ops::{
	Add, Sub, Mul, Div,
	Neg, Rem,

	AddAssign, SubAssign,
	MulAssign, DivAssign
};

use std::cmp::Ordering;

use crate::quantity::wrap_float;
use crate::quantity::Quantity;
use crate::quantity::QuantBase;
use crate::quantity::FloatBase;


macro_rules! foward {
	( $x:ident ) => {
		fn $x(&self) -> Quantity {
			wrap_float!(F64Q{ val: self.val.clone().$x() })
		}
	}
}

#[derive(Debug)]
#[derive(Clone)]
pub struct F64Q where {
	pub val: f64
}

impl ToString for F64Q {
	fn to_string(&self) -> String { self.val.to_string() }
}


impl QuantBase for F64Q {

	foward!(fract);

	fn is_zero(&self) -> bool {self.val == 0f64}
	fn is_negative(&self) -> bool { self.val.is_sign_negative() }
	fn is_positive(&self) -> bool { self.val.is_sign_positive() }

	foward!(abs);
	foward!(floor);
	foward!(ceil);
	foward!(round);

	foward!(sin);
	foward!(cos);
	foward!(tan);
	foward!(asin);
	foward!(acos);
	foward!(atan);

	foward!(sinh);
	foward!(cosh);
	foward!(tanh);
	foward!(asinh);
	foward!(acosh);
	foward!(atanh);

	foward!(exp);
	foward!(ln);
	foward!(log10);
	foward!(log2);

	fn log(&self, base: Quantity) -> Quantity {
		wrap_float!(F64Q{ val: self.val.clone().log10() }) /
		Quantity::float_from_rat(&base).log10()
	}

	fn pow(&self, base: Quantity) -> Quantity {
		match base {
			Quantity::Rational { .. } => self.pow(Quantity::float_from_rat(&base)),
			Quantity::Float { v } => wrap_float!(F64Q{ val: self.val.clone().powf(v.val) })
		}
		
	}

}

impl FloatBase for F64Q {
	fn from_f64(f: f64) -> Option<F64Q> {
		return Some(F64Q{ val: f });
	}

	fn from_string(s: &str) -> Option<F64Q> {
		let v = s.parse::<f64>();
		let v = match v {
			Ok(x) => x,
			Err(_) => return None
		};

		return Some(F64Q{ val: v });
	}
}



impl Add for F64Q where {
	type Output = Self;

	fn add(self, other: Self) -> Self::Output {
		Self { val: self.val + other.val}
	}
}

impl AddAssign for F64Q where {
	fn add_assign(&mut self, other: Self) {
		self.val += other.val;
	}
}

impl Sub for F64Q {
	type Output = Self;

	fn sub(self, other: Self) -> Self::Output {
		Self {val: self.val - other.val}
	}
}

impl SubAssign for F64Q where {
	fn sub_assign(&mut self, other: Self) {
		self.val -= other.val;
	}
}

impl Mul for F64Q {
	type Output = Self;

	fn mul(self, other: Self) -> Self::Output {
		Self {val: self.val * other.val}
	}
}

impl MulAssign for F64Q where {
	fn mul_assign(&mut self, other: Self) {
		self.val *= other.val;
	}
}

impl Div for F64Q {
	type Output = Self;

	fn div(self, other: Self) -> Self::Output {
		Self {val: self.val / other.val}
	}
}

impl DivAssign for F64Q where {
	fn div_assign(&mut self, other: Self) {
		self.val /= other.val;
	}
}

impl Neg for F64Q where {
	type Output = Self;

	fn neg(self) -> Self::Output {
		Self {val: -self.val}
	}
}

impl Rem<F64Q> for F64Q {
	type Output = Self;

	fn rem(self, modulus: F64Q) -> Self::Output {
		if {
			(!self.fract().is_zero()) ||
			(!modulus.fract().is_zero())
		} { panic!() }

		F64Q{val : self.val.fract() % modulus.val.fract()}
	}
}

impl PartialEq for F64Q {
	fn eq(&self, other: &Self) -> bool {
		self.val == other.val
	}
}

impl PartialOrd for F64Q {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		self.val.partial_cmp(&other.val)
	}
}