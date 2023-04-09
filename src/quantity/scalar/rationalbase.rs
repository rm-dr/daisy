use rug::Rational;
use rug::Integer;

use std::ops::{
	Add, Sub, Mul, Div,
	Neg, Rem,

	AddAssign, SubAssign,
	MulAssign, DivAssign
};

use std::cmp::Ordering;
use super::ScalarBase;


macro_rules! cant_do {
	( $x:ident ) => {
		fn $x(&self) -> Option<RationalBase> { None }
	}
}

#[derive(Debug)]
#[derive(Clone)]
pub struct RationalBase where {
	pub val: Rational
}

/*
fn to_string_radix(&self, radix: i32, num_digits: Option<usize>) -> String {
	self.to_float().to_string_radix(radix, num_digits)
}

fn to_sign_string_exp(&self, radix: i32, num_digits: Option<usize>) -> (bool, String, Option<i32>) {
	self.to_float().to_sign_string_exp(radix, num_digits)
}
*/

impl ToString for RationalBase{
	fn to_string(&self) -> String {
		return self.val.to_string();
	}
}

impl ScalarBase for RationalBase {


	fn from_f64(f: f64) -> Option<RationalBase> {
		let v = Rational::from_f64(f);
		if v.is_none() { return None }
		return Some(RationalBase{ val: v.unwrap() });
	}

	fn from_string(s: &str) -> Option<RationalBase> {
		// Scientific notation
		let mut sci = s.split("e");
		let num = sci.next().unwrap();
		let exp = sci.next();

		let exp = if exp.is_some() {
			let r = exp.unwrap().parse::<isize>();
			match r {
				Ok(x) => x,
				Err(_) => return None
			}
		} else {0isize};

		// Split integer and decimal parts
		let mut dec = num.split(".");
		let a = dec.next().unwrap();
		let b = dec.next();
		let b = if b.is_some() {b.unwrap()} else {""};

		// Error conditions
		if {
			dec.next().is_some() || // We should have at most one `.`
			sci.next().is_some() || // We should have at most one `e`
			a.len() == 0 // We need something in the numerator
		} { return None; }

		let s: String;
		if exp < 0 {
			let exp: usize = (-exp).try_into().unwrap();
			s = format!("{a}{b}/1{}", "0".repeat(b.len() + exp));
		} else if exp > 0 {
			let exp: usize = exp.try_into().unwrap();
			s = format!(
				"{a}{b}{}/1{}",
				"0".repeat(exp),
				"0".repeat(b.len())
			);
		} else { // exp == 0
			s = format!("{a}{b}/1{}", "0".repeat(b.len()));
		};


		// From fraction string
		let r = Rational::from_str_radix(&s, 10);
		let r = match r {
			Ok(x) => x,
			Err(_) => return None
		};

		return Some(RationalBase{val: r});

	}


	fn fract(&self) -> Option<RationalBase> {
		Some(RationalBase{val: self.val.clone().fract_floor(Integer::new()).0})
	}

	fn is_zero(&self) -> bool {self.val == Rational::from((0,1))}
	fn is_negative(&self) -> bool { self.val.clone().signum() == -1 }
	fn is_positive(&self) -> bool { self.val.clone().signum() == 1 }

	fn abs(&self) -> Option<RationalBase> {Some(RationalBase{val: self.val.clone().abs()})}
	fn floor(&self) -> Option<RationalBase> {Some(RationalBase{val: self.val.clone().floor()})}
	fn ceil(&self) -> Option<RationalBase> {Some(RationalBase{val: self.val.clone().ceil()})}
	fn round(&self) -> Option<RationalBase> {Some(RationalBase{val: self.val.clone().round()})}

	cant_do!(sin);
	cant_do!(cos);
	cant_do!(tan);
	cant_do!(asin);
	cant_do!(acos);
	cant_do!(atan);

	cant_do!(sinh);
	cant_do!(cosh);
	cant_do!(tanh);
	cant_do!(asinh);
	cant_do!(acosh);
	cant_do!(atanh);

	cant_do!(exp);
	cant_do!(ln);
	cant_do!(log10);
	cant_do!(log2);

	fn log(&self, _base: RationalBase) -> Option<RationalBase> { None }
	fn pow(&self, _base: RationalBase) -> Option<RationalBase> { None }

}


impl Add for RationalBase where {
	type Output = Self;

	fn add(self, other: Self) -> Self::Output {
		Self {
			val: self.val + other.val
		}
	}
}

impl AddAssign for RationalBase where {
	fn add_assign(&mut self, other: Self) {
		self.val += other.val;
	}
}

impl Sub for RationalBase {
	type Output = Self;

	fn sub(self, other: Self) -> Self::Output {
		Self {
			val: self.val - other.val
		}
	}
}

impl SubAssign for RationalBase where {
	fn sub_assign(&mut self, other: Self) {
		self.val -= other.val;
	}
}

impl Mul for RationalBase {
	type Output = Self;

	fn mul(self, other: Self) -> Self::Output {
		Self {
			val: self.val * other.val
		}
	}
}

impl MulAssign for RationalBase where {
	fn mul_assign(&mut self, other: Self) {
		self.val *= other.val;
	}
}

impl Div for RationalBase {
	type Output = Self;

	fn div(self, other: Self) -> Self::Output {
		Self {
			val: self.val / other.val
		}
	}
}

impl DivAssign for RationalBase where {
	fn div_assign(&mut self, other: Self) {
		self.val /= other.val;
	}
}

impl Neg for RationalBase where {
	type Output = Self;

	fn neg(self) -> Self::Output {
		Self {
			val: -self.val
		}
	}
}

impl Rem<RationalBase> for RationalBase {
	type Output = Self;

	fn rem(self, modulus: RationalBase) -> Self::Output {
		if {
			*self.val.denom() != 1 ||
			*modulus.val.denom() != 1
		} { panic!() }

		RationalBase{
			val : Rational::from((
				self.val.numer() % modulus.val.numer(),
				1
			))
		}
	}
}

impl PartialEq for RationalBase {
	fn eq(&self, other: &Self) -> bool {
		self.val == other.val
	}
}

impl PartialOrd for RationalBase {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		self.val.partial_cmp(&other.val)
	}
}