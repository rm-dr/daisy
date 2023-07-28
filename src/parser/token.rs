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
	Quantity(LineLocation, String),
	Word(LineLocation, String),
	Operator(LineLocation, String),

	GroupStart(LineLocation),
	GroupEnd(LineLocation),
	Group(LineLocation, VecDeque<Token>),

	// Never parsed from input, used to build a tree.
	Container(Expression)
}

impl Token {
	#[inline(always)]
	pub fn get_line_location(&self) -> &LineLocation {
		match self {
			Token::Quantity(l, _)
			| Token::Word(l, _)
			| Token::Operator(l, _)
			| Token::GroupStart(l)
			| Token::GroupEnd(l)
			| Token::Group(l, _)
			=> l,

			Token::Container(_) => panic!("Containers do not have a linelocation.")
		}
	}

	#[inline(always)]
	pub fn get_mut_line_location(&mut self) -> &mut LineLocation {
		match self {
			Token::Quantity(l, _)
			| Token::Word(l, _)
			| Token::Operator(l, _)
			| Token::GroupStart(l)
			| Token::GroupEnd(l)
			| Token::Group(l, _)
			=> l,

			Token::Container(_) => panic!("Containers do not have a linelocation.")
		}
	}

	#[inline(always)]
	pub fn to_expression(self, context: &Context) -> Result<Expression, (LineLocation, ParserError)>{
		match self {
			Token::Quantity(l, mut s) => {

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

			Token::Word(_l, s) => {

				let c = Constant::from_string(&s);
				if c.is_some() { return Ok(Expression::Constant(c.unwrap())); }

				let c = Unit::from_string(&s);
				if c.is_some() { return Ok(Expression::Quantity(c.unwrap())); }

				let c = context.get_variable(&s);
				if c.is_some() { return Ok(Expression::Variable(s)); }
				return Ok(Expression::Variable(s));
			}

			Token::Container(v) => { return Ok(v); }

			Token::Operator(_,_)
			| Token::GroupStart(_)
			| Token::GroupEnd(_)
			| Token::Group(_, _)
			=> panic!("This token cannot be converted to an expression")
		};
	}

}