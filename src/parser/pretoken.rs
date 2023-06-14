use std::collections::VecDeque;
use crate::quantity::Unit;
use crate::quantity::Quantity;

use super::{
	LineLocation,
	ParserError,
	Token,
	Constant
};


#[derive(Debug)]
pub enum PreToken {
	PreQuantity(LineLocation, String),
	PreWord(LineLocation, String),
	PreOperator(LineLocation, String),

	PreGroupStart(LineLocation),
	PreGroupEnd(LineLocation),
	PreGroup(LineLocation, VecDeque<PreToken>),

	Container(Token)
}

impl PreToken {
	#[inline(always)]
	pub fn get_line_location(&self) -> &LineLocation {
		match self {
			PreToken::PreQuantity(l, _)
			| PreToken::PreWord(l, _)
			| PreToken::PreOperator(l, _)
			| PreToken::PreGroupStart(l)
			| PreToken::PreGroupEnd(l)
			| PreToken::PreGroup(l, _)
			=> l,

			_ => panic!()
		}
	}

	#[inline(always)]
	pub fn get_mut_line_location(&mut self) -> &mut LineLocation {
		match self {
			PreToken::PreQuantity(l, _)
			| PreToken::PreWord(l, _)
			| PreToken::PreOperator(l, _)
			| PreToken::PreGroupStart(l)
			| PreToken::PreGroupEnd(l)
			| PreToken::PreGroup(l, _)
			=> l,

			_ => panic!()
		}
	}

	#[inline(always)]
	pub fn to_token(self) -> Result<Token, (LineLocation, ParserError)>{
		match self {
			PreToken::PreQuantity(l, mut s) => {

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
				return Ok(Token::Quantity(r.unwrap()));
			},

			PreToken::PreWord(l, s) => {
				let c = Constant::from_string(&s);

				if c.is_some() {
					return Ok(Token::Constant(c.unwrap()));
				}

				let c = Unit::from_string(&s);
				if c.is_some() { return Ok(Token::Quantity(c.unwrap())); }

				return Err((l, ParserError::Undefined(s)));
			}

			PreToken::Container(v) => { return Ok(v); }

			PreToken::PreOperator(_,_)
			| PreToken::PreGroupStart(_)
			| PreToken::PreGroupEnd(_)
			| PreToken::PreGroup(_, _)
			=> panic!()
		};
	}

}