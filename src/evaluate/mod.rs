mod operator;
mod function;
mod evaluate;

pub use self::evaluate::evaluate;

#[derive(Debug)]
pub enum EvalError {
	BadMath,
	TooBig,
	ZeroDivision,
	IncompatibleUnit
}