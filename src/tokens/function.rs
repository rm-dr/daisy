use std::collections::VecDeque;

use crate::tokens::Token;
use crate::tokens::EvalError;
use crate::tokens::Operator;

#[derive(Debug)]
#[derive(Copy, Clone)]
pub enum Function {
	Abs,
	Floor,
	Ceil,
	Round,

	NaturalLog,
	TenLog,

	Sin,
	Cos,
	Tan,
	Asin,
	Acos,
	Atan,
	Csc,
	Sec,
	Cot,

	Sinh,
	Cosh,
	Tanh,
	Asinh,
	Acosh,
	Atanh,
	Csch,
	Sech,
	Coth,
}

impl Function {
	pub fn to_string(&self) -> String {
		match self {
			Function::Abs => { String::from("abs") },
			Function::Floor => { String::from("floor") },
			Function::Ceil => { String::from("ceil") },
			Function::Round => { String::from("round") },
			Function::NaturalLog => { String::from("ln") },
			Function::TenLog => { String::from("log") },
			Function::Sin => { String::from("sin") },
			Function::Cos => { String::from("cos") },
			Function::Tan => { String::from("tan") },
			Function::Asin => { String::from("asin") },
			Function::Acos => { String::from("acos") },
			Function::Atan => { String::from("atan") },
			Function::Csc => { String::from("csc") },
			Function::Sec => { String::from("sec") },
			Function::Cot => { String::from("cot") },
			Function::Sinh => { String::from("sinh") },
			Function::Cosh => { String::from("cosh") },
			Function::Tanh => { String::from("tanh") },
			Function::Asinh => { String::from("asinh") },
			Function::Acosh => { String::from("acosh") },
			Function::Atanh => { String::from("atanh") },
			Function::Csch => { String::from("csch") },
			Function::Sech => { String::from("sech") },
			Function::Coth => { String::from("coth") },
		}
	}

	pub fn apply(&self, args: &VecDeque<Token>) -> Result<Token, EvalError> {
		if args.len() != 1 {panic!()};
		let a = args[0].as_number();
		let Token::Number(q) = a else {panic!()};

		if !q.unitless() {
			return Err(EvalError::IncompatibleUnit);
		}

		match self {
			Function::Abs => { return Ok(Token::Number(q.abs())); },
			Function::Floor => { return Ok(Token::Number(q.floor())); },
			Function::Ceil => { return Ok(Token::Number(q.ceil())); },
			Function::Round => { return Ok(Token::Number(q.round())); },

			Function::NaturalLog => { return Ok(Token::Number(q.ln())); },
			Function::TenLog => { return Ok(Token::Number(q.log10())); },

			Function::Sin => { return Ok(Token::Number(q.sin())); },
			Function::Cos => { return Ok(Token::Number(q.cos())); },
			Function::Tan => { return Ok(Token::Number(q.tan())); },
			Function::Asin => { return Ok(Token::Number(q.asin())); },
			Function::Acos => { return Ok(Token::Number(q.acos())); },
			Function::Atan => { return Ok(Token::Number(q.atan())); },

			Function::Csc => {
				return Ok(
					Token::Operator(
						Operator::Flip,
						VecDeque::from(vec!(Token::Number(q.sin())))
					).eval()?
				);
			},
			Function::Sec => {
				return Ok(
					Token::Operator(
						Operator::Flip,
						VecDeque::from(vec!(Token::Number(q.cos())))
					).eval()?
				);
			},
			Function::Cot => {
				return Ok(
					Token::Operator(
						Operator::Flip,
						VecDeque::from(vec!(Token::Number(q.tan())))
					).eval()?
				);
			},


			Function::Sinh => { return Ok(Token::Number(q.sinh())); },
			Function::Cosh => { return Ok(Token::Number(q.cosh())); },
			Function::Tanh => { return Ok(Token::Number(q.tanh())); },
			Function::Asinh => { return Ok(Token::Number(q.asinh())); },
			Function::Acosh => { return Ok(Token::Number(q.acosh())); },
			Function::Atanh => { return Ok(Token::Number(q.atanh())); },

			Function::Csch => {
				return Ok(
					Token::Operator(
						Operator::Flip,
						VecDeque::from(vec!(Token::Number(q.sinh())))
					).eval()?
				);
			},
			Function::Sech => {
				return Ok(
					Token::Operator(
						Operator::Flip,
						VecDeque::from(vec!(Token::Number(q.cosh())))
					).eval()?
				);
			},
			Function::Coth => {
				return Ok(
					Token::Operator(
						Operator::Flip,
						VecDeque::from(vec!(Token::Number(q.tanh())))
					).eval()?
				);
			},

		}
	}
}
