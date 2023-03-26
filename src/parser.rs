mod tokenize;
mod treeify;
mod groupify;
mod evaluate;
mod find_subs;

use crate::parser::tokenize::p_tokenize;
use crate::parser::groupify::p_groupify;
use crate::parser::treeify::p_treeify;
use crate::parser::evaluate::p_evaluate;
use crate::parser::find_subs::p_find_subs;

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

	PreGroupStart(LineLocation),
	PreGroupEnd(LineLocation),
	/// Used only until operators are parsed.
	/// PreGroups aren't needed once we have a tree.
	PreGroup(LineLocation, VecDeque<Token>),

	Number(LineLocation, f64),
	Constant(LineLocation, f64, String),

	Multiply(VecDeque<Token>),
	Divide(VecDeque<Token>),
	Add(VecDeque<Token>),
	Factorial(VecDeque<Token>),
	Negative(VecDeque<Token>),
	Power(VecDeque<Token>),
	Modulo(VecDeque<Token>),
}

impl Token {

	#[inline(always)]
	pub fn get_args(&mut self) -> Option<&mut VecDeque<Token>> {
		match self {
			Token::Multiply(ref mut v)
			| Token::Divide(ref mut v)
			| Token::Add(ref mut v)
			| Token::Factorial(ref mut v)
			| Token::Negative(ref mut v)
			| Token::Power(ref mut v)
			| Token::Modulo(ref mut v)
			=> Some(v),
			_ => None
		}
	}

	#[inline(always)]
	pub fn get_line_location(&self) -> &LineLocation {
		match self {
			Token::PreNumber(l, _) |
			Token::PreWord(l, _) |
			Token::PreOperator(l, _) |
			Token::PreGroupStart(l) |
			Token::PreGroupEnd(l) |
			Token::PreGroup(l, _)
			=> l,

			// These have a line location, but we shouldn't ever need to get it.
			Token::Number(_l, _) |
			Token::Constant(_l, _, _)
			=> panic!(),
			_ => panic!()
		}
	}

	#[inline(always)]
	pub fn get_mut_line_location(&mut self) -> &mut LineLocation {
		match self {
			Token::PreNumber(l, _) |
			Token::PreWord(l, _) |
			Token::PreOperator(l, _) |
			Token::PreGroupStart(l) |
			Token::PreGroupEnd(l) |
			Token::PreGroup(l, _)
			=> l,

			// These have a line location, but we shouldn't ever need to get it.
			Token::Number(_l, _) |
			Token::Constant(_l, _, _)
			=> panic!(),
			_ => panic!()
		}
	}

	#[inline(always)]
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
						if new_pos == 0 {new_pos = l.pos};
						new_len = new_len + l.len;
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
						if new_pos == 0 {new_pos = l.pos};
						new_len = new_len + l.len;
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
						Token::Number(
							LineLocation { pos: la.pos, len: lb.pos - la.pos + lb.len },
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
						Token::Number(
							LineLocation { pos: la.pos, len: lb.pos - la.pos + lb.len },
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
						Token::Number(
							LineLocation { pos: la.pos, len: lb.pos - la.pos + lb.len },
							va.powf(vb)
						)
					} else { panic!(); }
				} else { panic!(); }
			},

			Token::Factorial(ref _v) => { todo!() },
			_ => self.as_number()
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
	//MissingCloseParen,
	ExtraCloseParen,
	EmptyGroup,
	Syntax,
	Undefined(String),
	BadNumber
}

impl ParserError {
	pub fn to_message(&self) -> String {
		match self {
			//ParserError::MissingCloseParen => {
			//	String::from("This group is never closed")
			//},
			ParserError::ExtraCloseParen => {
				String::from("Extra close parenthesis")
			},
			ParserError::EmptyGroup => {
				String::from("Groups can't be empty")
			},
			ParserError::Syntax => {
				String::from("Syntax")
			},
			ParserError::Undefined(s) => {
				format!("\"{s}\" isn't defined")
			},
			ParserError::BadNumber => {
				String::from("Invalid number")
			}
		}
	}
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
pub fn evaluate(s: &String) -> Result<Token, (LineLocation, ParserError)> {

	let tokens = p_tokenize(s);
	let (_, tokens) = p_find_subs(tokens);
	let mut g = p_groupify(tokens)?;
	g = p_treeify(g)?;
	g = p_evaluate(g)?;

	return Ok(g);
}


pub fn substitute(s: &String) -> String{
	if s == "" { return s.clone() }
	let mut new_s = s.clone();

	let tokens = p_tokenize(s);
	let (subs, _) = p_find_subs(tokens);

	for r in subs.iter() {
		new_s.replace_range(
			r.0.pos..r.0.pos+r.0.len,
			&r.1[..]
		)
	}

	return new_s;
}