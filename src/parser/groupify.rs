use std::collections::VecDeque;

use crate::parser::Token;
use crate::parser::LineLocation;
use crate::parser::ParserError;
use crate::parser::Operator;

/// Looks backwards at the elements of g.
/// - Inserts ImplicitMultiply
/// - Removes multiple PreNegatives
/// - Applies PreNegative to Numbers
/// - Parses factorials
/// - Checks syntax
#[inline(always)]
fn lookback(
	g: &mut VecDeque<Token>
) -> Result<(), (LineLocation, ParserError)> {
	if g.len() >= 2 {
		let b: Token = g.pop_back().unwrap();
		let a: Token = g.pop_back().unwrap();

		match (&a, &b) {
			// Insert ImplicitMultiply
			(Token::PreGroup(_,_), Token::PreGroup(l ,_)) |
			(Token::PreGroup(_,_), Token::Number(l,_)) |
			(Token::Number(_,_), Token::PreGroup(l,_)) |
			(Token::Constant(_,_,_), Token::Number(l,_)) |
			(Token::Number(_,_), Token::Constant(l,_,_)) |
			(Token::Constant(_,_,_), Token::PreGroup(l,_)) |
			(Token::PreGroup(_,_), Token::Constant(l,_,_)) |
			(Token::Constant(_,_,_), Token::Constant(l,_,_))
			=> {
				g.push_back(a);
				g.push_back(Token::PreOperator(
					LineLocation{pos: l.pos-1, len: 0},
					Operator::ImplicitMultiply
				));
				g.push_back(b);
			},

			// The following are syntax errors
			(Token::Number(la, _), Token::Number(lb,_))
			=> {
				return Err((
					LineLocation{pos: la.pos, len: lb.pos - la.pos + lb.len},
					ParserError::Syntax
				));
			}

			// The following are fine
			(Token::PreOperator(_,_), _) |
			(_, Token::PreOperator(_,_))
			=> { g.push_back(a); g.push_back(b); },

			// If we get this far, we found a Token
			// that shouldn't be here.
			_ => panic!()
		}
	};
	return Ok(());
}


pub fn p_groupify(mut g: VecDeque<Token>) -> Result<Token, (LineLocation, ParserError)> {
	// Vector of grouping levels
	let mut levels: Vec<(LineLocation, VecDeque<Token>)> = Vec::with_capacity(8);
	levels.push((LineLocation{pos: 0, len: 0}, VecDeque::with_capacity(8)));

	// Makes sure parenthesis are matched
	let mut i_level = 0;

	while g.len() > 0 {
		let t = g.pop_front().unwrap();
		let (l_now, v_now) = levels.last_mut().unwrap();

		match t {
			Token::PreOperator(_, _) => {
				v_now.push_back(t);
				lookback(v_now)?;
			},

			Token::PreNumber(l, s) => {
				let n = match s.parse() {
					Ok(n) => n,
					Err(_) => return Err((l, ParserError::BadNumber))
				};
				v_now.push_back(Token::Number(l, n));
				lookback(v_now)?;
			},

			Token::PreWord(l, s) => {
				// This method must support both plain text and
				// unicode versions of each word.
				v_now.push_back(match &s[..] {
					"mod" => { Token::PreOperator(l, Operator::ModuloLong) },
					"π"|"pi" => { Token::Constant(l, 3.141592653, String::from("π")) },
					_ => { return Err((l, ParserError::Undefined(s))); }
				});
				lookback(v_now)?;
			},

			Token::PreGroupStart(l) => {
				levels.push((l, VecDeque::with_capacity(8)));
				i_level += 1;
			},

			Token::PreGroupEnd(l) => {
				let l = LineLocation {
					pos: l_now.pos,
					len: l.len + l.pos - l_now.pos
				};

				if i_level == 0 {
					return Err((l, ParserError::ExtraCloseParen))
				}
				i_level -= 1;

				// Catch empty groups
				if v_now.len() == 0 {
					return Err((l, ParserError::EmptyGroup))
				}

				let (_, v) = levels.pop().unwrap();
				let (_, v_now) = levels.last_mut().unwrap();

				v_now.push_back(Token::PreGroup(l, v));
				lookback(v_now)?;
			},

			_ => panic!()
		}
	}

	/*
	// Error on missing parenthesis
	if levels.len() != 1 {
		let (l, _) = levels.pop().unwrap();
		return Err((l, ParserError::MissingCloseParen))
	}
	*/

	// Auto-close parenthesis
	while levels.len() != 1 {
		let (l, v) = levels.pop().unwrap();
		let (_, v_now) = levels.last_mut().unwrap();

		// Catch empty groups
		if v.len() == 0 {
			return Err((l, ParserError::EmptyGroup))
		}

		v_now.push_back(Token::PreGroup(l, v));
		lookback(v_now)?;
	}


	let (_, v) = levels.pop().unwrap();
	return Ok(Token::PreGroup(LineLocation{pos:0, len:0}, v));
}