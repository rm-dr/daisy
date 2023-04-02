use rug::Float;
use rug::ops::Pow;
use rug::Rational;
use rug::Integer;

use std::ops::{
	Add, Sub, Mul, Div,
	Neg, Rem,

	AddAssign, SubAssign,
	MulAssign, DivAssign
};

use std::cmp::Ordering;


use crate::quantity::Quantity;
use crate::quantity::FLOAT_PRECISION;

macro_rules! rational {
	( $x:expr ) => {
		Quantity::Rational { v: RationalQ {
			val : $x
		}}
	};
}

macro_rules! float {
	( $x:expr ) => {
		Quantity::Float { v: $x }
	};
}

#[derive(Debug)]
#[derive(Clone)]
pub struct RationalQ where {
	pub val: Rational
}

impl ToString for RationalQ {
	fn to_string(&self) -> String {
		self.to_float().to_string()
	}
}

impl RationalQ {
	pub fn new(top: i64, bot: i64) -> RationalQ {
		return RationalQ {
			val: Rational::from((top, bot))
		}
	}

	pub fn is_zero(&self) -> bool{
		return self.val == Rational::from((0,1));
	}
	pub fn fract(&self) -> Quantity {
		rational!(self.val.clone().fract_floor(Integer::new()).0)
	}

	pub fn from_f64(f: f64) -> Option<RationalQ> {
		let v = Rational::from_f64(f);
		if v.is_none() { return None }
		return Some(RationalQ{ val: v.unwrap() });
	}

	pub fn from_string(s: &str) -> Option<RationalQ> {
		let v = Rational::from_str_radix(s, 10);
		let v = match v {
			Ok(x) => x,
			Err(_) => return None
		};
		return Some(RationalQ{ val: v });
	}

	pub fn to_float(&self) -> Float {
		Float::with_val(FLOAT_PRECISION, self.val.numer()) /
		Float::with_val(FLOAT_PRECISION, self.val.denom())
	}

	pub fn to_string_radix(&self, radix: i32, num_digits: Option<usize>) -> String {
		self.to_float().to_string_radix(radix, num_digits)
	}

	pub fn to_sign_string_exp(&self, radix: i32, num_digits: Option<usize>) -> (bool, String, Option<i32>) {
		self.to_float().to_sign_string_exp(radix, num_digits)
	}


	pub fn abs(&self) -> Quantity {rational!(self.val.clone().abs())}
	pub fn floor(&self) -> Quantity {rational!(self.val.clone().floor())}
	pub fn ceil(&self) -> Quantity {rational!(self.val.clone().ceil())}
	pub fn round(&self) -> Quantity {rational!(self.val.clone().round())}

	pub fn sin(&self) -> Quantity {float!(self.to_float().sin())}
	pub fn cos(&self) -> Quantity {float!(self.to_float().cos())}
	pub fn tan(&self) -> Quantity {float!(self.to_float().tan())}
	pub fn asin(&self) -> Quantity {float!(self.to_float().asin())}
	pub fn acos(&self) -> Quantity {float!(self.to_float().acos())}
	pub fn atan(&self) -> Quantity {float!(self.to_float().atan())}

	pub fn sinh(&self) -> Quantity {float!(self.to_float().sinh())}
	pub fn cosh(&self) -> Quantity {float!(self.to_float().cosh())}
	pub fn tanh(&self) -> Quantity {float!(self.to_float().tanh())}
	pub fn asinh(&self) -> Quantity {float!(self.to_float().asinh())}
	pub fn acosh(&self) -> Quantity {float!(self.to_float().acosh())}
	pub fn atanh(&self) -> Quantity {float!(self.to_float().atanh())}

	pub fn ln(&self) -> Quantity {float!(self.to_float().ln())}
	pub fn log10(&self) -> Quantity {float!(self.to_float().log10())}
	pub fn log2(&self) -> Quantity {float!(self.to_float().log2())}

	pub fn log(&self, base: Quantity) -> Quantity {
		float!(self.to_float().log10() / base.to_float().log10())
	}
	


	pub fn pow(&self, exp: Quantity) -> Quantity {
		float!(self.to_float().pow(exp.to_float()))
	}
}

impl Add for RationalQ where {
	type Output = Self;

	fn add(self, other: Self) -> Self::Output {
		Self {
			val: self.val + other.val
		}
	}
}

impl AddAssign for RationalQ where {
	fn add_assign(&mut self, other: Self) {
		self.val += other.val;
	}
}

impl Sub for RationalQ {
	type Output = Self;

	fn sub(self, other: Self) -> Self::Output {
		Self {
			val: self.val - other.val
		}
	}
}

impl SubAssign for RationalQ where {
	fn sub_assign(&mut self, other: Self) {
		self.val -= other.val;
	}
}

impl Mul for RationalQ {
	type Output = Self;

	fn mul(self, other: Self) -> Self::Output {
		Self {
			val: self.val * other.val
		}
	}
}

impl MulAssign for RationalQ where {
	fn mul_assign(&mut self, other: Self) {
		self.val *= other.val;
	}
}

impl Div for RationalQ {
	type Output = Self;

	fn div(self, other: Self) -> Self::Output {
		Self {
			val: self.val / other.val
		}
	}
}

impl DivAssign for RationalQ where {
	fn div_assign(&mut self, other: Self) {
		self.val /= other.val;
	}
}

impl Neg for RationalQ where {
	type Output = Self;

	fn neg(self) -> Self::Output {
		Self {
			val: -self.val
		}
	}
}

impl Rem<RationalQ> for RationalQ {
	type Output = Self;

	fn rem(self, modulus: RationalQ) -> Self::Output {
		if {
			*self.val.denom() != 1 ||
			*modulus.val.denom() != 1
		} { panic!() }

		RationalQ{
			val : Rational::from((
				self.val.numer() % modulus.val.numer(),
				1
			))
		}
	}
}

impl PartialEq for RationalQ {
	fn eq(&self, other: &Self) -> bool {
		self.val == other.val
	}
}

impl PartialOrd for RationalQ {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		self.val.partial_cmp(&other.val)
	}
}