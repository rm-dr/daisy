mod operator;
mod function;
mod evaluate;

pub use self::evaluate::evaluate;

#[derive(Debug)]
pub enum EvalError {
	BadMath,
	TooBig,
	ZeroDivision,
	IncompatibleUnit,
	IncompatibleUnits(String, String),
	BadDefineName,
	Undefined(String)
}


impl ToString for EvalError {
	fn to_string(&self) -> String {
		match self {
			EvalError::BadMath => {
				String::from("Failed to evaluate expression")
			},
			EvalError::TooBig => {
				String::from("Number too big")
			},
			EvalError::ZeroDivision => {
				String::from("Division by zero")
			},
			EvalError::IncompatibleUnit => {
				String::from("Incompatible unit")
			},
			EvalError::IncompatibleUnits(a, b) => {
				format!("Incompatible units ({} and {})", a, b)
			},
			EvalError::BadDefineName => {
				String::from("Invalid variable name")
			},
			EvalError::Undefined(s) => {
				format!("{} is undefined", s)
			}
		}
	}
}
