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

use crate::quantity::wrap_float;
use crate::quantity::Quantity;
use crate::quantity::QuantBase;
use crate::quantity::FloatBase;
use crate::quantity::PRINT_LEN;

use super::FLOAT_PRECISION;


macro_rules! foward {
	( $x:ident ) => {
		fn $x(&self) -> Quantity {
			wrap_float!(FloatQ{ val: self.val.clone().$x()})
		}
	}
}

#[derive(Debug)]
#[derive(Clone)]
pub struct FloatQ where {
	pub val: Float
}

impl FloatQ {
	pub fn from<T>(a: T) -> Option<FloatQ> where
		Float: Assign<T> + AssignRound<T>
	{
		let v = Float::with_val(FLOAT_PRECISION, a);
		return Some(FloatQ{ val: v });
	}
}

impl ToString for FloatQ {
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


impl QuantBase for FloatQ {

	foward!(fract);

	fn is_zero(&self) -> bool {self.val.is_zero()}
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
		wrap_float!(FloatQ{ val: self.val.clone().log10() }) /
		Quantity::float_from_rat(&base).log10()
	}

	fn pow(&self, base: Quantity) -> Quantity {
		match base {
			Quantity::Rational { .. } => self.pow(Quantity::float_from_rat(&base)),
			Quantity::Float { v } => wrap_float!(FloatQ{ val: self.val.clone().pow(v.val)})
		}
		
	}

}

impl FloatBase for FloatQ {
	fn from_f64(f: f64) -> Option<FloatQ> {
		let v = Float::with_val(FLOAT_PRECISION, f);
		return Some(FloatQ{ val: v });
	}

	fn from_string(s: &str) -> Option<FloatQ> {
		let v = Float::parse(s);
		let v = match v {
			Ok(x) => x,
			Err(_) => return None
		};

		return Some(
			FloatQ{ val:
				Float::with_val(FLOAT_PRECISION, v)
			}
		);
	}
}



impl Add for FloatQ where {
	type Output = Self;

	fn add(self, other: Self) -> Self::Output {
		Self { val: self.val + other.val}
	}
}

impl AddAssign for FloatQ where {
	fn add_assign(&mut self, other: Self) {
		self.val += other.val;
	}
}

impl Sub for FloatQ {
	type Output = Self;

	fn sub(self, other: Self) -> Self::Output {
		Self {val: self.val - other.val}
	}
}

impl SubAssign for FloatQ where {
	fn sub_assign(&mut self, other: Self) {
		self.val -= other.val;
	}
}

impl Mul for FloatQ {
	type Output = Self;

	fn mul(self, other: Self) -> Self::Output {
		Self {val: self.val * other.val}
	}
}

impl MulAssign for FloatQ where {
	fn mul_assign(&mut self, other: Self) {
		self.val *= other.val;
	}
}

impl Div for FloatQ {
	type Output = Self;

	fn div(self, other: Self) -> Self::Output {
		Self {val: self.val / other.val}
	}
}

impl DivAssign for FloatQ where {
	fn div_assign(&mut self, other: Self) {
		self.val /= other.val;
	}
}

impl Neg for FloatQ where {
	type Output = Self;

	fn neg(self) -> Self::Output {
		Self {val: -self.val}
	}
}

impl Rem<FloatQ> for FloatQ {
	type Output = Self;

	fn rem(self, modulus: FloatQ) -> Self::Output {
		if {
			(!self.fract().is_zero()) ||
			(!modulus.fract().is_zero())
		} { panic!() }

		FloatQ{val : self.val.fract() % modulus.val.fract()}
	}
}

impl PartialEq for FloatQ {
	fn eq(&self, other: &Self) -> bool {
		self.val == other.val
	}
}

impl PartialOrd for FloatQ {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		self.val.partial_cmp(&other.val)
	}
}