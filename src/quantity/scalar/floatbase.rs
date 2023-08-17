use rug::Float;
use rug::Assign;
use rug::ops::AssignRound;
use rug::ops::Pow;

use std::ops::{
	Add, Sub, Mul, Div,
	Neg, Rem,

	AddAssign, SubAssign,
	MulAssign, DivAssign
};

use std::cmp::Ordering;

use super::ScalarBase;
use super::PRINT_LEN;
use super::FLOAT_PRECISION;

#[derive(Debug)]
#[derive(Clone)]
pub struct FloatBase where {
	pub val: Float
}

impl FloatBase {
	pub fn from<T>(a: T) -> Option<FloatBase> where
		Float: Assign<T> + AssignRound<T>
	{
		let v = Float::with_val(FLOAT_PRECISION, a);
		return Some(FloatBase{ val: v });
	}
}

impl ToString for FloatBase {
	fn to_string(&self) -> String {
		let (sign, mut string, exp) = self.val.to_sign_string_exp(10, Some(PRINT_LEN));

		// zero, nan, or inf.
		let sign = if sign {"-"} else {""};
		if exp.is_none() { return format!("{sign}{string}"); }
		let exp = exp.unwrap();

		// Remove trailing zeros.
		// At this point, string is guaranteed to be nonzero.
		while string.chars().last().unwrap() == '0' {
			string.remove(string.len() - 1);
		}

		let exp_u: usize;

		if exp < 0 {
			exp_u = (-exp).try_into().unwrap()
		} else {
			exp_u = exp.try_into().unwrap()
		}

		if exp_u >= PRINT_LEN {
			// Exponential notation
			let pre = &string[0..1];
			let post = &string[1..];

			format!(
				"{pre}{}{post}e{}",
				if post.len() != 0 {"."} else {""},
				//if exp > 0 {"+"} else {""},
				exp - 1
			)
		} else {
			if exp <= 0 { // Decimal, needs `0.` and leading zeros
				format!(
					"{sign}0.{}{string}",
					"0".repeat(exp_u)
				)
			} else if exp_u < string.len() { // Decimal, needs only `.`
				format!(
					"{sign}{}.{}",
					&string[0..exp_u],
					&string[exp_u..]
				)
			} else { // Integer, needs trailing zeros
				format!(
					"{sign}{string}{}",
					"0".repeat(exp_u - string.len())
				)
			}
		}

	}
}


macro_rules! foward {
	( $x:ident ) => {
		fn $x(&self) -> Option<FloatBase> {
			Some(FloatBase{ val: self.val.clone().$x()})
		}
	}
}

impl ScalarBase for FloatBase {

	fn from_f64(f: f64) -> Option<FloatBase> {
		let v = Float::with_val(FLOAT_PRECISION, f);
		return Some(FloatBase{ val: v });
	}

	fn from_string(s: &str) -> Option<FloatBase> {
		let v = Float::parse(s);
		let v = match v {
			Ok(x) => x,
			Err(_) => return None
		};

		return Some(
			FloatBase{ val:
				Float::with_val(FLOAT_PRECISION, v)
			}
		);
	}

	foward!(fract);

	fn is_zero(&self) -> bool {self.val.is_zero()}
	fn is_one(&self) -> bool {self.val == Float::with_val(FLOAT_PRECISION, 1)}
	fn is_negative(&self) -> bool { self.val.is_sign_negative() }
	fn is_positive(&self) -> bool { self.val.is_sign_positive() }

	fn is_int(&self) -> bool {
		self.fract() == FloatBase::from_f64(0f64)
	}

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

	fn log(&self, base: FloatBase) -> Option<FloatBase> {
		Some(FloatBase{ val: self.val.clone().log10() } / base.log10().unwrap())
	}

	fn pow(&self, base: FloatBase) -> Option<FloatBase> {
		Some(FloatBase{ val: self.val.clone().pow(base.val)})
	}

}


impl Add for FloatBase where {
	type Output = Self;

	fn add(self, other: Self) -> Self::Output {
		Self { val: self.val + other.val}
	}
}

impl AddAssign for FloatBase where {
	fn add_assign(&mut self, other: Self) {
		self.val += other.val;
	}
}

impl Sub for FloatBase {
	type Output = Self;

	fn sub(self, other: Self) -> Self::Output {
		Self {val: self.val - other.val}
	}
}

impl SubAssign for FloatBase where {
	fn sub_assign(&mut self, other: Self) {
		self.val -= other.val;
	}
}

impl Mul for FloatBase {
	type Output = Self;

	fn mul(self, other: Self) -> Self::Output {
		Self {val: self.val * other.val}
	}
}

impl MulAssign for FloatBase where {
	fn mul_assign(&mut self, other: Self) {
		self.val *= other.val;
	}
}

impl Div for FloatBase {
	type Output = Self;

	fn div(self, other: Self) -> Self::Output {
		Self {val: self.val / other.val}
	}
}

impl DivAssign for FloatBase where {
	fn div_assign(&mut self, other: Self) {
		self.val /= other.val;
	}
}

impl Neg for FloatBase where {
	type Output = Self;

	fn neg(self) -> Self::Output {
		Self {val: -self.val}
	}
}

impl Rem<FloatBase> for FloatBase {
	type Output = Self;

	fn rem(self, modulus: FloatBase) -> Self::Output {
		if {
			(!self.fract().unwrap().is_zero()) ||
			(!modulus.fract().unwrap().is_zero())
		} { panic!() }

		FloatBase{val : self.val.trunc() % modulus.val.trunc()}
	}
}

impl PartialEq for FloatBase {
	fn eq(&self, other: &Self) -> bool {
		self.val == other.val
	}
}

impl PartialOrd for FloatBase {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		self.val.partial_cmp(&other.val)
	}
}