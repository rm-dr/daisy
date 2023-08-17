use std::ops::{
	Add, Sub, Mul, Div,
	Neg, Rem,

	AddAssign, SubAssign,
	MulAssign, DivAssign
};
use std::cmp::Ordering;

use super::floatbase::FloatBase as FloatBase;
use super::rationalbase::RationalBase;


pub trait ScalarBase:
	Sized + ToString +
	Add + AddAssign +
	Sub + SubAssign +
	Mul + MulAssign +
	Div + DivAssign +
	Neg + Rem +
	PartialEq + PartialOrd
{
	// Creation
	fn from_f64(f: f64) -> Option<Self>;
	fn from_string(s: &str) -> Option<Self>;

	// Utility
	fn fract(&self) -> Option<Self>;
	fn is_zero(&self) -> bool;
	fn is_one(&self) -> bool;
	fn is_int(&self) -> bool;
	fn is_negative(&self) -> bool;
	fn is_positive(&self) -> bool;

	// Mathematical
	fn exp(&self) -> Option<Self>;
	fn abs(&self) -> Option<Self>;
	fn floor(&self) -> Option<Self>;
	fn ceil(&self) -> Option<Self>;
	fn round(&self) -> Option<Self>;
	fn sin(&self) -> Option<Self>;
	fn cos(&self) -> Option<Self>;
	fn tan(&self) -> Option<Self>;
	fn csc(&self) -> Option<Self>;
	fn sec(&self) -> Option<Self>;
	fn cot(&self) -> Option<Self>;
	fn asin(&self) -> Option<Self>;
	fn acos(&self) -> Option<Self>;
	fn atan(&self) -> Option<Self>;
	fn sinh(&self) -> Option<Self>;
	fn cosh(&self) -> Option<Self>;
	fn tanh(&self) -> Option<Self>;
	fn csch(&self) -> Option<Self>;
	fn sech(&self) -> Option<Self>;
	fn coth(&self) -> Option<Self>;
	fn asinh(&self) -> Option<Self>;
	fn acosh(&self) -> Option<Self>;
	fn atanh(&self) -> Option<Self>;
	fn ln(&self) -> Option<Self>;
	fn log10(&self) -> Option<Self>;
	fn log2(&self) -> Option<Self>;
	fn log(&self, base: Self) -> Option<Self>;
	fn pow(&self, exp: Self) -> Option<Self>;
}




#[derive(Debug)]
#[derive(Clone)]
pub enum Scalar {
	Rational{ v: RationalBase },
	Float{ v: FloatBase }
}


macro_rules! wrap_rational {
	( $x:expr) => { Scalar::Rational{v: $x} }
}

macro_rules! wrap_float {
	( $x:expr) => { Scalar::Float{v: $x} }
}


fn to_float(r: Scalar) -> Scalar {
	match &r {
		Scalar::Float {..} => r,
		Scalar::Rational {v} => wrap_float!(
			FloatBase::from(v.val.numer()).unwrap() /
			FloatBase::from(v.val.denom()).unwrap()
		)
	}
}

impl ToString for Scalar {
	fn to_string(&self) -> String {
		match self {
			Scalar::Rational{..} => to_float(self.clone()).to_string(),
			Scalar::Float{v} => v.to_string()
		}
	}
}

// Creation methods
impl Scalar {
	pub fn new_float(f: f64) -> Option<Self> {
		let v = FloatBase::from_f64(f);
		if v.is_none() { return None; }
		return Some(wrap_float!(v.unwrap()));
	}

	pub fn new_rational(f: f64) -> Option<Self> {
		let r = RationalBase::from_f64(f);
		if r.is_none() { return None; }
		return Some(wrap_rational!(r.unwrap()));
	}

	pub fn new_rational_from_string(s: &str) -> Option<Self> {
		let r = RationalBase::from_string(s);
		if r.is_none() { return None; }
		return Some(wrap_rational!(r.unwrap()));
	}

	pub fn new_rational_from_frac(t: i64, b: i64) -> Option<Self> {
		let r = RationalBase::from_frac(t, b);
		if r.is_none() { return None; }
		return Some(wrap_rational!(r.unwrap()));
	}

	pub fn new_float_from_string(s: &str) -> Option<Self> {
		let v = FloatBase::from_string(s);
		if v.is_none() { return None; }
		return Some(wrap_float!(v.unwrap()))
	}
}


// Forwarded functions
macro_rules! scalar_foward {
	( $x:ident ) => {
		pub fn $x(&self) -> Scalar {
			match self {
				Scalar::Rational{v} => {
					let r = v.$x();
					if r.is_none() {
						let v = to_float(self.clone());
						return v.$x();
					} else {wrap_rational!(r.unwrap())}
				},
				Scalar::Float{v} => {wrap_float!(v.$x().unwrap())},
			}
		}
	}
}

impl Scalar {
	pub fn is_zero(&self) -> bool {
		match self {
			Scalar::Rational{v} => v.is_zero(),
			Scalar::Float{v} => v.is_zero(),
		}
	}

	pub fn is_one(&self) -> bool {
		match self {
			Scalar::Rational{v} => v.is_one(),
			Scalar::Float{v} => v.is_one(),
		}
	}

	pub fn is_negative(&self) -> bool {
		match self {
			Scalar::Rational{v} => v.is_negative(),
			Scalar::Float{v} => v.is_negative(),
		}
	}

	pub fn is_positive(&self) -> bool {
		match self {
			Scalar::Rational{v} => v.is_positive(),
			Scalar::Float{v} => v.is_positive(),
		}
	}

	pub fn is_nan(&self) -> bool {
		match self {
			Scalar::Float {v} => {v.val.is_nan()},
			Scalar::Rational {..} => {false}
		}
	}

	pub fn is_rational(&self) -> bool {
		match self {
			Scalar::Float { .. } => false,
			Scalar::Rational {..} => true
		}
	}

	pub fn is_int(&self) -> bool {
		match self {
			Scalar::Rational{v} => v.is_int(),
			Scalar::Float{v} => v.is_int(),
		}
	}

	scalar_foward!(fract);
	scalar_foward!(abs);
	scalar_foward!(floor);
	scalar_foward!(ceil);
	scalar_foward!(round);
	scalar_foward!(sin);
	scalar_foward!(cos);
	scalar_foward!(tan);
	scalar_foward!(csc);
	scalar_foward!(sec);
	scalar_foward!(cot);
	scalar_foward!(asin);
	scalar_foward!(acos);
	scalar_foward!(atan);
	scalar_foward!(sinh);
	scalar_foward!(cosh);
	scalar_foward!(tanh);
	scalar_foward!(csch);
	scalar_foward!(sech);
	scalar_foward!(coth);
	scalar_foward!(asinh);
	scalar_foward!(acosh);
	scalar_foward!(atanh);
	scalar_foward!(exp);
	scalar_foward!(ln);
	scalar_foward!(log10);
	scalar_foward!(log2);

	pub fn log(&self, base: Scalar) -> Scalar {
		match self {
			Scalar::Rational{..} => { to_float(self.clone()).log(to_float(base)) },
			Scalar::Float{..} => { to_float(self.clone()).log(to_float(base)) },
		}
	}

	pub fn pow(&self, base: Scalar) -> Scalar {
		match self {
			Scalar::Rational{..} => {
				let a = match to_float(self.clone()) {
					Scalar::Rational{..} => panic!(),
					Scalar::Float{v} => v,
				};

				let b = match to_float(base) {
					Scalar::Rational{..} => panic!(),
					Scalar::Float{v} => v,
				};

				wrap_float!(a.pow(b).unwrap())
			},
			Scalar::Float{..} => {
				let a = match to_float(self.clone()) {
					Scalar::Rational{..} => panic!(),
					Scalar::Float{v} => v,
				};

				let b = match to_float(base) {
					Scalar::Rational{..} => panic!(),
					Scalar::Float{v} => v,
				};

				wrap_float!(a.pow(b).unwrap())

			},
		}
	}
}

impl Neg for Scalar where {
	type Output = Self;

	fn neg(self) -> Self::Output {
		match self {
			Scalar::Float { v } => {wrap_float!(-v)},
			Scalar::Rational { v } => {wrap_rational!(-v)},
		}
	}
}

impl Add for Scalar {
	type Output = Self;

	fn add(self, other: Self) -> Self::Output {
		match (&self, &other) {
			(Scalar::Float{v:va}, Scalar::Float{v:vb}) => {wrap_float!(va.clone()+vb.clone())},
			(Scalar::Float{..}, Scalar::Rational{..}) => {self + to_float(other)},
			(Scalar::Rational{..}, Scalar::Float{..}) => {to_float(self) + other},
			(Scalar::Rational{v:va}, Scalar::Rational{v:vb}) => {wrap_rational!(va.clone()+vb.clone())},
		}
	}
}

impl AddAssign for Scalar where {
	fn add_assign(&mut self, other: Self) {
		match (&mut *self, &other) {
			(Scalar::Float{v:va}, Scalar::Float{v:vb}) => {*va += vb.clone()},
			(Scalar::Float{..}, Scalar::Rational{..}) => {*self += to_float(other)},
			(Scalar::Rational{..}, Scalar::Float{..}) => {*self = to_float(self.clone()) + other },
			(Scalar::Rational{v:va}, Scalar::Rational{v:vb}) => {*va += vb.clone()},
		}
	}
}

impl Sub for Scalar {
	type Output = Self;

	fn sub(self, other: Self) -> Self::Output {
		match (&self, &other) {
			(Scalar::Float{v:va}, Scalar::Float{v:vb}) => {wrap_float!(va.clone()-vb.clone())},
			(Scalar::Float{..}, Scalar::Rational{..}) => {self - to_float(other)},
			(Scalar::Rational{..}, Scalar::Float{..}) => {to_float(self) - other},
			(Scalar::Rational{v:va}, Scalar::Rational{v:vb}) => {wrap_rational!(va.clone()-vb.clone())},
		}
	}
}

impl SubAssign for Scalar where {
	fn sub_assign(&mut self, other: Self) {
		match (&mut *self, &other) {
			(Scalar::Float{v:va}, Scalar::Float{v:vb}) => {*va -= vb.clone()},
			(Scalar::Float{..}, Scalar::Rational{..}) => {*self -= to_float(other)},
			(Scalar::Rational{..}, Scalar::Float{..}) => {*self = to_float(self.clone()) - other },
			(Scalar::Rational{v:va}, Scalar::Rational{v:vb}) => {*va -= vb.clone()},
		}
	}
}

impl Mul for Scalar {
	type Output = Self;

	fn mul(self, other: Self) -> Self::Output {
		match (&self, &other) {
			(Scalar::Float{v:va}, Scalar::Float{v:vb}) => {wrap_float!(va.clone()*vb.clone())},
			(Scalar::Float{..}, Scalar::Rational{..}) => {self * to_float(other)},
			(Scalar::Rational{..}, Scalar::Float{..}) => {to_float(self) * other},
			(Scalar::Rational{v:va}, Scalar::Rational{v:vb}) => {wrap_rational!(va.clone()*vb.clone())},
		}
	}
}

impl MulAssign for Scalar where {
	fn mul_assign(&mut self, other: Self) {
		match (&mut *self, &other) {
			(Scalar::Float{v:va}, Scalar::Float{v:vb}) => {*va *= vb.clone()},
			(Scalar::Float{..}, Scalar::Rational{..}) => {*self *= to_float(other)},
			(Scalar::Rational{..}, Scalar::Float{..}) => {*self = to_float(self.clone()) * other },
			(Scalar::Rational{v:va}, Scalar::Rational{v:vb}) => {*va *= vb.clone()},
		}
	}
}

impl Div for Scalar {
	type Output = Self;

	fn div(self, other: Self) -> Self::Output {
		match (&self, &other) {
			(Scalar::Float{v:va}, Scalar::Float{v:vb}) => {wrap_float!(va.clone()/vb.clone())},
			(Scalar::Float{..}, Scalar::Rational{..}) => {self / to_float(other)},
			(Scalar::Rational{..}, Scalar::Float{..}) => {to_float(self) / other},
			(Scalar::Rational{v:va}, Scalar::Rational{v:vb}) => {wrap_rational!(va.clone()/vb.clone())},
		}
	}
}

impl DivAssign for Scalar where {
	fn div_assign(&mut self, other: Self) {
		match (&mut *self, &other) {
			(Scalar::Float{v:va}, Scalar::Float{v:vb}) => {*va /= vb.clone()},
			(Scalar::Float{..}, Scalar::Rational{..}) => {*self /= to_float(other)},
			(Scalar::Rational{..}, Scalar::Float{..}) => {*self = to_float(self.clone()) / other },
			(Scalar::Rational{v:va}, Scalar::Rational{v:vb}) => {*va /= vb.clone()},
		}
	}
}

impl Rem<Scalar> for Scalar {
	type Output = Self;

	fn rem(self, other: Scalar) -> Self::Output {
		match (&self, &other) {
			(Scalar::Float{v:va}, Scalar::Float{v:vb}) => {wrap_float!(va.clone()%vb.clone())},
			(Scalar::Float{..}, Scalar::Rational{..}) => {self % to_float(other)},
			(Scalar::Rational{..}, Scalar::Float{..}) => {to_float(self) % other},
			(Scalar::Rational{v:va}, Scalar::Rational{v:vb}) => {wrap_rational!(va.clone()%vb.clone())},
		}
	}
}

impl PartialEq for Scalar {
	fn eq(&self, other: &Self) -> bool {
		match (self, other) {
			(Scalar::Float{v:va}, Scalar::Float{v:vb}) => { va == vb },
			(Scalar::Float{..}, Scalar::Rational{..}) => {*self == to_float(other.clone())},
			(Scalar::Rational{..}, Scalar::Float{..}) => {to_float(self.clone()) == *other},
			(Scalar::Rational{v:va}, Scalar::Rational{v:vb}) => { va == vb },
		}
	}
}

impl PartialOrd for Scalar {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		match (self, other) {
			(Scalar::Float{v:va}, Scalar::Float{v:vb}) => {va.partial_cmp(vb)},
			(Scalar::Float{..}, Scalar::Rational{..}) => {(*self).partial_cmp(&to_float(other.clone()))},
			(Scalar::Rational{..}, Scalar::Float{..}) => {to_float(self.clone()).partial_cmp(other)},
			(Scalar::Rational{v:va}, Scalar::Rational{v:vb}) => {va.partial_cmp(vb)},
		}
	}
}