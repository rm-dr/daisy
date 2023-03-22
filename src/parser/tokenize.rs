use std::collections::VecDeque;

use crate::parser::Token;
use crate::parser::LineLocation;
use crate::parser::ParserError;
use crate::parser::Operators;

/// Updates the length of a Token's LineLocation.
/// Run whenever a token is finished.
#[inline(always)]
fn update_line_location(mut t: Token, stop_i: usize) -> Token {
	match t {
		Token::PreGroup(ref mut l, _) |
		Token::PreOperator(ref mut l, _) |
		Token::PreNumber(ref mut l, _) |
		Token::PreWord(ref mut l, _) |
		Token::PreNegative(ref mut l) |
		Token::PreFactorial(ref mut l)
		=> {
			let LineLocation{pos, .. } = l;
			*l = LineLocation{
				pos: *pos,
				len: stop_i - *pos,
			};
		},
		_ => panic!()
	};

	return t;
}


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

			( // Delete consecutive negatives
				Token::PreNegative(_),
				Token::PreNegative(_)
			) => {},

			// Insert ImplicitMultiply
			(Token::PreGroup(_,_), Token::PreGroup(l ,_)) |
			(Token::PreGroup(_,_), Token::Number(l,_)) |
			(Token::Number(_,_), Token::PreGroup(l,_))
			=> {
				g.push_back(a);
				let LineLocation { pos: i, .. } = l;
				g.push_back(Token::PreOperator(
					LineLocation{pos: i-1, len: 0},
					Operators::ImplicitMultiply
				));
				g.push_back(b);
			},

			// The following are syntax errors
			(Token::PreOperator(la,_), Token::PreOperator(lb,_)) |
			(Token::Number(la, _), Token::Number(lb,_)) |
			(Token::PreNegative(la), Token::PreOperator(lb,_)) |
			(Token::PreOperator(la, _), Token::PreFactorial(lb)) |
			(Token::PreNegative(la), Token::PreFactorial(lb))
			=> {
				let LineLocation { pos: posa, .. } = *la;
				let LineLocation { pos: posb, len: lenb } = *lb;
				return Err((
					LineLocation{pos: posa, len: posb - posa + lenb},
					ParserError::Syntax
				));
			}

			// The following are fine
			(Token::PreOperator(_,_), Token::PreNegative(_)) |
			(Token::PreOperator(_,_), Token::Number(_,_)) |
			(Token::Number(_,_), Token::PreOperator(_,_)) |
			(Token::PreOperator(_,_), Token::PreGroup(_,_)) |
			(Token::PreGroup(_,_), Token::PreOperator(_,_)) |
			(Token::PreNegative(_), Token::PreGroup(_,_)) |
			(Token::PreNegative(_), Token::Number(_,_)) |
			(Token::PreGroup(_,_), Token::PreFactorial(_)) |
			(Token::Number(_,_), Token::PreFactorial(_))
			=> { g.push_back(a); g.push_back(b); },

			// If we get this far, we found a Token
			// that shouldn't be here.
			_ => panic!()
		}
	};
	return Ok(());
}



/// Pushes (and potentially processes) a token we just read to a vector.
/// - Converts all `PreNumbers` to `Numbers`, returning a BadNumber error if necessary
/// - Converts all `PreWords` to other tokens.
fn push_token(
	g_now: &mut VecDeque<Token>,
	i: usize,
	t: Option<Token>
) -> Result<(), (LineLocation, ParserError)>{
	if t.is_none() {
		return Ok(());
	} else {
		let t: Token = update_line_location(t.unwrap(), i);
		g_now.push_back(match t {
			Token::PreNumber(l, s) => {
				let n = match s.parse() {
					Ok(n) => n,
					Err(_) => return Err((l, ParserError::BadNumber))
				};
				Token::Number(l, n)
			},
			Token::PreWord(l, s) => {
				if s == "mod" {
					Token::PreOperator(l, Operators::ModuloLong)
				} else {
					return Err((l, ParserError::Syntax));
				}
			},
			Token::PreOperator(_, _) => t,
			Token::PreGroup(_, _) => t,
			Token::PreNegative(_) => t,
			Token::PreFactorial(_) => t,
			_ => panic!()
		});

		lookback(g_now)?;
	}
	return Ok(());
}


/// Turns a string into Tokens. First stage of parsing.
pub fn tokenize(input: &String) -> Result<Token, (LineLocation, ParserError)> {
	let mut t: Option<Token> = None; // The current token we're reading
	let mut g: Vec<Token> = Vec::with_capacity(8); // Vector of "grouping levels"
	g.push(Token::PreGroup(LineLocation{pos: 0, len: 0}, VecDeque::with_capacity(8)));


	for (i, c) in input.chars().enumerate() {

		// The grouping level we're on now
		let g_now: &mut VecDeque<Token> = match g.last_mut().unwrap() {
			Token::PreGroup(_, ref mut x) => x,
			_ => panic!()
		};

		match c {
			'!' => {
				push_token(g_now, i, t)?;
				t = Some(Token::PreFactorial(LineLocation{pos: i, len: 1}));
			},

			// The minus sign can be both a Negative and an Operator.
			// Needs special treatment.
			'-' => {
				push_token(g_now, i, t)?;
				match g_now.back() {
					// If previous token was any of the following,
					// this is the "minus" operator
					Some(Token::Number(_, _)) |
					Some(Token::PreGroup(_, _)) |
					Some(Token::PreWord(_, _)) => {
						t = Some(Token::PreOperator(
							LineLocation{pos: i, len: 1},
							Operators::Subtract
						));
					},

					// Otherwise, this is a negative sign.
					_ => {
						t = Some(Token::PreNegative(LineLocation{pos: i, len: 1}));
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
						push_token(g_now, i, t)?;
						t = Some(Token::PreNumber(LineLocation{pos: i, len: 0}, String::from(c)));
					}
				};
			},

			// Word
			'A'..='Z' |
			'a'..='z' => {
				match &mut t {
					Some(Token::PreWord(_, val)) => {
						val.push(c);
					},

					_ => {
						push_token(g_now, i, t)?;
						t = Some(Token::PreWord(LineLocation{pos: i, len: 0}, String::from(c)));
					}
				};
			},

			// Operator
			// Always one character
			'+' | '*' | '/' | '^' | '%' => {
				push_token(g_now, i, t)?;
				t = Some(Token::PreOperator(
					LineLocation{pos: i, len: 1},
					match c {
						'^' => Operators::Power,
						'%' => Operators::Modulo,
						'*' => Operators::Multiply,
						'/' => Operators::Divide,
						'+' => Operators::Add,
						_ => panic!()
					}
				));
			}
			
			// Group
			'(' => {
				push_token(g_now, i, t)?; t = None;
				g.push(Token::PreGroup(LineLocation{pos: i, len: 0}, VecDeque::with_capacity(8)));
			},
			')' => {
				push_token(g_now, i, t)?;
				t = Some(g.pop().unwrap());
			},

			// Space. Basic seperator.
			' ' => {
				push_token(g_now, i, t)?; t = None;
			}

			// Invalid character
			_ => { return Err((LineLocation{pos: i, len: 1}, ParserError::InvalidChar)); }
		};
	}

	
	let g_now: &mut VecDeque<Token> = match g.last_mut().unwrap() {
		Token::PreGroup(_, ref mut x) => x,
		_ => panic!()
	};
	push_token(g_now, input.len(), t)?;

	if g.len() != 1 {
		let q: LineLocation = match g.last_mut().unwrap() {
			Token::PreGroup(l, _) => *l,
			_ => panic!()
		};

		let LineLocation{pos:p, ..} = q;
		return Err((
			LineLocation{
				pos: p,
				len: input.len() - p
			},
			ParserError::MissingCloseParen
		))
	}

	return Ok(g.pop().unwrap());
}