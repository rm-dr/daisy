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

cfg_if::cfg_if! {
	if #[cfg(target_arch = "unix")] {
		mod rationalq;
		mod floatq;
	} else {
		mod f64q;
	}
}

macro_rules! wrap_rational {
	( $x:expr ) => { Quantity::Rational{v: $x} }
}

macro_rules! wrap_float {
	( $x:expr ) => { Quantity::Float{v: $x} }
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
	fn fract(&self) -> Quantity;
	fn is_zero(&self) -> bool;
	fn is_negative(&self) -> bool;
	fn is_positive(&self) -> bool;

	fn exp(&self) -> Quantity;
	fn abs(&self) -> Quantity;
	fn floor(&self) -> Quantity;
	fn ceil(&self) -> Quantity;
	fn round(&self) -> Quantity;
	fn sin(&self) -> Quantity;
	fn cos(&self) -> Quantity;
	fn tan(&self) -> Quantity;
	fn asin(&self) -> Quantity;
	fn acos(&self) -> Quantity;
	fn atan(&self) -> Quantity;
	fn sinh(&self) -> Quantity;
	fn cosh(&self) -> Quantity;
	fn tanh(&self) -> Quantity;
	fn asinh(&self) -> Quantity;
	fn acosh(&self) -> Quantity;
	fn atanh(&self) -> Quantity;
	fn ln(&self) -> Quantity;
	fn log10(&self) -> Quantity;
	fn log2(&self) -> Quantity;
	fn log(&self, base: Quantity) -> Quantity;
	fn pow(&self, exp: Quantity) -> Quantity;
}