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
use crate::quantity::Unit;

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
			PreToken::PreNumber(l, mut s) => {

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
				return Ok(Token::Number(r.unwrap()));
			},

			PreToken::PreWord(l, s) => {
				let c = match &s[..] {
					// Mathematical constants
					// 100 digits of each.
					"π"|"pi" => { Some((Quantity::new_float_from_string(
						"3.141592653589793238462643383279502884197169399375105820974944592307816406286208998628034825342117067"
					).unwrap(), String::from("π")))},

					"e" => { Some((Quantity::new_float_from_string(
						"2.718281828459045235360287471352662497757247093699959574966967627724076630353547594571382178525166427"
					).unwrap(), String::from("e"))) },

					"phi"|"φ" => { Some((Quantity::new_float_from_string(
						"1.618033988749894848204586834365638117720309179805762862135448622705260462818902449707207204189391137"
					).unwrap(), String::from("φ"))) },

					_ => { None }
				};

				if c.is_some() {
					let (a, b) = c.unwrap();
					return Ok(Token::Constant(a, b));
				}

				let c = Quantity::from_unit_string(&s);

				if c.is_some() { return Ok(Token::Number(c.unwrap())); }

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
	s: &String, // The string to substitute
	c: usize    // Location of the cursor right now
) -> (
	usize,  // Location of cursor in substituted string
	String  // String with substitutions
) {
	if s == "" { return (c, s.clone()) }
	let mut new_s = s.clone();

	let l = s.chars().count();
	let tokens = tokenize(s);
	let (subs, _) = find_subs(tokens);
	let mut new_c = l - c;

	for r in subs.iter() {
		// find_subs gives substitutions in reverse order.

		if { // Don't substitute if our cursor is inside the substitution
			c >= r.0.pos &&
			c < r.0.pos+r.0.len
		} { continue; }

		if c < r.0.pos {
			let ct = r.1.chars().count();
			if ct >= r.0.len {
				if new_c >= ct - r.0.len {
					new_c += ct - r.0.len
				}
			} else {
				new_c -= r.0.len - ct
			}
		}

		new_s.replace_range(
			r.0.pos..r.0.pos+r.0.len,
			&r.1[..]
		)
	}

	return (new_c, new_s);
}