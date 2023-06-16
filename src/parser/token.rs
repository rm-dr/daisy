use std::collections::VecDeque;
use crate::quantity::Unit;
use crate::quantity::Quantity;
use crate::context::Context;

use super::{
	LineLocation,
	ParserError,
	Expression,
	Constant
};


#[derive(Debug)]
pub enum Token {
	PreQuantity(LineLocation, String),
	PreWord(LineLocation, String),
	PreOperator(LineLocation, String),

	PreGroupStart(LineLocation),
	PreGroupEnd(LineLocation),
	PreGroup(LineLocation, VecDeque<Token>),

	Container(Expression)
}

impl Token {
	#[inline(always)]
	pub fn get_line_location(&self) -> &LineLocation {
		match self {
			Token::PreQuantity(l, _)
			| Token::PreWord(l, _)
			| Token::PreOperator(l, _)
			| Token::PreGroupStart(l)
			| Token::PreGroupEnd(l)
			| Token::PreGroup(l, _)
			=> l,

			_ => panic!()
		}
	}

	#[inline(always)]
	pub fn get_mut_line_location(&mut self) -> &mut LineLocation {
		match self {
			Token::PreQuantity(l, _)
			| Token::PreWord(l, _)
			| Token::PreOperator(l, _)
			| Token::PreGroupStart(l)
			| Token::PreGroupEnd(l)
			| Token::PreGroup(l, _)
			=> l,

			_ => panic!()
		}
	}

	#[inline(always)]
	pub fn to_expression(self, context: &Context) -> Result<Expression, (LineLocation, ParserError)>{
		match self {
			Token::PreQuantity(l, mut s) => {

				// The length check here ensures that
				// `.` is not parsed as `0.`
				// That should be a syntax error.
				if s.len() != 1 && &s[0..1] == "." {
					s.insert(0, '0');
				}

				let r = Quantity::new_rational_from_string(&s);

				if r.is_none() {
					return Err((l, ParserError::BadNumber))
				}

				return Ok(Expression::Quantity(r.unwrap()));
			},

			Token::PreWord(_l, s) => {

				let c = Constant::from_string(&s);
				if c.is_some() { return Ok(Expression::Constant(c.unwrap())); }

				let c = Unit::from_string(&s);
				if c.is_some() { return Ok(Expression::Quantity(c.unwrap())); }

				let c = context.get_variable(&s);
				if c.is_some() { return Ok(Expression::Variable(s)); }
				return Ok(Expression::Variable(s));
			}

			Token::Container(v) => { return Ok(v); }

			Token::PreOperator(_,_)
			| Token::PreGroupStart(_)
			| Token::PreGroupEnd(_)
			| Token::PreGroup(_, _)
			=> panic!()
		};
	}

}