use bigdecimal::BigDecimal;
use bigdecimal::Zero;
use bigdecimal::RoundingMode;
use std::str::FromStr;

use std::ops::{
	Add, Sub, Mul, Div,
	Neg, Rem,

	AddAssign, SubAssign,
	MulAssign, DivAssign
};

use std::cmp::Ordering;

use super::ScalarBase;
use super::dec_to_sci;


#[derive(Debug)]
#[derive(Clone)]
pub struct FloatBase where {
	pub val: BigDecimal
}

impl FloatBase {
	pub fn new(s: &str) -> FloatBase {
		return FloatBase {
			val: s.parse().unwrap()
		};
	}
}


impl ToString for FloatBase {
	fn to_string(&self) -> String {

		if self.val.is_nan() {
			return "NaN".to_string();
		} else if self.val.is_inf_neg() {
			return "-Inf".to_string();
		} else if self.val.is_inf_pos() {
			return "+Inf".to_string();
		}


		// Already in scientific notation,we just need to trim significant digits.
		let mut _a = self.val.round(32, astro_float::RoundingMode::Up).to_string();
		let mut _b = _a.split('e');

		let mut s = String::from(_b.next().unwrap()); // Decimal
		let p: i64 = _b.next().unwrap().parse().unwrap(); // Exponent

		// Remove negative sign from string
		let neg = s.starts_with("-");
		if neg { s = String::from(&s[1..]); }

		// We no longer need a decimal point in our string.
		// also, trim off leading zeros and adjust power.
		let mut s: &str = &s.replace(".", "");
		s = &s[0..];
		s = s.trim_end_matches('0');
		s = s.trim_start_matches('0');

		return dec_to_sci(neg, s.to_string(), p);
	}
}



impl ScalarBase for FloatBase {

	fn from_string(s: &str) -> Option<FloatBase> {
		let v = BigDecimal::from_str(s);
		let v = match v {
			Ok(x) => x,
			Err(_) => return None
		};

		return Some(FloatBase{ val: v });
	}

	//foward!(fract);

	fn is_zero(&self) -> bool {self.val.is_zero()}
	fn is_one(&self) -> bool {self.val == BigDecimal::from_str("1").unwrap()}
	fn is_negative(&self) -> bool { self.val.sign() == num::bigint::Sign::Minus }
	fn is_positive(&self) -> bool { self.val.sign() == num::bigint::Sign::Plus }

	fn is_int(&self) -> bool { self.val.is_integer() }

	fn abs(&self) -> Option<FloatBase> { Some(FloatBase{ val: self.val.abs() }) }
	fn round(&self) -> Option<FloatBase> { Some(FloatBase{ val: self.val.round(0) }) }

	fn floor(&self) -> Option<FloatBase> {
		let (_, scale) = self.val.as_bigint_and_exponent();
		Some(FloatBase{ val: self.val.with_scale_round(scale, RoundingMode::Down) })
	}

	fn ceil(&self) -> Option<FloatBase> {
		let (_, scale) = self.val.as_bigint_and_exponent();
		Some(FloatBase{ val: self.val.with_scale_round(scale, RoundingMode::Up) })
	}

	fn fract(&self) -> Option<FloatBase> { Some(self.clone() - self.floor().unwrap()) }


	fn sin(&self) -> Option<FloatBase> {
		let c0: BigDecimal = "1.276278962".parse().unwrap();
		let c1: BigDecimal = "-.285261569".parse().unwrap();
		let c2: BigDecimal = "0.009118016".parse().unwrap();
		let c3: BigDecimal = "-.000136587".parse().unwrap();
		let c4: BigDecimal = "0.000001185".parse().unwrap();
		let c5: BigDecimal = "-.000000007".parse().unwrap();
		

		// z should be between -0.25 to 0.25 (percent of a full circle)
		let z: BigDecimal = self.val.clone() / 360f64;
		let w = BigDecimal::from(4) * z;
		let x: BigDecimal = 2 * w.clone() * w.clone() - 1;

		let p = (
			c0 * 1 +
			c1 * x.clone() +
			c2 * (2 * x.clone()*x.clone() - 1) +
			c3 * (4 * x.clone()*x.clone()*x.clone() - 3 * x.clone()) +
			c4 * (8 * x.clone()*x.clone()*x.clone()*x.clone() - 8 * x.clone()*x.clone() + 1) +
			c5 * (16 * x.clone()*x.clone()*x.clone()*x.clone()*x.clone() - 20 * x.clone()*x.clone()*x.clone() + 5 * x.clone())
		) * w;

		return Some(FloatBase{ val: p })
	}
	fn cos(&self) -> Option<FloatBase> { Some(FloatBase{ val: "1".parse().unwrap() }) }
	fn tan(&self) -> Option<FloatBase> { Some(FloatBase{ val: "1".parse().unwrap() }) }
	fn csc(&self) -> Option<FloatBase> { Some(FloatBase{ val: "1".parse().unwrap() }) }
	fn sec(&self) -> Option<FloatBase> { Some(FloatBase{ val: "1".parse().unwrap() }) }
	fn cot(&self) -> Option<FloatBase> { Some(FloatBase{ val: "1".parse().unwrap() }) }
	fn asin(&self) -> Option<FloatBase> { Some(FloatBase{ val: "1".parse().unwrap() }) }
	fn acos(&self) -> Option<FloatBase> { Some(FloatBase{ val: "1".parse().unwrap() }) }
	fn atan(&self) -> Option<FloatBase> { Some(FloatBase{ val: "1".parse().unwrap() }) }
	fn sinh(&self) -> Option<FloatBase> { Some(FloatBase{ val: "1".parse().unwrap() }) }
	fn cosh(&self) -> Option<FloatBase> { Some(FloatBase{ val: "1".parse().unwrap() }) }
	fn tanh(&self) -> Option<FloatBase> { Some(FloatBase{ val: "1".parse().unwrap() }) }
	fn csch(&self) -> Option<FloatBase> { Some(FloatBase{ val: "1".parse().unwrap() }) }
	fn sech(&self) -> Option<FloatBase> { Some(FloatBase{ val: "1".parse().unwrap() }) }
	fn coth(&self) -> Option<FloatBase> { Some(FloatBase{ val: "1".parse().unwrap() }) }
	fn asinh(&self) -> Option<FloatBase> { Some(FloatBase{ val: "1".parse().unwrap() }) }
	fn acosh(&self) -> Option<FloatBase> { Some(FloatBase{ val: "1".parse().unwrap() }) }
	fn atanh(&self) -> Option<FloatBase> { Some(FloatBase{ val: "1".parse().unwrap() }) }
	fn exp(&self) -> Option<FloatBase> { Some(FloatBase{ val: "1".parse().unwrap() }) }
	fn ln(&self) -> Option<FloatBase> { Some(FloatBase{ val: "1".parse().unwrap() }) }
	fn log10(&self) -> Option<FloatBase> { Some(FloatBase{ val: "1".parse().unwrap() }) }
	fn log2(&self) -> Option<FloatBase> { Some(FloatBase{ val: "1".parse().unwrap() }) }


	fn log(&self, _base: FloatBase) -> Option<FloatBase> {
		Some(FloatBase{ val: "1".parse().unwrap() })
	}

	fn pow(&self, _base: FloatBase) -> Option<FloatBase> {
		Some(FloatBase{ val: "1".parse().unwrap() })
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
		self.val = self.val.clone() / other.val;
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
			(!self.is_int()) ||
			(!modulus.is_int())
		} { panic!() }

		FloatBase{val : self.val.round(0) % modulus.val.round(0)}
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