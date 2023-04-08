use std::ops::{
	Add, Sub, Mul, Div,
	Neg, Rem,

	AddAssign, SubAssign,
	MulAssign, DivAssign
};


/*
Quantity:
	Represents a value with a unit attached to it.
Units have yet to be implemented.

f64q: a quantity based on plain f64s
floatq: a quantity using rug bigfloat
rationalq: a quantity using rug rationals

All of the above are ONLY used for values.
There is only one kind of unit type.


The cfg_if blocks here are a temporary hack to allow for
cross-compilation to other systems. RUG does not work on all systems.
*/

pub mod quantity;
mod unit;
pub use crate::quantity::unit::Unit;
pub use crate::quantity::unit::BaseUnit;

cfg_if::cfg_if! {
	if #[cfg(target_family = "unix")] {
		mod rationalq;
		mod floatq;
	} else {
		mod f64q;
	}
}

macro_rules! wrap_rational {
	( $x:expr, $y:expr ) => { Quantity::Rational{v: $x, u: $y} }
}

macro_rules! wrap_float {
	( $x:expr, $y:expr ) => { Quantity::Float{v: $x, u: $y} }
}

pub use crate::quantity::quantity::Quantity;
pub(in crate::quantity) use wrap_rational;
pub(in crate::quantity) use wrap_float;


const FLOAT_PRECISION: u32 = 1024;
const PRINT_LEN: usize = 5; // How many significant digits we will show in output


pub trait RationalBase: QuantBase {
	fn from_frac(top: i64, bot: i64) -> Self;
	fn from_f64(f: f64) -> Option<Self> where Self: Sized;
	fn from_string(s: &str) -> Option<Self>where Self: Sized;
}

pub trait FloatBase: QuantBase {
	fn from_f64(f: f64) -> Option<Self> where Self: Sized;
	fn from_string(s: &str) -> Option<Self> where Self: Sized;
}

pub trait QuantBase:
	Sized + ToString +
	Add + AddAssign +
	Sub + SubAssign +
	Mul + MulAssign + 
	Div + DivAssign + 
	Neg + Rem +
	PartialEq + PartialOrd
{
	fn fract(&self) -> Option<Self>;
	fn is_zero(&self) -> bool;
	fn is_negative(&self) -> bool;
	fn is_positive(&self) -> bool;

	fn exp(&self) -> Option<Self>;
	fn abs(&self) -> Option<Self>;
	fn floor(&self) -> Option<Self>;
	fn ceil(&self) -> Option<Self>;
	fn round(&self) -> Option<Self>;
	fn sin(&self) -> Option<Self>;
	fn cos(&self) -> Option<Self>;
	fn tan(&self) -> Option<Self>;
	fn asin(&self) -> Option<Self>;
	fn acos(&self) -> Option<Self>;
	fn atan(&self) -> Option<Self>;
	fn sinh(&self) -> Option<Self>;
	fn cosh(&self) -> Option<Self>;
	fn tanh(&self) -> Option<Self>;
	fn asinh(&self) -> Option<Self>;
	fn acosh(&self) -> Option<Self>;
	fn atanh(&self) -> Option<Self>;
	fn ln(&self) -> Option<Self>;
	fn log10(&self) -> Option<Self>;
	fn log2(&self) -> Option<Self>;
	fn log(&self, base: Self) -> Option<Self>;
	fn pow(&self, exp: Self) -> Option<Self>;
}