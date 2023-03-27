mod tokenize;
mod treeify;
mod groupify;
mod find_subs;


use crate::parser::tokenize::p_tokenize;
use crate::parser::groupify::p_groupify;
use crate::parser::treeify::p_treeify;
use crate::parser::find_subs::p_find_subs;

use crate::tokens;

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
	tokens::Token,
	(tokens::LineLocation, ParserError)
> {

	let tokens = p_tokenize(s);
	let (_, tokens) = p_find_subs(tokens);
	let mut g = p_groupify(tokens)?;
	g = p_treeify(g)?;

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