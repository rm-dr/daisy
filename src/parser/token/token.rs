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

	pub fn is_quantity(&self) -> bool {
		match self {
			Token::Quantity(_) => true,
			_ => false
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
	pub fn get_args(&self) -> Option<&VecDeque<Token>> {
		match self {
			Token::Operator(_, ref a) => Some(a),
			_ => None
		}
	}

	#[inline(always)]
	pub fn get_at_coords<'a, 'b, I>(&'a self, coords: I) -> Option<&'a Token>
	where I: IntoIterator<Item = &'b usize> + Sized {
		let mut g = self;
		for t in coords.into_iter() {
			let args = g.get_args();
			let Some(args) = args else { return None; };
			g = &args[*t];
		}
		return Some(g);
	}

	#[inline(always)]
	pub fn get_at_coords_mut<'a, 'b, I>(&'a mut self, coords: I) -> Option<&'a mut Token>
	where I: IntoIterator<Item = &'b usize> + Sized {
		let mut g = self;
		for t in coords.into_iter() {
			let args = g.get_args_mut();
			let Some(args) = args else { return None; };
			g = &mut args[*t];
		}
		return Some(g);
	}
}