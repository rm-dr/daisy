use std::ops::{
	Add, Sub, Mul, Div,
	Neg, Rem,

	AddAssign, SubAssign,
	MulAssign, DivAssign
};

use std::cmp::Ordering;
use super::ScalarBase;
use super::PRINT_LEN;

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


		let neg = if neg {"-"} else {""};

		if (p.abs() as usize) < PRINT_LEN {

			if p >= 0 {
				let q = p as usize;

				// Add zero padding
				let t;
				if s.len() < (q + 1) {
					t = format!("{s}{}", "0".repeat(q + 1 - s.len()));
				} else { t = s.to_string() }

				// Section before decimal point
				let first = &t[0..q+1];

				// The rest of the number, including a decimal point
				let mut rest: String;
				if first.len() == t.len() {
					rest = String::from("");
				} else {
					rest = format!(".{}", &t[q+1..]);
				}

				// Limit length of decimal portion
				if rest.len() > PRINT_LEN {
					rest = String::from(&rest[0..PRINT_LEN]);
				}

				return format!("{neg}{first}{rest}");
			} else {
				let q = p.abs() as usize;

				let t = format!("{neg}0.{}{s}", "0".repeat(q-1));

				return if t.len() > PRINT_LEN { String::from(&t[0..PRINT_LEN]) } else {t};
			}

		// Print full scientific notation
		} else {
			// First (non-zero) digit of our number
			let first = &s[0..1];

			// The rest of the number, including a decimal point
			let mut rest: String;
			if first.len() == s.len() {
				rest = String::from("");
			} else {
				rest = format!(".{}", &s[1..]);
			}

			// Limit length of decimal portion
			if rest.len() > 5 {
				rest = String::from(&rest[0..PRINT_LEN]);
			}

			return format!("{neg}{first}{rest}e{p}");
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