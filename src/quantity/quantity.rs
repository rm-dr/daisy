use std::ops::{
	Add, Sub, Mul, Div,
	Neg, Rem,

	AddAssign, SubAssign,
	MulAssign, DivAssign
};
use std::cmp::Ordering;

use crate::quantity::wrap_rational;
use crate::quantity::wrap_float;

use crate::quantity::QuantBase;
use crate::quantity::RationalBase;
use crate::quantity::FloatBase;

use crate::quantity::Unit;
use crate::quantity::BaseUnit;

cfg_if::cfg_if! {
	if #[cfg(target_family = "unix")] {
		use crate::quantity::rationalq::RationalQ;
		use crate::quantity::floatq::FloatQ;

		#[derive(Debug)]
		#[derive(Clone)]
		pub enum Quantity {
			Rational{ v: RationalQ, u: Unit },
			Float{ v: FloatQ, u: Unit }
		}
	} else {
		use crate::quantity::f64q::F64Q;

		#[derive(Debug)]
		#[derive(Clone)]
		pub enum Quantity {
			Rational{ v: F64Q, u: Unit },
			Float{ v: F64Q, u: Unit }
		}
	}
}


impl Quantity {

	cfg_if::cfg_if! {
		if #[cfg(target_family = "unix")] {
			pub fn new_rational(top: i64, bottom: i64) -> Quantity {
				return wrap_rational!(
					RationalQ::from_frac(top, bottom),
					Unit::new()
				);
			}
		
			pub fn new_float(v: f64) -> Quantity {
				return wrap_float!(
					FloatQ::from_f64(v).unwrap(),
					Unit::new()
				)
			}
		
			pub fn new_rational_from_string(s: &str) -> Option<Quantity> {
				let r = RationalQ::from_string(s);
				if r.is_none() { return None; }
				return Some(wrap_rational!(
					r.unwrap(),
					Unit::new()
				));
			}
		
			pub fn new_float_from_string(s: &str) -> Option<Quantity> {
				let v = FloatQ::from_string(s);
				if v.is_none() { return None; }
				return Some(wrap_float!(
					v.unwrap(),
					Unit::new()
				))
			}

			pub fn float_from_rat(r: &Quantity) -> Quantity {
				match &r {
					Quantity::Float { .. } => r.clone(),
					Quantity::Rational { v, u } => wrap_float!(
						FloatQ::from(v.val.numer()).unwrap() /
						FloatQ::from(v.val.denom()).unwrap(),
						u.clone()
					)
				}
			}
		} else {
			pub fn new_rational(top: i64, bottom: i64) -> Quantity {
				return wrap_float!(F64Q::from_f64( (top as f64) / (bottom as f64)).unwrap())
			}
		
			pub fn new_float(v: f64) -> Quantity {
				return wrap_float!(F64Q::from_f64(v).unwrap())
			}
		
			pub fn new_rational_from_string(s: &str) -> Option<Quantity> {
				let r = F64Q::from_string(s);
				if r.is_none() { return None; }
				return Some(wrap_rational!(r.unwrap()));
			}
		
			pub fn new_float_from_string(s: &str) -> Option<Quantity> {
				let v = F64Q::from_string(s);
				if v.is_none() { return None; }
				return Some(wrap_float!(v.unwrap()))
			}

			pub fn float_from_rat(r: &Quantity) -> Quantity {
				match &r {
					Quantity::Float { .. } => r.clone(),
					Quantity::Rational { .. } => r.clone()
				}
			}
		}
	}

	pub fn is_nan(&self) -> bool {
		match self {
			Quantity::Float { v, .. } => {v.val.is_nan()},
			Quantity::Rational { .. } => {panic!()}
		}
	}

	pub fn add_unit(&mut self, ui: BaseUnit, pi: f64) {
		match self {
			Quantity::Float { u, .. } => {u.insert(ui, pi)},
			Quantity::Rational { u, .. } => {u.insert(ui, pi)}
		}
	}
}



impl ToString for Quantity {
	fn to_string(&self) -> String {
		let mut n: String;
		let u: &Unit;
		match self {
			Quantity::Rational{u:un, ..} => {
				n = Quantity::float_from_rat(self).to_string();
				u = un
			},
			Quantity::Float{v, u:un} => {
				n = v.to_string();
				u = un;
			},
		};

		//n.push(' ');
		//n.push_str(&u.to_string());
		n
	}
}


macro_rules! quant_foward {
	( $x:ident ) => {
		pub fn $x(&self) -> Quantity {
			match self {
				Quantity::Rational{v, u} => {
					if !u.unitless() { panic!() }
					let r = v.$x();
					if r.is_none() {
						let v = Quantity::float_from_rat(self);
						return v.$x();
					} else {wrap_rational!(r.unwrap(), u.clone())}
				},
				Quantity::Float{v, u} => {
					if !u.unitless() { panic!() }
					wrap_float!(v.$x().unwrap(), u.clone())
				},
			}
		}
	}
}

impl Quantity {

	pub fn is_zero(&self) -> bool {
		match self {
			Quantity::Rational{v, .. } => v.is_zero(),
			Quantity::Float{v, .. } => v.is_zero(),
		}
	}

	pub fn is_negative(&self) -> bool {
		match self {
			Quantity::Rational{v, .. } => v.is_negative(),
			Quantity::Float{v, .. } => v.is_negative(),
		}
	}

	pub fn is_positive(&self) -> bool {
		match self {
			Quantity::Rational{v, .. } => v.is_positive(),
			Quantity::Float{v, .. } => v.is_positive(),
		}
	}

	pub fn unitless(&self) -> bool {
		match self {
			Quantity::Rational{ u, .. } => u.unitless(),
			Quantity::Float{ u, .. } => u.unitless(),
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

	pub fn log(&self, base: Quantity) -> Quantity {

		if !self.unitless() { panic!() }
		
		match self {
			Quantity::Rational{u, .. } => {
				if !u.unitless() { panic!() }
				Quantity::float_from_rat(self).log(Quantity::float_from_rat(&base))
			},
			Quantity::Float{u, .. } => {
				if !u.unitless() { panic!() }
				Quantity::float_from_rat(self).log(base)
			},
		}
	}
	pub fn pow(&self, base: Quantity) -> Quantity {
		match self {
			Quantity::Rational{u, .. } => {
				let a = match Quantity::float_from_rat(self) {
					Quantity::Rational{ .. } => panic!(),
					Quantity::Float{v, .. } => v,
				};

				let b = match Quantity::float_from_rat(&base) {
					Quantity::Rational{ .. } => panic!(),
					Quantity::Float{v, .. } => v,
				};

				let mut nu = u.clone();
				nu.pow(2f64);
				wrap_float!(a.pow(b).unwrap(), nu)
			},
			Quantity::Float{u, .. } => {
				if !u.unitless() { panic!() }

				let a = match Quantity::float_from_rat(self) {
					Quantity::Rational{ .. } => panic!(),
					Quantity::Float{v, .. } => v,
				};

				let b = match Quantity::float_from_rat(&base) {
					Quantity::Rational{ .. } => panic!(),
					Quantity::Float{v, .. } => v,
				};
				let mut nu = u.clone();
				nu.pow(2f64);
				wrap_float!(a.pow(b).unwrap(), nu)

			},
		}
	}
}



impl Neg for Quantity where {
	type Output = Self;

	fn neg(self) -> Self::Output {
		match self {
			Quantity::Float { v, u } => {wrap_float!(-v, u)},
			Quantity::Rational { v, u } => {wrap_rational!(-v, u)},
		}
	}
}

impl Add for Quantity {
	type Output = Self;

	fn add(self, other: Self) -> Self::Output {
		match (&self, &other) {
			(Quantity::Float{v:va,u:ua}, Quantity::Float{v:vb,u:ub}) => {
				if ua != ub { panic!() }
				wrap_float!(va.clone()+vb.clone(), ua.clone())
			},
			(Quantity::Float{ .. }, Quantity::Rational{ .. }) => {self + Quantity::float_from_rat(&other)},
			(Quantity::Rational{ .. }, Quantity::Float{ .. }) => {Quantity::float_from_rat(&self) + other},
			(Quantity::Rational{v:va,u:ua}, Quantity::Rational{v:vb,u:ub}) => {
				if ua != ub { panic!() }
				wrap_rational!(va.clone()+vb.clone(), ua.clone())
			},
		}
	}
}

impl AddAssign for Quantity where {
	fn add_assign(&mut self, other: Self) {
		match (&mut *self, &other) {
			(Quantity::Float{v:va,u:ua}, Quantity::Float{v:vb,u:ub}) => {
				if ua != ub { panic!() }
				*va += vb.clone()
			},
			(Quantity::Float{ .. }, Quantity::Rational{ .. }) => {*self += Quantity::float_from_rat(&other)},
			(Quantity::Rational{ .. }, Quantity::Float{ .. }) => {*self = Quantity::float_from_rat(self) + other },
			(Quantity::Rational{v:va,u:ua}, Quantity::Rational{v:vb,u:ub}) => {
				if ua != ub { panic!() }
				*va += vb.clone()
			},
		}
	}
}

impl Sub for Quantity {
	type Output = Self;

	fn sub(self, other: Self) -> Self::Output {
		match (&self, &other) {
			(Quantity::Float{v:va,u:ua}, Quantity::Float{v:vb,u:ub}) => {
				if ua != ub { panic!() }
				wrap_float!(va.clone()-vb.clone(), ua.clone())
			},
			(Quantity::Float{ .. }, Quantity::Rational{ .. }) => {self - Quantity::float_from_rat(&other)},
			(Quantity::Rational{ .. }, Quantity::Float{ .. }) => {Quantity::float_from_rat(&self) - other},
			(Quantity::Rational{v:va,u:ua}, Quantity::Rational{v:vb,u:ub}) => {
				if ua != ub { panic!() }
				wrap_rational!(va.clone()-vb.clone(), ua.clone())
			},
		}
	}
}

impl SubAssign for Quantity where {
	fn sub_assign(&mut self, other: Self) {
		match (&mut *self, &other) {
			(Quantity::Float{v:va,u:ua}, Quantity::Float{v:vb,u:ub}) => {
				if ua != ub { panic!() }
				*va -= vb.clone()
			},
			(Quantity::Float{ .. }, Quantity::Rational{ .. }) => {*self -= Quantity::float_from_rat(&other)},
			(Quantity::Rational{ .. }, Quantity::Float{ .. }) => {*self = Quantity::float_from_rat(self) - other },
			(Quantity::Rational{v:va,u:ua}, Quantity::Rational{v:vb,u:ub}) => {
				if ua != ub { panic!() }
				*va -= vb.clone()
			},
		}
	}
}

impl Mul for Quantity {
	type Output = Self;

	fn mul(self, other: Self) -> Self::Output {
		match (&self, &other) {
			(Quantity::Float{v:va,u:ua}, Quantity::Float{v:vb,u:ub}) => {
				let u = ua.clone()*ub.clone();
				wrap_float!(va.clone()*vb.clone(), u)
			},
			(Quantity::Float{ .. }, Quantity::Rational{ .. }) => {self * Quantity::float_from_rat(&other)},
			(Quantity::Rational{ .. }, Quantity::Float{ .. }) => {Quantity::float_from_rat(&self) * self},
			(Quantity::Rational{v:va,u:ua}, Quantity::Rational{v:vb,u:ub}) => {
				let u = ua.clone()*ub.clone();
				wrap_rational!(va.clone()*vb.clone(), u)
			},
		}
	}
}

impl MulAssign for Quantity where {
	fn mul_assign(&mut self, other: Self) {
		match (&mut *self, &other) {
			(Quantity::Float{v:va,u:ua}, Quantity::Float{v:vb,u:ub}) => {
				*ua *= ub.clone();
				*va *= vb.clone()
			},
			(Quantity::Float{ .. }, Quantity::Rational{ .. }) => {*self *= Quantity::float_from_rat(&other)},
			(Quantity::Rational{ .. }, Quantity::Float{ .. }) => {*self = Quantity::float_from_rat(self) * other },
			(Quantity::Rational{v:va,u:ua}, Quantity::Rational{v:vb,u:ub}) => {
				*ua *= ub.clone();
				*va *= vb.clone()
			},
		}
	}
}

impl Div for Quantity {
	type Output = Self;

	fn div(self, other: Self) -> Self::Output {
		match (&self, &other) {
			(Quantity::Float{v:va,u:ua}, Quantity::Float{v:vb,u:ub}) => {
				let u = ua.clone()/ub.clone();
				wrap_float!(va.clone()/vb.clone(), u)
			},
			(Quantity::Float{ .. }, Quantity::Rational{ .. }) => {self / Quantity::float_from_rat(&other)},
			(Quantity::Rational{ .. }, Quantity::Float{ .. }) => {Quantity::float_from_rat(&self) / other},
			(Quantity::Rational{v:va,u:ua}, Quantity::Rational{v:vb,u:ub}) => {
				let u = ua.clone()/ub.clone();
				wrap_rational!(va.clone()/vb.clone(), u)
			},
		}
	}
}

impl DivAssign for Quantity where {
	fn div_assign(&mut self, other: Self) {
		match (&mut *self, &other) {
			(Quantity::Float{v:va,u:ua}, Quantity::Float{v:vb,u:ub}) => {
				*ua /= ub.clone();
				*va /= vb.clone()
			},
			(Quantity::Float{ .. }, Quantity::Rational{ .. }) => {*self /= Quantity::float_from_rat(&other)},
			(Quantity::Rational{ .. }, Quantity::Float{ .. }) => {*self = Quantity::float_from_rat(self) / other },
			(Quantity::Rational{v:va,u:ua}, Quantity::Rational{v:vb,u:ub}) => {
				*ua /= ub.clone();
				*va /= vb.clone()
			}
		}
	}
}

impl Rem<Quantity> for Quantity {
	type Output = Self;

	fn rem(self, other: Quantity) -> Self::Output {
		match (&self, &other) {
			(Quantity::Float{v:va,u:ua}, Quantity::Float{v:vb,u:ub}) => {
				if ua != ub { panic!() }
				wrap_float!(va.clone()%vb.clone(), ua.clone())
			},
			(Quantity::Float{ .. }, Quantity::Rational{ .. }) => {self % Quantity::float_from_rat(&other)},
			(Quantity::Rational{ .. }, Quantity::Float{ .. }) => {Quantity::float_from_rat(&self) % other},
			(Quantity::Rational{v:va,u:ua}, Quantity::Rational{v:vb,u:ub}) => {
				if ua != ub { panic!() }
				wrap_rational!(va.clone()%vb.clone(), ua.clone())
			},
		}
	}
}

impl PartialEq for Quantity {
	fn eq(&self, other: &Self) -> bool {
		match (self, other) {
			(Quantity::Float{v:va,u:ua}, Quantity::Float{v:vb,u:ub}) => { if ua!=ub {false} else {va == vb} },
			(Quantity::Float{ .. }, Quantity::Rational{ .. }) => {*self == Quantity::float_from_rat(other)},
			(Quantity::Rational{ .. }, Quantity::Float{ .. }) => {Quantity::float_from_rat(self) == *other},
			(Quantity::Rational{v:va,u:ua}, Quantity::Rational{v:vb,u:ub}) => { if ua!=ub {false} else {va == vb} },
		}
	}
}

impl PartialOrd for Quantity {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		match (self, other) {
			(Quantity::Float{v:va,u:ua}, Quantity::Float{v:vb,u:ub}) => {
				if ua != ub { panic!() }
				va.partial_cmp(vb)
			},
			(Quantity::Float{ .. }, Quantity::Rational{ .. }) => {(*self).partial_cmp(&Quantity::float_from_rat(other))},
			(Quantity::Rational{ .. }, Quantity::Float{ .. }) => {Quantity::float_from_rat(self).partial_cmp(other)},
			(Quantity::Rational{v:va,u:ua}, Quantity::Rational{v:vb,u:ub}) => {
				if ua != ub { panic!() }
				va.partial_cmp(vb)
			},
		}
	}
}