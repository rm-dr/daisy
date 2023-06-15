use std::ops::{
	Add, Sub, Mul, Div,
	Neg, Rem,

	AddAssign, SubAssign,
	MulAssign, DivAssign
};

use std::cmp::Ordering;
use super::ScalarBase;


macro_rules! foward {
	( $x:ident ) => {
		fn $x(&self) -> Option<F64Base> {
			Some(F64Base{ val: self.val.clone().$x() })
		}
	}
}

#[derive(Debug)]
#[derive(Clone)]
pub struct F64Base where {
	pub val: f64
}

impl ToString for F64Base {
	fn to_string(&self) -> String { self.val.to_string() }
}


impl ScalarBase for F64Base {

	fn from_f64(f: f64) -> Option<F64Base> {
		return Some(F64Base{ val: f });
	}

	fn from_string(s: &str) -> Option<F64Base> {
		let v = s.parse::<f64>();
		let v = match v {
			Ok(x) => x,
			Err(_) => return None
		};

		return Some(F64Base{ val: v });
	}

	foward!(fract);

	fn is_zero(&self) -> bool {self.val == 0f64}
	fn is_one(&self) -> bool {self.val == 1f64}
	fn is_negative(&self) -> bool { self.val.is_sign_negative() }
	fn is_positive(&self) -> bool { self.val.is_sign_positive() }

	foward!(abs);
	foward!(floor);
	foward!(ceil);
	foward!(round);

	foward!(sin);
	foward!(cos);
	foward!(tan);
	foward!(csc);
	foward!(sec);
	foward!(cot);
	foward!(asin);
	foward!(acos);
	foward!(atan);

	foward!(sinh);
	foward!(cosh);
	foward!(tanh);
	foward!(csch);
	foward!(sech);
	foward!(coth);
	foward!(asinh);
	foward!(acosh);
	foward!(atanh);

	foward!(exp);
	foward!(ln);
	foward!(log10);
	foward!(log2);

	fn log(&self, base: Self) -> Option<Self> {
		Some(F64Base{ val: self.val.clone().log10() } / base.log10().unwrap())
	}

	fn pow(&self, base: Self) -> Option<Self> {
		Some(F64Base{ val: self.val.clone().powf(base.val)})
	}

}

impl Add for F64Base where {
	type Output = Self;

	fn add(self, other: Self) -> Self::Output {
		Self { val: self.val + other.val}
	}
}

impl AddAssign for F64Base where {
	fn add_assign(&mut self, other: Self) {
		self.val += other.val;
	}
}

impl Sub for F64Base {
	type Output = Self;

	fn sub(self, other: Self) -> Self::Output {
		Self {val: self.val - other.val}
	}
}

impl SubAssign for F64Base where {
	fn sub_assign(&mut self, other: Self) {
		self.val -= other.val;
	}
}

impl Mul for F64Base {
	type Output = Self;

	fn mul(self, other: Self) -> Self::Output {
		Self {val: self.val * other.val}
	}
}

impl MulAssign for F64Base where {
	fn mul_assign(&mut self, other: Self) {
		self.val *= other.val;
	}
}

impl Div for F64Base {
	type Output = Self;

	fn div(self, other: Self) -> Self::Output {
		Self {val: self.val / other.val}
	}
}

impl DivAssign for F64Base where {
	fn div_assign(&mut self, other: Self) {
		self.val /= other.val;
	}
}

impl Neg for F64Base where {
	type Output = Self;

	fn neg(self) -> Self::Output {
		Self {val: -self.val}
	}
}

impl Rem<F64Base> for F64Base {
	type Output = Self;

	fn rem(self, modulus: F64Base) -> Self::Output {
		if {
			(!self.fract().unwrap().is_zero()) ||
			(!modulus.fract().unwrap().is_zero())
		} { panic!() }

		F64Base{val : self.val.fract() % modulus.val.fract()}
	}
}

impl PartialEq for F64Base {
	fn eq(&self, other: &Self) -> bool {
		self.val == other.val
	}
}

impl PartialOrd for F64Base {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		self.val.partial_cmp(&other.val)
	}
}