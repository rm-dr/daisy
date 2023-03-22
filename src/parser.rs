mod tokenize;
mod replace_pre;
mod fold_operators;
mod unwrap_groups;

use crate::parser::tokenize::tokenize;
use crate::parser::replace_pre::replace_pre;
use crate::parser::fold_operators::fold_operators;
use crate::parser::unwrap_groups::unwrap_groups;

use std::collections::VecDeque;


#[derive(Debug)]
pub enum Token {

	// Used only while tokenizing.
	// All of these are replaced with one of the tokens below.
	//
	// If parsing is successful,
	//  - all PreGroups will vanish
	//  - all PreOperators will become Operators
	//  - all PreNumbers will become Numbers
	PreGroup(LineLocation, VecDeque<Token>),
	PreOperator(LineLocation, String),
	PreNumber(LineLocation, String),
	PreWord(LineLocation, String),

	Number(f64),

	// Operators
	Multiply(VecDeque<Token>),
	Divide(VecDeque<Token>),
	Add(VecDeque<Token>),
	Subtract(VecDeque<Token>),
	Factorial(VecDeque<Token>),
	Negative(VecDeque<Token>),
	Power(VecDeque<Token>),
	Modulo(VecDeque<Token>),
}

#[derive(Debug)]
#[derive(Copy, Clone)]
pub struct LineLocation {
	pub pos: usize,
	pub len: usize
}

#[derive(Debug)]
pub enum ParserError {
	InvalidChar,
	MissingCloseParen,
	Syntax,
	BadNumber  // Cannot parse a number
}



pub fn parse(s: &String) -> Result<Token, (LineLocation, ParserError)> {

	let mut g: Token = tokenize(s)?;
	replace_pre(&mut g)?;
	fold_operators(&mut g)?;
	unwrap_groups(&mut g)?;
	
	return Ok(g);
}