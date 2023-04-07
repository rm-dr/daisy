use std::collections::VecDeque;

mod function;
mod operator;

pub use crate::tokens::function::Function;
pub use crate::tokens::operator::Operator;


use crate::quantity::Quantity;

/// Tokens represent logical objects in an expession.
///
/// Tokens starting with `Pre*` are intermediate tokens, and
/// will never show up in a fully-parsed expression tree.
#[derive(Debug)]
#[derive(Clone)]
pub enum Token {
	Number(Quantity),

	Constant(Quantity, String),

	Operator(
		Operator,
		VecDeque<Token>
	),
}

impl Token {

	pub fn print(&self) -> String {
		match self {
			Token::Number(v) => v.to_string(),
			Token::Constant(_,s) => s.clone(),
			Token::Operator(o,a) => o.print(a)
		}
	}

	#[inline(always)]
	pub fn get_args(&self) -> Option<&VecDeque<Token>> {
		match self {
			Token::Operator(_, ref a) => Some(a),
			_ => None
		}
	}

	#[inline(always)]
	pub fn get_args_mut(&mut self) -> Option<&mut VecDeque<Token>> {
		match self {
			Token::Operator(_, ref mut a) => Some(a),
			_ => None
		}
	}

	#[inline(always)]
	pub fn eval(&self) -> Result<Token, ()> {
		Ok(match self {
			Token::Number(_) => { self.clone() },
			Token::Constant(v,_) => { Token::Number(v.clone()) },
			Token::Operator(o,v) => { o.apply(&v)? }
		})
	}

	// Temporary solution
	#[inline(always)]
	pub fn as_number(&self) -> Token {
		match self {
			Token::Number(v) => { Token::Number(v.clone()) },
			Token::Constant(v,_) => { Token::Number(v.clone()) },
			_ => panic!()
		}
	}

}