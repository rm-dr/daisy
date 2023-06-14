use std::collections::VecDeque;
use crate::quantity::Quantity;

use super::Operator;
use super::Constant;

/// Tokens represent logical objects in an expession.
#[derive(Debug)]
#[derive(Clone)]
pub enum Token {
	Variable(String),
	Quantity(Quantity),
	Constant(Constant),
	Operator(Operator, VecDeque<Token>),
}

impl ToString for Token {
	fn to_string(&self) -> String {
		match self {
			Token::Quantity(v) => v.to_string(),
			Token::Constant(c) => c.to_string(),
			Token::Variable(s) => s.clone(),
			Token::Operator(o,a) => o.print(a)
		}
	}
}

impl Token {
	// This is called only when this is the outermost token.
	// This sometimes leads to different--usually more verbose--behavior.
	pub fn to_string_outer(&self) -> String {
		match self {
			Token::Quantity(v) => v.to_string_outer(),
			Token::Constant(c) => c.to_string(),
			Token::Variable(s) => s.clone(),
			Token::Operator(o,a) => o.print(a)
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
	pub fn get_at_coords<'a>(g: &'a mut Token, coords: &Vec<usize>) -> &'a mut Token {
		let mut h = &mut *g;

		for t in coords.iter() {
			let inner = h.get_args_mut().unwrap();
			h = &mut inner[*t];
		}

		return h;
	}
}