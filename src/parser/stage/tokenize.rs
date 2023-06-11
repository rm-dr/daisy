use std::collections::VecDeque;

use super::super::{
	PreToken,
	LineLocation,
	Operator
};

// Called whenever a token is finished.
#[inline(always)]
fn push_token(g: &mut VecDeque<PreToken>, t: Option<PreToken>, stop_i: usize) {

	if t.is_none() { return }
	let mut t = t.unwrap();

	match t {
		PreToken::PreGroupStart(ref mut l)
		| PreToken::PreGroupEnd(ref mut l)
		| PreToken::PreOperator(ref mut l, _)
		| PreToken::PreQuantity(ref mut l, _)
		| PreToken::PreWord(ref mut l, _)
		=> {
			*l = LineLocation{
				pos: l.pos,
				len: stop_i - l.pos,
			};
		},

		PreToken::PreGroup(_,_)
		| PreToken::Container(_)
		=> panic!()
	};


	// `2e` isn't exponential notation, it's 2*e.
	// If a number ends in `e`, disconnect the `e` and make it a word.
	if let PreToken::PreQuantity(l, s) = &t {
		let last = &s[s.len()-1..];
		if last == "e" {
			g.push_back(PreToken::PreQuantity(
				LineLocation { pos: l.pos, len: l.len-1 },
				String::from(&s[0..s.len()-1])
			));
			g.push_back(PreToken::PreWord(
				LineLocation { pos: l.pos + l.len - 1, len: 1 },
				String::from("e")
			));

			return;
		}
	}

	if let PreToken::PreWord(l, s) = &t {
		let o = Operator::from_string(s);
		if o.is_some() {
			t = PreToken::PreOperator(*l, s.clone());
		}
	}

	g.push_back(t);
}

/// Turns a string into Tokens. First stage of parsing.
pub fn tokenize(input: &String) -> VecDeque<PreToken> {
	let mut t: Option<PreToken> = None; // The current token we're reading
	let mut g: VecDeque<PreToken> = VecDeque::with_capacity(32);


	for (i, c) in input.chars().enumerate() {
		match c {
			// Number
			// Commas act just like dots.
			',' | '.' | '0'..='9' => {
				match &mut t {
					// If we're already building a number,
					// append.
					Some(PreToken::PreQuantity(_, val)) => {
						val.push(if c == ',' {'.'} else {c});
					},

					// If we're not building a number, finalize
					// previous token and start one.
					_ => {
						push_token(&mut g, t, i);
						t = Some(PreToken::PreQuantity(LineLocation{pos: i, len: 0}, String::from(c)));
					}
				};
			},

			// 'e' needs special treatment.
			// Can be both a word or a number.
			'e' => {
				match &mut t {
					Some(PreToken::PreWord(_, val)) => { val.push(c); },
					Some(PreToken::PreQuantity(_, val)) => { val.push(c); },

					_ => {
						push_token(&mut g, t, i);
						t = Some(PreToken::PreWord(LineLocation{pos: i, len: 0}, String::from(c)));
					}
				};
			}

			// The minus sign also needs special treatment.
			// It can be the `neg` operator, the `minus` operator,
			// or it can specify a negative exponent.
			'-' | '+' => {
				match &mut t {
					Some(PreToken::PreQuantity(_, val)) => {
						if &val[val.len()-1..] == "e" {
							// If the current number ends in an `e`,
							// this negative specifies a negative exponent
							// like 2e-2 = 0.02.
							val.push(c);
						} else {
							// Otherwise, end the number.
							// We probably have a subtraction.
							push_token(&mut g, t, i);
							t = Some(PreToken::PreOperator(
								LineLocation{pos: i, len: 1},
								String::from(c)
							));
						}
					},

					// This may be a negative or a subtraction
					_ => {
						push_token(&mut g, t, i);
						t = Some(PreToken::PreOperator(
							LineLocation{pos: i, len: 1},
							String::from(c)
						));
					}
				};
			},

			// Operator
			'*'|'×'|'/'|'÷'|
			'^'|'!'|'%'
			=> {
				match &mut t {
					Some(PreToken::PreOperator(_, val)) => { val.push(c); },
					_ => {
						push_token(&mut g, t, i);
						t = Some(PreToken::PreOperator(LineLocation{pos: i, len: 0}, String::from(c)));
					}
				};
			},

			// Group
			'(' => {
				push_token(&mut g, t, i);
				t = Some(PreToken::PreGroupStart(LineLocation{pos: i, len: 0}));
			},
			')' => {
				push_token(&mut g, t, i);
				t = Some(PreToken::PreGroupEnd(LineLocation{pos: i, len: 0}));
			},

			// Space. Basic seperator.
			' ' => {
				push_token(&mut g, t, i);
				t = None;
			}

			// Word
			_ => {
				match &mut t {
					Some(PreToken::PreWord(_, val)) => { val.push(c); },

					_ => {
						push_token(&mut g, t, i);
						t = Some(PreToken::PreWord(LineLocation{pos: i, len: 0}, String::from(c)));
					}
				};
			}
		};
	}

	push_token(&mut g, t, input.chars().count());

	return g;
}