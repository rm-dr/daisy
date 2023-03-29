use std::collections::VecDeque;

use crate::parser::PreToken;
use crate::parser::LineLocation;
use crate::parser::ParserError;

use crate::tokens::Operator;

// Inserts implicit operators
fn lookback(
	g: &mut VecDeque<PreToken>
) -> Result<(), (LineLocation, ParserError)> {
	if g.len() == 1 {
		let a: PreToken = g.pop_back().unwrap();
		match &a {
			PreToken::PreOperator(l,o)
			=> {
				if o == "-" {
					g.push_back(PreToken::PreOperator(*l, String::from("neg")));
				} else { g.push_back(a); }
			},
			_ => { g.push_back(a); }
		};

	} else {
		let b: PreToken = g.pop_back().unwrap();
		let a: PreToken = g.pop_back().unwrap();

		match (&a, &b) {
			// Insert ImplicitMultiply
			(PreToken::PreGroup(_,_), PreToken::PreGroup(l ,_))
			| (PreToken::PreGroup(_,_), PreToken::PreNumber(l,_))
			| (PreToken::PreNumber(_,_), PreToken::PreGroup(l,_))
			| (PreToken::PreGroup(_,_), PreToken::PreWord(l,_))
			| (PreToken::PreWord(_,_), PreToken::PreGroup(l,_))
			| (PreToken::PreNumber(_,_), PreToken::PreWord(l,_))
			| (PreToken::PreWord(_,_), PreToken::PreNumber(l,_))
			| (PreToken::PreWord(_,_), PreToken::PreWord(l,_))
			=> {
				g.push_back(a);
				g.push_back(PreToken::PreOperator(
					LineLocation{pos: l.pos-1, len: 0},
					String::from("i*")
				));
				g.push_back(b);
			},

			// The following are syntax errors
			(PreToken::PreNumber(la,_), PreToken::PreNumber(lb,_))
			=> {
				return Err((
					LineLocation{pos: la.pos, len: lb.pos - la.pos + lb.len},
					ParserError::Syntax
				));
			}

			(PreToken::PreOperator(_, sa), PreToken::PreOperator(l,sb))
			=> {
				if sb == "-" && {
					let o = Operator::from_string(sa);

					o.is_some() &&
					(
						o.as_ref().unwrap().is_binary() ||
						!o.as_ref().unwrap().is_left_associative()
					)
				} {
					g.push_back(a);
					g.push_back(PreToken::PreOperator(*l, String::from("neg")));
				} else { g.push_back(a); g.push_back(b); }
			}

			// Insert implicit multiplications for unary operators
			(PreToken::PreNumber(_,_), PreToken::PreOperator(l,s))
			| (PreToken::PreGroup(_,_), PreToken::PreOperator(l,s))
			| (PreToken::PreWord(_,_), PreToken::PreOperator(l,s))
			=> {
				let o = Operator::from_string(s);
				g.push_back(a);
				if o.is_some() {
					let o = o.unwrap();
					if (!o.is_binary()) && (!o.is_left_associative()) {
						g.push_back(PreToken::PreOperator(
							LineLocation{pos: l.pos-1, len: 0},
							String::from("i*")
						));
					}
				}
				g.push_back(b);
			},

			(PreToken::PreOperator(_,s), PreToken::PreNumber(l,_))
			| (PreToken::PreOperator(_,s), PreToken::PreGroup(l,_))
			| (PreToken::PreOperator(_,s), PreToken::PreWord(l,_))
			=> {
				let o = Operator::from_string(s);
				g.push_back(a);
				if o.is_some() {
					let o = o.unwrap();
					if (!o.is_binary()) && o.is_left_associative() {
						g.push_back(PreToken::PreOperator(
							LineLocation{pos: l.pos-1, len: 0},
							String::from("i*")
						));
					}
				}
				g.push_back(b);
			},

			// This shouldn't ever happen.
			(PreToken::PreGroupStart(_), _)
			| (_, PreToken::PreGroupStart(_))
			| (PreToken::PreGroupEnd(_), _)
			| (_, PreToken::PreGroupEnd(_))
			| (PreToken::Container(_), _)
			| (_, PreToken::Container(_))
			=> panic!()
		}
	};
	return Ok(());
}


pub(in crate::parser) fn groupify(
	mut g: VecDeque<PreToken>
) -> Result<
	PreToken,
	(LineLocation, ParserError)
> {
	// Vector of grouping levels
	let mut levels: Vec<(LineLocation, VecDeque<PreToken>)> = Vec::with_capacity(8);
	levels.push((LineLocation{pos: 0, len: 0}, VecDeque::with_capacity(8)));

	// Makes sure parenthesis are matched
	let mut i_level = 0;

	while g.len() > 0 {
		let t = g.pop_front().unwrap();
		let (l_now, v_now) = levels.last_mut().unwrap();

		match t {
			PreToken::PreGroupStart(l) => {
				levels.push((l, VecDeque::with_capacity(8)));
				i_level += 1;
			},

			PreToken::PreGroupEnd(l) => {
				let l = LineLocation {
					pos: l_now.pos,
					len: l.len + l.pos - l_now.pos
				};

				if i_level == 0 { return Err((l, ParserError::ExtraCloseParen)) }
				if v_now.len() == 0 { return Err((l, ParserError::EmptyGroup)) }

				i_level -= 1;

				let (_, v) = levels.pop().unwrap();
				let (_, v_now) = levels.last_mut().unwrap();

				v_now.push_back(PreToken::PreGroup(l, v));
				lookback(v_now)?;
			},

			_ => {
				v_now.push_back(t);
				lookback(v_now)?;
			}
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

		if v.len() == 0 { return Err((l, ParserError::EmptyGroup)) }

		v_now.push_back(PreToken::PreGroup(l, v));
		lookback(v_now)?;
	}


	let (_, v) = levels.pop().unwrap();
	return Ok(PreToken::PreGroup(LineLocation{pos:0, len:0}, v));
}