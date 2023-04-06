use std::collections::VecDeque;

use crate::parser::PreToken;
use crate::parser::LineLocation;
use crate::parser::ParserError;

use crate::tokens::Operator;


fn lookback_signs(
	g: &mut VecDeque<PreToken>
) -> Result<(), (LineLocation, ParserError)> {

	// Convert `-` operators to `neg` operators
	// Delete `+`s that mean "positive" instead of "add"
	let mut i: usize = 0;
	while i < g.len() {
		if i == 0 {
			let a: PreToken = g.remove(i).unwrap();
			match &a {
				PreToken::PreOperator(l,o)
				=> {
					if o == "-" {
						g.insert(i, PreToken::PreOperator(*l, String::from("neg")));
					} else if o != "+" { g.insert(i, a); }
				},
				_ => { g.insert(i, a); }
			};

		} else {
			let a: PreToken = g.remove(i-1).unwrap();
			let b: PreToken = g.remove(i-1).unwrap();
	
			match (&a, &b) {
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
						g.insert(i-1, PreToken::PreOperator(*l, String::from("neg")));
						g.insert(i-1, a);
					} else if sb == "+" {
						g.insert(i-1, a);
						i -= 1; // g is now shorter, we don't need to advance i.
						// This nullifies the i += 1 at the end of the loop.
					} else { g.insert(i-1, b); g.insert(i-1, a); }
				},

				_ => { g.insert(i-1, b); g.insert(i-1, a); }
			}
		}

		i += 1;
	}

	// Make negative numbers negative numbers.
	// We don't need `neg` operators in this case.
	// Note that we read the token array right-to-left, since `neg` is right-associative.
	let mut i: usize = g.len();
	loop {
		if i > 1 { i -= 1; } else { break; }

		let a: PreToken = g.remove(i-1).unwrap();
		let b: PreToken = g.remove(i-1).unwrap();

		match (&a, &b) {
			(PreToken::PreOperator(la,sa), PreToken::PreNumber(lb,sb))
			=> {
				if sa == "neg" {
					let first = &sb[0..1];
					if first == "-" {
						// Already negative. Remove the old one.
						g.insert(i-1,
							PreToken::PreNumber(
								LineLocation { pos: la.pos, len: lb.pos - la.pos + lb.len },
								format!("{}", &sb[1..])
							)
						);
					} else {
						// Not negative yet. Add one.
						g.insert(i-1,
							PreToken::PreNumber(
								LineLocation { pos: la.pos, len: lb.pos - la.pos + lb.len },
								format!("-{sb}")
							)
						);
					}
				} else { g.insert(i-1, b); g.insert(i-1, a); }
			},

			_ => { g.insert(i-1, b); g.insert(i-1, a); }
		}
	}

	return Ok(());
}


// Inserts implicit operators
fn lookback(
	g: &mut VecDeque<PreToken>
) -> Result<(), (LineLocation, ParserError)> {

	lookback_signs(g)?;


	let mut i: usize = 0;
	while i < g.len() {
		if i >= 1 {
			let a: PreToken = g.remove(i-1).unwrap();
			let b: PreToken = g.remove(i-1).unwrap();
	
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
					let loc = LineLocation{pos: l.pos-1, len: 0};

					g.insert(i-1, b);
					g.insert(i-1, PreToken::PreOperator(
						loc,
						String::from("i*")
					));
					g.insert(i-1, a);
				},
	
				// Insert implicit multiplications for right-unary operators
				(PreToken::PreNumber(_,_), PreToken::PreOperator(l,s))
				| (PreToken::PreGroup(_,_), PreToken::PreOperator(l,s))
				| (PreToken::PreWord(_,_), PreToken::PreOperator(l,s))
				=> {
					let o = Operator::from_string(s);
					let loc = LineLocation{pos: l.pos-1, len: 0};

					g.insert(i-1, b);
					if o.is_some() {
						let o = o.unwrap();
						if (!o.is_binary()) && (!o.is_left_associative()) {
							g.insert(i-1, PreToken::PreOperator(
								loc,
								String::from("i*")
							));
						}
					}
					g.insert(i-1, a);
				},
	
				// Insert implicit multiplications for left-unary operators.
				(PreToken::PreOperator(_,s), PreToken::PreNumber(l,_))
				| (PreToken::PreOperator(_,s), PreToken::PreGroup(l,_))
				| (PreToken::PreOperator(_,s), PreToken::PreWord(l,_))
				=> {
					let o = Operator::from_string(s);
					let loc = LineLocation{pos: l.pos-1, len: 0};

					g.insert(i-1, b);
					if o.is_some() {
						let o = o.unwrap();
						if (!o.is_binary()) && o.is_left_associative() {
							g.insert(i-1, PreToken::PreOperator(
								loc,
								String::from("i*")
							));
						}
					}
					g.insert(i-1, a);
				},

				// The following are syntax errors
				(PreToken::PreNumber(la,_), PreToken::PreNumber(lb,_))
				=> {
					return Err((
						LineLocation{pos: la.pos, len: lb.pos - la.pos + lb.len},
						ParserError::Syntax
					));
				},
				_ => {g.insert(i-1, b); g.insert(i-1, a);}
			}
		}

		i += 1;
	}

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

				let (_, mut v) = levels.pop().unwrap();
				let (_, v_now) = levels.last_mut().unwrap();
				lookback(&mut v)?;

				v_now.push_back(PreToken::PreGroup(l, v));
			},

			_ => {
				v_now.push_back(t);
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
		let (l, mut v) = levels.pop().unwrap();
		let (_, v_now) = levels.last_mut().unwrap();

		if v.len() == 0 { return Err((l, ParserError::EmptyGroup)) }
		lookback(&mut v)?;

		v_now.push_back(PreToken::PreGroup(l, v));
	}


	let (_, mut v) = levels.pop().unwrap();
	lookback(&mut v)?;

	return Ok(PreToken::PreGroup(LineLocation{pos:0, len:0}, v));
}