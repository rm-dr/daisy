use rug::Float;
use rug::ops::Pow;

use std::ops::{
	Add, Sub, Mul, Div,
	Neg, Rem,

	AddAssign, SubAssign,
	MulAssign, DivAssign
};
use std::cmp::Ordering;


use crate::quantity::rationalq::RationalQ;
use crate::quantity::FLOAT_PRECISION;
use crate::quantity::PRINT_LEN;

#[derive(Debug)]
#[derive(Clone)]
pub enum Quantity {
	Rational{ v: RationalQ },
	Float{ v: Float }
}

impl ToString for Quantity{
	fn to_string(&self) -> String {
		let (sign, mut string, exp) = match self {
			Quantity::Float { v } => { v.to_sign_string_exp(10, Some(PRINT_LEN)) }
			Quantity::Rational { v } => { v.to_sign_string_exp(10, Some(PRINT_LEN)) }
		};

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


macro_rules! quick_quant_fn {
	( $x:ident ) => {
		pub fn $x(&self) -> Quantity {
			match self {
				Quantity::Float { v } => {Quantity::Float{ v:v.clone().$x()}},
				Quantity::Rational { v } => {v.$x()}
			}
		}
	}
}

impl Quantity {

	pub fn new_float(f: f64) -> Quantity {
		return Quantity::Float {
			v: Float::with_val(FLOAT_PRECISION, f)
		}
	}

	pub fn new_float_from_string(s: &str) -> Quantity {
		let v = Float::parse(s);
		return Quantity::Float {
			v: Float::with_val(FLOAT_PRECISION, v.unwrap())
		}
	}


	pub fn new_rational(top: i64, bottom: i64) -> Quantity {
		return Quantity::Rational {
			v: RationalQ::new(top, bottom)
		}
	}

	pub fn new_rational_from_string(s: &str) -> Option<Quantity> {
		let r = RationalQ::from_string(s);
		if r.is_none() { return None; }
		return Some(Quantity::Rational { v: r.unwrap() });
	}

	pub fn new_rational_from_f64(f: f64) -> 
	Option<Quantity> {
		let r = RationalQ::from_f64(f);

		if r.is_some() {
			return Some(Quantity::Rational {
				v: r.unwrap() 
			});
		} else {
			return None;
		}
	}

	pub fn new_rational_from_float_string(s: &str) -> Option<Quantity> {

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

		return Quantity::new_rational_from_string(&s);
	}

	pub fn to_float(&self) -> Float {
		match self {
			Quantity::Float { v } => {v.clone()},
			Quantity::Rational { v } => {v.to_float()}
		}
	}

	quick_quant_fn!(fract);

	quick_quant_fn!(abs);
	quick_quant_fn!(floor);
	quick_quant_fn!(ceil);
	quick_quant_fn!(round);
	quick_quant_fn!(sin);
	quick_quant_fn!(cos);
	quick_quant_fn!(tan);
	quick_quant_fn!(asin);
	quick_quant_fn!(acos);
	quick_quant_fn!(atan);
	quick_quant_fn!(sinh);
	quick_quant_fn!(cosh);
	quick_quant_fn!(tanh);
	quick_quant_fn!(asinh);
	quick_quant_fn!(acosh);
	quick_quant_fn!(atanh);

	quick_quant_fn!(ln);
	quick_quant_fn!(log10);
	quick_quant_fn!(log2);

	pub fn log(&self, base: Quantity) -> Quantity {
		match (&self, &base) {
			(Quantity::Float{v:a}, Quantity::Float{v:b}) => {Quantity::Float{v: a.clone().log10() / b.clone().log10()}},
			(Quantity::Float{v:a}, Quantity::Rational{v:b}) => {Quantity::Float{v: a.clone().log10() / b.to_float().log10()}},
			(Quantity::Rational{v:a}, _) => {a.log(base)}
		}
	}

	pub fn is_zero(&self) -> bool {
		match self {
			Quantity::Float { v } => {v.is_zero()},
			Quantity::Rational { v } => {v.is_zero()}
		}
	}

	pub fn pow(&self, exp: Quantity) -> Quantity {
		match self {
			Quantity::Float { v } => {Quantity::Float {v: v.pow(exp.to_float())}},
			Quantity::Rational { v } => {v.pow(exp) }
		}
	}

	pub fn is_nan(&self) -> bool {
		match self {
			Quantity::Float { v } => {v.is_nan()},
			Quantity::Rational { .. } => {panic!()}
		}
	}
}

impl Neg for Quantity where {
	type Output = Self;

	fn neg(self) -> Self::Output {
		match self {
			Quantity::Float { v } => {Quantity::Float{ v: -v }},
			Quantity::Rational { v } => {Quantity::Rational { v: -v }},
		}
	}
}

impl Add for Quantity {
	type Output = Self;

	fn add(self, other: Self) -> Self::Output {
		match (self, other) {
			(Quantity::Float{v:a}, Quantity::Float{v:b}) => {Quantity::Float{ v: a+b }},
			(Quantity::Float{v:a}, Quantity::Rational{v:b}) => {Quantity::Float{ v: a+b.to_float() }},
			(Quantity::Rational{v:a}, Quantity::Float{v:b}) => {Quantity::Float{ v: a.to_float()+b }},
			(Quantity::Rational{v:a}, Quantity::Rational{v:b}) => {Quantity::Rational{ v: a+b }},
		}
	}
}

impl AddAssign for Quantity where {
	fn add_assign(&mut self, other: Self) {
		match (&mut *self, other) {
			(Quantity::Float{v: a}, Quantity::Float{v: ref b}) => {*a += b},
			(Quantity::Float{v: a}, Quantity::Rational{v:b}) => {*a += b.to_float()},
			(Quantity::Rational{v:a}, Quantity::Float{v:b}) => {*self = Quantity::Float{ v: a.to_float()+b }},
			(Quantity::Rational{v:a}, Quantity::Rational{v:b}) => {*a += b},
		}
	}
}

impl Sub for Quantity {
	type Output = Self;

	fn sub(self, other: Self) -> Self::Output {
		match (self, other) {
			(Quantity::Float{v:a}, Quantity::Float{v:b}) => {Quantity::Float{ v: a-b }},
			(Quantity::Float{v:a}, Quantity::Rational{v:b}) => {Quantity::Float{ v: a-b.to_float() }},
			(Quantity::Rational{v:a}, Quantity::Float{v:b}) => {Quantity::Float{ v: a.to_float()-b }},
			(Quantity::Rational{v:a}, Quantity::Rational{v:b}) => {Quantity::Rational{ v: a-b }},
		}
	}
}

impl SubAssign for Quantity where {
	fn sub_assign(&mut self, other: Self) {
		match (&mut *self, other) {
			(Quantity::Float{v: a}, Quantity::Float{v: ref b}) => {*a -= b},
			(Quantity::Float{v: a}, Quantity::Rational{v:b}) => {*a -= b.to_float()},
			(Quantity::Rational{v:a}, Quantity::Float{v:b}) => {*self = Quantity::Float{ v: a.to_float()-b }},
			(Quantity::Rational{v:a}, Quantity::Rational{v:b}) => {*a -= b},
		}
	}
}

impl Mul for Quantity {
	type Output = Self;

	fn mul(self, other: Self) -> Self::Output {
		match (self, other) {
			(Quantity::Float{v:a}, Quantity::Float{v:b}) => {Quantity::Float{ v: a*b }},
			(Quantity::Float{v:a}, Quantity::Rational{v:b}) => {Quantity::Float{ v: a*b.to_float() }},
			(Quantity::Rational{v:a}, Quantity::Float{v:b}) => {Quantity::Float{ v: a.to_float()*b }},
			(Quantity::Rational{v:a}, Quantity::Rational{v:b}) => {Quantity::Rational{ v: a*b }},
		}
	}
}

impl MulAssign for Quantity where {
	fn mul_assign(&mut self, other: Self) {
		match (&mut *self, other) {
			(Quantity::Float{v: a}, Quantity::Float{v: ref b}) => {*a *= b},
			(Quantity::Float{v: a}, Quantity::Rational{v:b}) => {*a *= b.to_float()},
			(Quantity::Rational{v:a}, Quantity::Float{v:b}) => {*self = Quantity::Float{ v: a.to_float() * b }},
			(Quantity::Rational{v:a}, Quantity::Rational{v:b}) => {*a *= b},
		}
	}
}


impl Div for Quantity {
	type Output = Self;

	fn div(self, other: Self) -> Self::Output {
		match (self, other) {
			(Quantity::Float{v:a}, Quantity::Float{v:b}) => {Quantity::Float{ v: a/b }},
			(Quantity::Float{v:a}, Quantity::Rational{v:b}) => {Quantity::Float{ v: a/b.to_float() }},
			(Quantity::Rational{v:a}, Quantity::Float{v:b}) => {Quantity::Float{ v: a.to_float()/b }},
			(Quantity::Rational{v:a}, Quantity::Rational{v:b}) => {Quantity::Rational{ v: a/b }},
		}
	}
}

impl DivAssign for Quantity where {
	fn div_assign(&mut self, other: Self) {
		match (&mut *self, other) {
			(Quantity::Float{v: a}, Quantity::Float{v: ref b}) => {*a /= b},
			(Quantity::Float{v: a}, Quantity::Rational{v:b}) => {*a /= b.to_float()},
			(Quantity::Rational{v:a}, Quantity::Float{v:b}) => {*self = Quantity::Float{ v: a.to_float()/b }},
			(Quantity::Rational{v:a}, Quantity::Rational{v:b}) => {*a /= b},
		}
	}
}

impl Rem<Quantity> for Quantity {
	type Output = Self;

	fn rem(self, modulus: Quantity) -> Self::Output {
		match (self, modulus) {
			(Quantity::Float{v:a}, Quantity::Float{v:b}) => {Quantity::Float{ v: a%b }},
			(Quantity::Float{v:a}, Quantity::Rational{v:b}) => {Quantity::Float{ v: a%b.to_float() }},
			(Quantity::Rational{v:a}, Quantity::Float{v:b}) => {Quantity::Float{ v: a.to_float()%b }},
			(Quantity::Rational{v:a}, Quantity::Rational{v:b}) => {Quantity::Rational { v: a%b }},
		}
	}
}

impl PartialEq for Quantity {
	fn eq(&self, other: &Self) -> bool {
		match (self, other) {
			(Quantity::Float{v:a}, Quantity::Float{v:b}) => {a == b},
			(Quantity::Float{v:a}, Quantity::Rational{v:b}) => {*a==b.to_float()},
			(Quantity::Rational{v:a}, Quantity::Float{v:b}) => {a.to_float()==*b},
			(Quantity::Rational{v:a}, Quantity::Rational{v:b}) => {a == b},
		}
	}
}

impl PartialOrd for Quantity {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		match (self, other) {
			(Quantity::Float{v:a}, Quantity::Float{v:b}) => {a.partial_cmp(b)},
			(Quantity::Float{v:a}, Quantity::Rational{v:b}) => {(*a).partial_cmp(&b.to_float())},
			(Quantity::Rational{v:a}, Quantity::Float{v:b}) => {a.to_float().partial_cmp(b)},
			(Quantity::Rational{v:a}, Quantity::Rational{v:b}) => {a.partial_cmp(b)},
		}
	}
}




