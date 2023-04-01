use std::collections::VecDeque;


mod tokenize;
mod treeify;
mod groupify;
mod find_subs;


use crate::parser::tokenize::tokenize;
use crate::parser::groupify::groupify;
use crate::parser::treeify::treeify;
use crate::parser::find_subs::find_subs;

use crate::quantity::Quantity;

use crate::tokens::Token;

/// Specifies the location of a token in an input string.
/// Used to locate ParserErrors.
#[derive(Debug)]
#[derive(Copy, Clone)]
pub struct LineLocation {
	pub pos: usize,
	pub len: usize
}

#[derive(Debug)]
enum PreToken {
	PreNumber(LineLocation, String),
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
			PreToken::PreNumber(l, _)
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
			PreToken::PreNumber(l, _)
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
			PreToken::PreNumber(l, s) => {
				let n: f64 = match s.parse() {
					Ok(n) => n,
					Err(_) => return Err((l, ParserError::BadNumber))
				};
				return Ok(Token::Number(Quantity::new_rational_from_f64(n).unwrap()));
			},
			PreToken::PreWord(l, s) => {
				return Ok(match &s[..] {
					// Mathematical constants
					// 100 digits of each.
					"π"|"pi" => { Token::Constant(Quantity::float_from_string("3.141592653589793238462643383279502884197169399375105820974944592307816406286208998628034825342117067"), String::from("π")) },
					"e" => { Token::Constant(Quantity::float_from_string("2.7182818284590452353602874713526624977572470936999595749669676277240766303535475945713 8217852516642"), String::from("e")) },
					"phi"|"φ" => { Token::Constant(Quantity::float_from_string("1.618033988749894848204586834365638117720309179805762862135448622705260462818902449707207204189391137"), String::from("φ")) },

					_ => { return Err((l, ParserError::Undefined(s))); }
				});
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



pub fn parse(
	s: &String
) -> Result<
	Token,
	(LineLocation, ParserError)
> {

	let tokens = tokenize(s);
	let (_, tokens) = find_subs(tokens);
	let g = groupify(tokens)?;
	let g = treeify(g)?;

	return Ok(g);
}


pub fn substitute(
	s: &String, // The string to subsitute
	c: usize    // Location of the cursor right now
) -> String{
	if s == "" { return s.clone() }
	let mut new_s = s.clone();

	let tokens = tokenize(s);
	let (subs, _) = find_subs(tokens);

	for r in subs.iter() {
		if { // Don't subsitute if our cursor is inside the substitution
			c >= r.0.pos &&
			c < r.0.pos+r.0.len
		} { continue; }

		new_s.replace_range(
			r.0.pos..r.0.pos+r.0.len,
			&r.1[..]
		)
	}

	return new_s;
}