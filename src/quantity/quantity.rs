use std::ops::{
	Add, Sub, Mul, Div,
	Neg, Rem,

	AddAssign, SubAssign,
	MulAssign, DivAssign
};
use std::cmp::Ordering;

use crate::quantity::rationalq::RationalQ;
use crate::quantity::floatq::FloatQ;


use crate::quantity::QuantBase;
use crate::quantity::RationalBase;
use crate::quantity::FloatBase;

#[derive(Debug)]
#[derive(Clone)]
pub enum Quantity {
	Rational{ v: RationalQ },
	Float{ v: FloatQ }
}


impl Quantity {

	pub fn new_rational(top: i64, bottom: i64) -> Quantity {
		return Quantity::Rational {
			v: RationalQ::from_frac(top, bottom)
		}
	}

	pub fn new_float(v: f64) -> Quantity {
		return Quantity::Float {
			v: FloatQ::from_f64(v).unwrap()
		}
	}

	pub fn new_rational_from_string(s: &str) -> Option<Quantity> {
		let r = RationalQ::from_string(s);
		if r.is_none() { return None; }
		return Some(Quantity::Rational{v: r.unwrap()})
	}

	pub fn new_float_from_string(s: &str) -> Option<Quantity> {
		let v = FloatQ::from_string(s);
		if v.is_none() { return None; }
		return Some(Quantity::Float{v: v.unwrap()})
	}

	pub fn float_from_rat(r: &Quantity) -> Quantity {
		match &r {
			Quantity::Float { .. } => r.clone(),
			Quantity::Rational { v } => Quantity::Float { v:
				FloatQ::from(v.val.numer()).unwrap() /
				FloatQ::from(v.val.denom()).unwrap()
			}
		}
	}

	pub fn is_nan(&self) -> bool {
		match self {
			Quantity::Float { v } => {v.val.is_nan()},
			Quantity::Rational { .. } => {panic!()}
		}
	}
}



impl ToString for Quantity {
	fn to_string(&self) -> String {
		match self {
			Quantity::Rational{v} => v.to_string(),
			Quantity::Float{v} => v.to_string(),
		}
	}
}


macro_rules! quant_foward {
	( $x:ident ) => {
		fn $x(&self) -> Quantity {
			match self {
				Quantity::Rational{v} => v.$x(),
				Quantity::Float{v} => v.$x(),
			}
		}
	}
}

impl QuantBase for Quantity {

	fn is_zero(&self) -> bool {
		match self {
			Quantity::Rational{v} => v.is_zero(),
			Quantity::Float{v} => v.is_zero(),
		}
	}

	fn is_negative(&self) -> bool {
		match self {
			Quantity::Rational{v} => v.is_negative(),
			Quantity::Float{v} => v.is_negative(),
		}
	}

	fn is_positive(&self) -> bool {
		match self {
			Quantity::Rational{v} => v.is_positive(),
			Quantity::Float{v} => v.is_positive(),
		}
	}

	quant_foward!(fract);
	quant_foward!(abs);
	quant_foward!(floor);
	quant_foward!(ceil);
	quant_foward!(round);
	quant_foward!(sin);
	quant_foward!(cos);
	quant_foward!(tan);
	quant_foward!(asin);
	quant_foward!(acos);
	quant_foward!(atan);
	quant_foward!(sinh);
	quant_foward!(cosh);
	quant_foward!(tanh);
	quant_foward!(asinh);
	quant_foward!(acosh);
	quant_foward!(atanh);
	quant_foward!(exp);
	quant_foward!(ln);
	quant_foward!(log10);
	quant_foward!(log2);

	fn log(&self, base: Quantity) -> Quantity {
		match self {
			Quantity::Rational{v} => v.log(base),
			Quantity::Float{v} => v.log(base),
		}
	}
	fn pow(&self, base: Quantity) -> Quantity {
		match self {
			Quantity::Rational{v} => v.pow(base),
			Quantity::Float{v} => v.pow(base),
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
		match (&self, &other) {
			(Quantity::Float{v:a}, Quantity::Float{v:b}) => {Quantity::Float{ v: a.clone()+b.clone() }},
			(Quantity::Float{ .. }, Quantity::Rational{ .. }) => {self + Quantity::float_from_rat(&other)},
			(Quantity::Rational{ .. }, Quantity::Float{ .. }) => {Quantity::float_from_rat(&self) + other},
			(Quantity::Rational{v:a}, Quantity::Rational{v:b}) => {Quantity::Rational{ v: a.clone()+b.clone() }},
		}
	}
}

impl AddAssign for Quantity where {
	fn add_assign(&mut self, other: Self) {
		match (&mut *self, &other) {
			(Quantity::Float{v: a}, Quantity::Float{v: ref b}) => {*a += b.clone()},
			(Quantity::Float{ .. }, Quantity::Rational{ .. }) => {*self += Quantity::float_from_rat(&other)},
			(Quantity::Rational{ .. }, Quantity::Float{ .. }) => {*self = Quantity::float_from_rat(self) + other },
			(Quantity::Rational{v:a}, Quantity::Rational{v:b}) => {*a += b.clone()},
		}
	}
}

impl Sub for Quantity {
	type Output = Self;

	fn sub(self, other: Self) -> Self::Output {
		match (&self, &other) {
			(Quantity::Float{v:a}, Quantity::Float{v:b}) => {Quantity::Float{ v: a.clone()-b.clone() }},
			(Quantity::Float{ .. }, Quantity::Rational{ .. }) => {self - Quantity::float_from_rat(&other)},
			(Quantity::Rational{ .. }, Quantity::Float{ .. }) => {Quantity::float_from_rat(&self) - other},
			(Quantity::Rational{v:a}, Quantity::Rational{v:b}) => {Quantity::Rational{ v: a.clone()-b.clone() }},
		}
	}
}

impl SubAssign for Quantity where {
	fn sub_assign(&mut self, other: Self) {
		match (&mut *self, &other) {
			(Quantity::Float{v: a}, Quantity::Float{v: ref b}) => {*a -= b.clone()},
			(Quantity::Float{ .. }, Quantity::Rational{ .. }) => {*self -= Quantity::float_from_rat(&other)},
			(Quantity::Rational{ .. }, Quantity::Float{ .. }) => {*self = Quantity::float_from_rat(self) - other },
			(Quantity::Rational{v:a}, Quantity::Rational{v:b}) => {*a -= b.clone()},
		}
	}
}

impl Mul for Quantity {
	type Output = Self;

	fn mul(self, other: Self) -> Self::Output {
		match (&self, &other) {
			(Quantity::Float{v:a}, Quantity::Float{v:b}) => {Quantity::Float{ v: a.clone()*b.clone() }},
			(Quantity::Float{ .. }, Quantity::Rational{ .. }) => {self * Quantity::float_from_rat(&other)},
			(Quantity::Rational{ .. }, Quantity::Float{ .. }) => {Quantity::float_from_rat(&self) * self},
			(Quantity::Rational{v:a}, Quantity::Rational{v:b}) => {Quantity::Rational{ v: a.clone()*b.clone() }},
		}
	}
}

impl MulAssign for Quantity where {
	fn mul_assign(&mut self, other: Self) {
		match (&mut *self, &other) {
			(Quantity::Float{v: a}, Quantity::Float{v:b}) => {*a *= b.clone()},
			(Quantity::Float{ .. }, Quantity::Rational{ .. }) => {*self *= Quantity::float_from_rat(&other)},
			(Quantity::Rational{ .. }, Quantity::Float{ .. }) => {*self = Quantity::float_from_rat(self) * other },
			(Quantity::Rational{v:a}, Quantity::Rational{v:b}) => {*a *= b.clone()},
		}
	}
}

impl Div for Quantity {
	type Output = Self;

	fn div(self, other: Self) -> Self::Output {
		match (&self, &other) {
			(Quantity::Float{v:a}, Quantity::Float{v:b}) => {Quantity::Float{ v: a.clone()/b.clone() }},
			(Quantity::Float{ .. }, Quantity::Rational{ .. }) => {self / Quantity::float_from_rat(&other)},
			(Quantity::Rational{ .. }, Quantity::Float{ .. }) => {Quantity::float_from_rat(&self) / other},
			(Quantity::Rational{v:a}, Quantity::Rational{v:b}) => {Quantity::Rational{ v: a.clone()/b.clone() }},
		}
	}
}

impl DivAssign for Quantity where {
	fn div_assign(&mut self, other: Self) {
		match (&mut *self, &other) {
			(Quantity::Float{v: a}, Quantity::Float{v: ref b}) => {*a /= b.clone()},
			(Quantity::Float{ .. }, Quantity::Rational{ .. }) => {*self /= Quantity::float_from_rat(&other)},
			(Quantity::Rational{ .. }, Quantity::Float{ .. }) => {*self = Quantity::float_from_rat(self) / other },
			(Quantity::Rational{v:a}, Quantity::Rational{v:b}) => {*a /= b.clone()},
		}
	}
}

impl Rem<Quantity> for Quantity {
	type Output = Self;

	fn rem(self, other: Quantity) -> Self::Output {
		match (&self, &other) {
			(Quantity::Float{v:a}, Quantity::Float{v:b}) => {Quantity::Float{ v: a.clone()%b.clone() }},
			(Quantity::Float{ .. }, Quantity::Rational{ .. }) => {self % Quantity::float_from_rat(&other)},
			(Quantity::Rational{ .. }, Quantity::Float{ .. }) => {Quantity::float_from_rat(&self) % other},
			(Quantity::Rational{v:a}, Quantity::Rational{v:b}) => {Quantity::Rational { v: a.clone()%b.clone() }},
		}
	}
}

impl PartialEq for Quantity {
	fn eq(&self, other: &Self) -> bool {
		match (self, other) {
			(Quantity::Float{v:a}, Quantity::Float{v:b}) => {a == b},
			(Quantity::Float{ .. }, Quantity::Rational{ .. }) => {*self == Quantity::float_from_rat(other)},
			(Quantity::Rational{ .. }, Quantity::Float{ .. }) => {Quantity::float_from_rat(self) == *other},
			(Quantity::Rational{v:a}, Quantity::Rational{v:b}) => {a == b},
		}
	}
}

impl PartialOrd for Quantity {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		match (self, other) {
			(Quantity::Float{v:a}, Quantity::Float{v:b}) => {a.partial_cmp(b)},
			(Quantity::Float{ .. }, Quantity::Rational{ .. }) => {(*self).partial_cmp(&Quantity::float_from_rat(other))},
			(Quantity::Rational{ .. }, Quantity::Float{ .. }) => {Quantity::float_from_rat(self).partial_cmp(other)},
			(Quantity::Rational{v:a}, Quantity::Rational{v:b}) => {a.partial_cmp(b)},
		}
	}
}