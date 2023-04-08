use std::ops::{
	Add, Sub, Mul, Div,
	Neg, Rem,

	AddAssign, SubAssign,
	MulAssign, DivAssign
};



mod rationalq;
mod floatq;
pub mod quantity;

pub use crate::quantity::quantity::Quantity;

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