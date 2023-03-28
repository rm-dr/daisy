use std::collections::VecDeque;

use crate::parser::PreToken;
use crate::parser::LineLocation;

use crate::tokens::Operator;

// Called whenever a token is finished.
#[inline(always)]
fn push_token(g: &mut VecDeque<PreToken>, t: Option<PreToken>, stop_i: usize) {
	
	if t.is_none() { return }
	let mut t = t.unwrap();

	match t {
		PreToken::PreGroupStart(ref mut l) 
		| PreToken::PreGroupEnd(ref mut l)
		| PreToken::PreOperator(ref mut l, _)
		| PreToken::PreNumber(ref mut l, _)
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


	if let PreToken::PreWord(l, s) = &t {
		let o = Operator::from_string(s);
		if o.is_some() {
			t = PreToken::PreOperator(*l, s.clone());
		}
	}

	g.push_back(t);
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
				push_token(&mut g, t, i);
				t = Some(PreToken::PreOperator(
					LineLocation{pos: i, len: 1},
					String::from("-")
				));
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
						push_token(&mut g, t, i);
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