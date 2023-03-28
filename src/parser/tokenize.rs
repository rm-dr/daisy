use std::collections::VecDeque;

use crate::parser::PreToken;
use crate::tokens::LineLocation;

/// Updates the length of a Token's LineLocation.
/// Run whenever a token is finished.
#[inline(always)]
fn update_line_location(mut t: PreToken, stop_i: usize) -> PreToken {
	match t {
		PreToken::PreGroupStart(ref mut l) |
		PreToken::PreGroupEnd(ref mut l) |
		PreToken::PreOperator(ref mut l, _) |
		PreToken::PreNumber(ref mut l, _) |
		PreToken::PreWord(ref mut l, _)
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
pub(in crate::parser) fn tokenize(input: &String) -> VecDeque<PreToken> {
	let mut t: Option<PreToken> = None; // The current token we're reading
	let mut g: VecDeque<PreToken> = VecDeque::with_capacity(32); 


	for (i, c) in input.chars().enumerate() {
		match c {
			// The minus sign can be both a Negative and an Operator.
			// Needs special treatment.
			'-' => {
				if t.is_some() { g.push_back(update_line_location(t.unwrap(), i)); }
				match g.back().as_ref() {
					// If previous token was any of the following,
					// this is the "minus" operator
					Some(PreToken::PreNumber(_, _)) |
					Some(PreToken::PreGroupEnd(_)) |
					Some(PreToken::PreWord(_, _)) => {
						t = Some(PreToken::PreOperator(
							LineLocation{pos: i, len: 1},
							String::from("-")
						));
					},

					// Otherwise, this is a negative sign.
					_ => {
						t = Some(PreToken::PreOperator(
							LineLocation{pos: i, len: 1},
							String::from("neg")
						));
					}
				};
			},

			// Number
			// Commas act just like dots.
			',' | '.' | '0'..='9' => {
				match &mut t {
					// If we're already building a number,
					// append.
					Some(PreToken::PreNumber(_, val)) => {
						val.push(if c == ',' {'.'} else {c});
					},

					// If we're not building a number, finalize
					// previous token and start one.
					_ => {
						if t.is_some() { g.push_back(update_line_location(t.unwrap(), i)); }
						t = Some(PreToken::PreNumber(LineLocation{pos: i, len: 0}, String::from(c)));
					}
				};
			},

			// Operator
			'*'|'×'|'/'|'÷'|
			'+'|'^'|'!'|'%'
			=> {
				match &mut t {
					Some(PreToken::PreOperator(_, val)) => { val.push(c); },
					_ => {
						if t.is_some() { g.push_back(update_line_location(t.unwrap(), i)); }
						t = Some(PreToken::PreOperator(LineLocation{pos: i, len: 0}, String::from(c)));
					}
				};
			},
			
			// Group
			'(' => {
				if t.is_some() { g.push_back(update_line_location(t.unwrap(), i)); }
				t = Some(PreToken::PreGroupStart(LineLocation{pos: i, len: 0}));
			},
			')' => {
				if t.is_some() { g.push_back(update_line_location(t.unwrap(), i)); }
				t = Some(PreToken::PreGroupEnd(LineLocation{pos: i, len: 0}));
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
					Some(PreToken::PreWord(_, val)) => { val.push(c); },

					_ => {
						if t.is_some() { g.push_back(update_line_location(t.unwrap(), i)); }
						t = Some(PreToken::PreWord(LineLocation{pos: i, len: 0}, String::from(c)));
					}
				};
			}
		};
	}

	if t.is_some() { g.push_back(update_line_location(t.unwrap(), input.chars().count())); }

	return g;
}