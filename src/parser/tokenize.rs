use std::collections::VecDeque;

use crate::parser::Token;
use crate::parser::LineLocation;
use crate::parser::ParserError;

#[inline(always)]
fn update_line_location(mut t: Token, stop_i: usize) -> Token {
	match t {
		Token::PreGroup(ref mut l, _) |
		Token::PreOperator(ref mut l, _) |
		Token::PreNumber(ref mut l, _) |
		Token::PreWord(ref mut l, _) => {
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
				if t.is_some() { g_now.push_back(update_line_location(t.unwrap(), i)); t = None; }
				g_now.push_back(
					Token::PreOperator(
						LineLocation{pos: i, len: 1},
						String::from("!")
					)
				);
			},

			// Minus sign can be both a Negative and an Operator.
			// Needs special treatment.
			'-' => {
				if t.is_some() { g_now.push_back(update_line_location(t.unwrap(), i)); t = None; }
				match g_now.back() {
					// If previous token was any of the following,
					// this is the "minus" operator
					Some(Token::PreNumber(_, _)) |
					Some(Token::PreGroup(_, _)) |
					Some(Token::PreWord(_, _)) => {
						g_now.push_back(
							Token::PreOperator(
								LineLocation{pos: i, len: 1},
								String::from(c)
							)
						);
					},

					// Otherwise, this is a negative sign.
					_ => {
						g_now.push_back(
							Token::PreOperator(
								LineLocation{pos: i, len: 1},
								String::from("neg")
							)
						);
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
						if t.is_some() { g_now.push_back(update_line_location(t.unwrap(), i)); }
						t = Some(Token::PreNumber(LineLocation{pos: i, len: 0}, String::from(c)));
					}
				};
			},

			// Word
			'A'..='Z' |
			'a'..='z' => {
				match &mut t {
					// If we're already building a number,
					// append.
					Some(Token::PreWord(_, val)) => {
						val.push(c);
					},

					// If we're not building a number, finalize
					// previous token and start one.
					_ => {
						if t.is_some() { g_now.push_back(update_line_location(t.unwrap(), i)); }
						t = Some(Token::PreWord(LineLocation{pos: i, len: 0}, String::from(c)));
					}
				};
			},


			// Operation
			// Always one character
			'+' |
			'*' |
			'/' |
			'^' |
			'%' => {
				// Finalize previous token
				if t.is_some() { g_now.push_back(update_line_location(t.unwrap(), i)); t = None; }
				g_now.push_back(Token::PreOperator(LineLocation{pos: i, len: 1}, String::from(c)));
			}
			
			// Groups
			// Always one character
			'(' => {
				if t.is_some() { g_now.push_back(update_line_location(t.unwrap(), i)); t = None; }
				g.push(Token::PreGroup(LineLocation{pos: i, len: 0}, VecDeque::with_capacity(8)));
			},
			')' => {
				if t.is_some() { g_now.push_back(update_line_location(t.unwrap(), i)); t = None; }
				let new_group: Token = g.pop().unwrap();

				let g_now: &mut VecDeque<Token> = match g.last_mut().unwrap() {
					Token::PreGroup(_, ref mut x) => x,
					_ => panic!()
				};
		
				g_now.push_back(update_line_location(new_group, i+1));
			},

			// Space. Basic seperator.
			' ' => {
				if t.is_some() { g_now.push_back(update_line_location(t.unwrap(), i)); t = None; }
			}

			// Invalid token
			_ => { return Err((LineLocation{pos: i, len: 1}, ParserError::InvalidChar)); }
		};
	}

	
	let g_now: &mut VecDeque<Token> = match g.last_mut().unwrap() {
		Token::PreGroup(_, ref mut x) => x,
		_ => panic!()
	};
	if t.is_some() { g_now.push_back(update_line_location(t.unwrap(), input.len())); }

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