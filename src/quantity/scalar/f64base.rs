use std::ops::{
	Add, Sub, Mul, Div,
	Neg, Rem,

	AddAssign, SubAssign,
	MulAssign, DivAssign
};

use std::cmp::Ordering;
use super::ScalarBase;
use super::SHOW_SIG;
use super::MAX_LEN;

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
	fn to_string(&self) -> String {
		// Decimal, looks like xxx.xxx.
		// May start with a zero.
		let mut s = self.val.to_string();

		// Remove negative sign from string
		let neg = s.starts_with("-");
		if neg { s = String::from(&s[1..]); }
		
		// Power of ten
		let mut p: i32 = {
			if let Some(x) = s.find(".") {
				x as i32
			} else {
				s.len() as i32
			}
		};
		p -= 1;

		// We no longer need a decimal point in our string.
		// also, trim off leading zeros and adjust power.
		let mut s: &str = &s.replace(".", "");
		s = &s[0..];
		s = s.trim_end_matches('0');
		while s.starts_with('0') {
			s = &s[1..];
			p -= 1;
		}


		// Pick significant digits and round
		let mut s = String::from(s);
		if s.len() > SHOW_SIG {
			let round;
			if s.len() != SHOW_SIG + 1 {
				round = s[SHOW_SIG..SHOW_SIG+1].parse().unwrap();
			} else { round = 0; }

			s = String::from(&s[0..SHOW_SIG]);

			if round >= 5 {
				let new = s[s.len()-1..s.len()].parse::<u8>().unwrap() + 1u8;
				if new != 10 {
					s = format!("{}{new}", &s[0..s.len()-1]);
				}
			}
		}

		s = format!("{s}{}", "0".repeat(SHOW_SIG - s.len()));
		// at this point, s is guaranteed to have exactly SHOW_SIG digits.

		let neg = if neg {"-"} else {""};

		if (p.abs() as usize) < MAX_LEN {
			if p >= 0 {
				let q = p as usize;

				let first = &s[0..q+1];
				let mut rest = &s[q+1..];
				rest = rest.trim_end_matches('0');
				if rest == "" {
					return format!("{neg}{first}");
				} else {
					return format!("{neg}{first}.{rest}");
				}
			} else {
				let q = p.abs() as usize;
				let t = format!("0.{}{s}", "0".repeat(q-1));
				return format!("{neg}{}", t.trim_end_matches('0'));
			}

		// Print full scientific notation
		} else {
			let first = &s[0..1];
			let mut rest = &s[1..];
			rest = rest.trim_end_matches('0');
			if rest == "" {
				return format!("{neg}{first}e{p}");
			} else {
				return format!("{neg}{first}.{rest}e{p}");
			}
		}
	}
}


impl ScalarBase for F64Base {
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
	fn is_int(&self) -> bool { self.val.floor() == self.val }

	foward!(abs);
	foward!(floor);
	foward!(ceil);
	foward!(round);

	foward!(sin);
	foward!(cos);
	foward!(tan);

	fn csc(&self) -> Option<F64Base> { Some(F64Base{ val: 1f64/self.val.sin() }) }
	fn sec(&self) -> Option<F64Base> { Some(F64Base{ val: 1f64/self.val.cos() }) }
	fn cot(&self) -> Option<F64Base> { Some(F64Base{ val: 1f64/self.val.tan() }) }

	foward!(asin);
	foward!(acos);
	foward!(atan);

	foward!(sinh);
	foward!(cosh);
	foward!(tanh);

	fn csch(&self) -> Option<F64Base> { Some(F64Base{ val: 1f64/self.val.sinh() }) }
	fn sech(&self) -> Option<F64Base> { Some(F64Base{ val: 1f64/self.val.cosh() }) }
	fn coth(&self) -> Option<F64Base> { Some(F64Base{ val: 1f64/self.val.tanh() }) }

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
			(!self.is_int()) ||
			(!modulus.is_int())
		} { panic!() }

		F64Base{val : self.val.round() % modulus.val.round()}
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