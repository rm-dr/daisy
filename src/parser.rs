mod tokenize;
mod treeify;

use crate::parser::tokenize::tokenize;
use crate::parser::treeify::treeify;

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
	PreOperator(LineLocation, Operators),

	/// Used only until operators are parsed.
	/// PreGroups aren't needed once we have a tree.
	PreGroup(LineLocation, VecDeque<Token>),


	Number(LineLocation, f64),
	Multiply(VecDeque<Token>),
	Divide(VecDeque<Token>),
	Add(VecDeque<Token>),
	Subtract(VecDeque<Token>),
	Factorial(VecDeque<Token>),
	Negative(VecDeque<Token>),
	Power(VecDeque<Token>),
	Modulo(VecDeque<Token>),
}


/// Operator types, in order of increasing priority.
/// The Null operator MUST be equal to zero.
#[derive(Debug)]
#[derive(Copy, Clone)]
pub enum Operators {
	Null = 0,
	ModuloLong, // Mod invoked with "mod"
	Subtract,
	Add,
	Divide,
	Multiply,
	ImplicitMultiply,
	Modulo, // Mod invoked with %
	Power,
	Negative,
	Factorial,
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
	Syntax,
	InvalidImplicitMultiply,
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
	treeify(&mut g)?;
	
	return Ok(g);
}