use std::collections::VecDeque;

use crate::tokens::Token;
use crate::tokens::Operator;
use crate::tokens::LineLocation;

/// Updates the length of a Token's LineLocation.
/// Run whenever a token is finished.
#[inline(always)]
fn update_line_location(mut t: Token, stop_i: usize) -> Token {
	match t {
		Token::PreGroupStart(ref mut l) |
		Token::PreGroupEnd(ref mut l) |
		Token::PreOperator(ref mut l, _) |
		Token::PreNumber(ref mut l, _) |
		Token::PreWord(ref mut l, _)
		=> {
			*l = LineLocation{
				pos: l.pos,
				len: stop_i - l.pos,
			};
		},
		_ => panic!()
	};

	return t;
}

/// Turns a string into Tokens. First stage of parsing.
pub fn p_tokenize(input: &String) -> VecDeque<Token> {
	let mut t: Option<Token> = None; // The current token we're reading
	let mut g: VecDeque<Token> = VecDeque::with_capacity(32); 


	for (i, c) in input.chars().enumerate() {
		match c {
			// The minus sign can be both a Negative and an Operator.
			// Needs special treatment.
			'-' => {
				if t.is_some() { g.push_back(update_line_location(t.unwrap(), i)); }
				match g.back().as_ref() {
					// If previous token was any of the following,
					// this is the "minus" operator
					Some(Token::PreNumber(_, _)) |
					Some(Token::PreGroup(_, _)) |
					Some(Token::PreWord(_, _)) => {
						t = Some(Token::PreOperator(
							LineLocation{pos: i, len: 1},
							Operator::Subtract
						));
					},

					// Otherwise, this is a negative sign.
					_ => {
						t = Some(Token::PreOperator(
							LineLocation{pos: i, len: 1},
							Operator::Negative
						));
					}
				};
			},

			// Number.
			// Commas act just like dots.
			',' | '.' | '0'..='9' => {
				match &mut t {
					// If we're already building a number,
					// append.
					Some(Token::PreNumber(_, val)) => {
						val.push(if c == ',' {'.'} else {c});
					},

					// If we're not building a number, finalize
					// previous token and start one.
					_ => {
						if t.is_some() { g.push_back(update_line_location(t.unwrap(), i)); }
						t = Some(Token::PreNumber(LineLocation{pos: i, len: 0}, String::from(c)));
					}
				};
			},

			// Operator
			// Always one character
			'*'|'×'|'/'|'÷'|
			'+'|'^'|'!'|'%'
			=> {
				if t.is_some() { g.push_back(update_line_location(t.unwrap(), i)); }
				t = Some(Token::PreOperator(
					LineLocation{pos: i, len: 0},
					match c {
						'^' => Operator::Power,
						'%' => Operator::Modulo,
						'*'|'×' => Operator::Multiply,
						'/'|'÷' => Operator::Divide,
						'+' => Operator::Add,
						'!' => Operator::Factorial,
						_ => panic!()
					}
				));
			}
			
			// Group
			'(' => {
				if t.is_some() { g.push_back(update_line_location(t.unwrap(), i)); }
				t = Some(Token::PreGroupStart(LineLocation{pos: i, len: 0}));
			},
			')' => {
				if t.is_some() { g.push_back(update_line_location(t.unwrap(), i)); }
				t = Some(Token::PreGroupEnd(LineLocation{pos: i, len: 0}));
			},

			// Space. Basic seperator.
			' ' => {
				if t.is_some() {
					g.push_back(update_line_location(t.unwrap(), i));
					t = None;
				}
			}

			// Word
			_ => {
				match &mut t {
					Some(Token::PreWord(_, val)) => {
						val.push(c);
					},

					_ => {
						if t.is_some() { g.push_back(update_line_location(t.unwrap(), i)); }
						t = Some(Token::PreWord(LineLocation{pos: i, len: 0}, String::from(c)));
					}
				};
			}
		};
	}

	if t.is_some() { g.push_back(update_line_location(t.unwrap(), input.chars().count())); }

	return g;
}