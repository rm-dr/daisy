mod function;
mod operator;
mod token;

pub use crate::tokens::token::Token;
pub use crate::tokens::function::Function;
pub use crate::tokens::operator::Operator;

pub enum EvalError {
	BadMath,
	TooBig,
	ZeroDivision,
	IncompatibleUnit
}