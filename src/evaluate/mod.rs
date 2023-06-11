mod operator;
mod function;
mod evaluate;
pub use self::evaluate::evaluate;

pub enum EvalError {
	BadMath,
	TooBig,
	ZeroDivision,
	IncompatibleUnit
}