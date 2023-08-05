use std::collections::VecDeque;
use crate::quantity::Unit;
use crate::quantity::Quantity;
use crate::context::Context;
use crate::errors::DaisyError;

use super::{
	LineLocation,
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
	pub fn get_linelocation(&self) -> LineLocation {
		match self {
			Token::Quantity(l, _)
			| Token::Word(l, _)
			| Token::Operator(l, _)
			| Token::GroupStart(l)
			| Token::GroupEnd(l)
			| Token::Group(l, _)
			=> l.clone(),

			Token::Container(_) => panic!("Containers do not have a linelocation.")
		}
	}

	#[inline(always)]
	pub fn get_mut_linelocation(&mut self) -> &mut LineLocation {
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
	pub fn to_expression(self, context: &Context) -> Result<Expression, (LineLocation, DaisyError)>{
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
					return Err((l, DaisyError::BadNumber))
				}

				return Ok(Expression::Quantity(l, r.unwrap()));
			},

			Token::Word(l, s) => {
				let c = Constant::from_string(&s);
				if c.is_some() { return Ok(Expression::Constant(l, c.unwrap())); }

				let c = Unit::from_string(&s);
				if c.is_some() { return Ok(Expression::Quantity(l, c.unwrap())); }

				if context.is_varible(&s) { return Ok(Expression::Variable(l, s)); }
				return Ok(Expression::Variable(l, s));
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