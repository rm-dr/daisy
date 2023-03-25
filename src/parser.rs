mod tokenize;
mod treeify;
mod evaluate;

use crate::parser::tokenize::tokenize;
use crate::parser::treeify::treeify;
use crate::parser::evaluate::evaluate;

use std::collections::VecDeque;


/// Tokens represent logical objects in an expession.
/// 
/// Tokens starting with `Pre*` are intermediate tokens, and
/// will never show up in a fully-parsed expression tree.

#[derive(Debug)]
pub enum Token {

	/// Used only while tokenizing.
	/// Will be replaced with a Number once we finish.
	PreNumber(LineLocation, String),

	/// Used only while tokenizing.
	/// Will be replaced with one of the Tokens below once we finish.
	PreWord(LineLocation, String),

	/// Used only until operators are parsed.
	/// Each of these will become one of the operators below.
	PreOperator(LineLocation, Operator),

	/// Used only until operators are parsed.
	/// PreGroups aren't needed once we have a tree.
	PreGroup(LineLocation, VecDeque<Token>),

	Number(LineLocation, f64),
	Constant(LineLocation, f64, String),

	Root(VecDeque<Token>),
	Multiply(VecDeque<Token>),
	Divide(VecDeque<Token>),
	Add(VecDeque<Token>),
	Factorial(VecDeque<Token>),
	Negative(VecDeque<Token>),
	Power(VecDeque<Token>),
	Modulo(VecDeque<Token>),
}

impl Token {
	fn as_number(&self) -> Token {
		match self {
			Token::Number(l,v) => {
				Token::Number(*l, *v)
			},
			Token::Constant(l,v,_) => {
				Token::Number(*l, *v)
			},
			_ => panic!()
		}
	}

	pub fn eval(&self) -> Token {
		match self {
			Token::Root(ref v) => {
				if v.len() != 1 {panic!()};
				let v = v[0].as_number();

				if let Token::Number(l, v) = v {
					Token::Number(l, v)
				} else { panic!(); }
			},

			Token::Negative(ref v) => {
				if v.len() != 1 {panic!()};
				let v = v[0].as_number();

				if let Token::Number(l, v) = v {
					Token::Number(l, -v)
				} else { panic!(); }
			},

			Token::Add(ref v) => {
				let mut sum: f64 = 0f64;
				let mut new_pos: usize = 0;
				let mut new_len: usize = 0;
				for i in v.iter() {
					let j = i.as_number();
					if let Token::Number(l, v) = j {
						let LineLocation{pos, len} = l;
						if new_pos == 0 {new_pos = pos};
						new_len = new_len + len;
						sum += v;
					} else {
						panic!();
					}
				}

				Token::Number(
					LineLocation { pos: new_pos, len: new_len },
					sum
				)
			},

			Token::Multiply(ref v) => {
				let mut prod: f64 = 1f64;
				let mut new_pos: usize = 0;
				let mut new_len: usize = 0;
				for i in v.iter() {
					let j = i.as_number();
					if let Token::Number(l, v) = j {
						let LineLocation{pos, len} = l;
						if new_pos == 0 {new_pos = pos};
						new_len = new_len + len;
						prod *= v;
					} else {
						panic!();
					}
				}

				Token::Number(
					LineLocation { pos: new_pos, len: new_len },
					prod
				)
			},

			Token::Divide(ref v) => {
				if v.len() != 2 {panic!()};
				let a = v[0].as_number();
				let b = v[1].as_number();

				if let Token::Number(la, va) = a {
					if let Token::Number(lb, vb) = b {
						let LineLocation{pos: posa, ..} = la;
						let LineLocation{pos: posb, len: lenb} = lb;
						Token::Number(
							LineLocation { pos: posa, len: posb - posa + lenb },
							va/vb
						)
					} else { panic!(); }
				} else { panic!(); }
			},

			Token::Modulo(ref v) => {
				if v.len() != 2 {panic!()};
				let a = v[0].as_number();
				let b = v[1].as_number();

				if let Token::Number(la, va) = a {
					if let Token::Number(lb, vb) = b {
						let LineLocation{pos: posa, ..} = la;
						let LineLocation{pos: posb, len: lenb} = lb;
						Token::Number(
							LineLocation { pos: posa, len: posb - posa + lenb },
							va%vb
						)
					} else { panic!(); }
				} else { panic!(); }
			},

			Token::Power(ref v) => {
				if v.len() != 2 {panic!()};
				let a = v[0].as_number();
				let b = v[1].as_number();

				if let Token::Number(la, va) = a {
					if let Token::Number(lb, vb) = b {
						let LineLocation{pos: posa, ..} = la;
						let LineLocation{pos: posb, len: lenb} = lb;
						Token::Number(
							LineLocation { pos: posa, len: posb - posa + lenb },
							va.powf(vb)
						)
					} else { panic!(); }
				} else { panic!(); }
			},

			Token::Factorial(ref _v) => { todo!() },
			_ => panic!()
		}
	}
}


/// Operator types, in order of increasing priority.
/// The Null operator MUST be equal to zero.
#[derive(Debug)]
#[derive(Copy, Clone)]
pub enum Operator {
	ModuloLong = 0, // Mod invoked with "mod"
	Subtract,
	Add,
	Divide,
	Multiply,
	ImplicitMultiply,
	Modulo, // Mod invoked with %
	Power,

	Negative,
	Factorial
}

/// Specifies the location of a token in an input string.
/// Used to locate ParserErrors.
#[derive(Debug)]
#[derive(Copy, Clone)]
pub struct LineLocation {
	pub pos: usize,
	pub len: usize
}

/// Types of parser errors.
/// If we cannot parse a string, one of these is returned.
#[derive(Debug)]
pub enum ParserError {
	InvalidChar,
	MissingCloseParen,
	ExtraCloseParen,
	EmptyGroup,
	Syntax,
	BadNumber 
}


/// Parse a user string. This is the only method that should be used
/// outside this module.
/// 
/// # Arguments:
/// `s`: the string to parse. Must be trimmed.
/// 
/// # Returns:
/// - `Err(LineLocation, ParserError)` if we couldn't parse this string.
/// `LineLocation` specifies *where* the error is, and `ParserError` specifies
/// *what* the error is.
/// 
/// - `Ok(Token)` otherwise, where `Token` is the top of an expression tree.
pub fn parse(s: &String) -> Result<Token, (LineLocation, ParserError)> {

	let mut g: Token = tokenize(s)?;
	g = treeify(g)?;
	g = evaluate(g)?;

	return Ok(g);
}