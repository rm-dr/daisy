use std::collections::VecDeque;

use super::super::{
	Token,
	LineLocation,
	ParserError,
	Operator
};


fn lookback_signs(
	g: &mut VecDeque<Token>
) -> Result<(), (LineLocation, ParserError)> {

	// Convert `-` operators to `neg` operators
	// Delete `+`s that mean "positive" instead of "add"
	let mut i: usize = 0;
	while i < g.len() {
		if i == 0 {
			let a: Token = g.remove(i).unwrap();
			match &a {
				Token::PreOperator(l,o)
				=> {
					if o == "-" {
						g.insert(i, Token::PreOperator(*l, String::from("neg")));
					} else if o == "+" {
						continue; // We should not increment i if we remove a token
					} else {g.insert(i, a);}
				},
				_ => { g.insert(i, a); }
			};

		} else {
			let a: Token = g.remove(i-1).unwrap();
			let b: Token = g.remove(i-1).unwrap();

			match (&a, &b) {
				(Token::PreOperator(_, sa), Token::PreOperator(l,sb))
				=> {
					if {
						let o = Operator::from_string(sa);

						o.is_some() &&
						(
							o.as_ref().unwrap().is_binary() ||
							!o.as_ref().unwrap().is_left_associative()
						)
					} {
						if sb == "-" {
							g.insert(i-1, Token::PreOperator(*l, String::from("neg")));
							g.insert(i-1, a);
						} else if sb == "+" {
							g.insert(i-1, a);
							i -= 1; // g is now shorter, we don't need to advance i.
							// This nullifies the i += 1 at the end of the loop.
						} else { g.insert(i-1, b); g.insert(i-1, a); }
					} else { g.insert(i-1, b); g.insert(i-1, a); }
				},

				_ => { g.insert(i-1, b); g.insert(i-1, a); }
			}
		}

		i += 1;
	}

	// Delete consecutive `neg`s
	let mut i: usize = 1;
	while i < g.len() {
		let a: Token = g.remove(i-1).unwrap();
		let b: Token = g.remove(i-1).unwrap();

		match (&a, &b) {
			(Token::PreOperator(_,sa), Token::PreOperator(_,sb))
			=> {
				if !((sa == "neg") && (sb == "neg")) {
					g.insert(i-1, b);
					g.insert(i-1, a);
					i += 1;
				}
			},

			_ => {
				g.insert(i-1, b);
				g.insert(i-1, a);
				i += 1;
			}
		}

	}

	return Ok(());
}


// Inserts implicit operators
fn lookback(
	g: &mut VecDeque<Token>
) -> Result<(), (LineLocation, ParserError)> {

	lookback_signs(g)?;

	let mut i: usize = 0;
	while i < g.len() {
		if i >= 1 {
			let a: Token = g.remove(i-1).unwrap();
			let b: Token = g.remove(i-1).unwrap();

			match (&a, &b) {
				// Insert ImplicitMultiply
				(Token::PreGroup(_,_), Token::PreGroup(l ,_))
				| (Token::PreGroup(_,_), Token::PreQuantity(l,_))
				| (Token::PreQuantity(_,_), Token::PreGroup(l,_))
				| (Token::PreGroup(_,_), Token::PreWord(l,_))
				| (Token::PreWord(_,_), Token::PreGroup(l,_))
				| (Token::PreQuantity(_,_), Token::PreWord(l,_))
				| (Token::PreWord(_,_), Token::PreQuantity(l,_))
				| (Token::PreWord(_,_), Token::PreWord(l,_))
				=> {
					let loc = LineLocation{pos: l.pos-1, len: 0};

					g.insert(i-1, b);
					g.insert(i-1, Token::PreOperator(
						loc,
						String::from("i*")
					));
					g.insert(i-1, a);
				},

				// Insert implicit multiplications for right-unary operators
				(Token::PreQuantity(_,_), Token::PreOperator(l,s))
				| (Token::PreGroup(_,_), Token::PreOperator(l,s))
				| (Token::PreWord(_,_), Token::PreOperator(l,s))
				=> {
					let o = Operator::from_string(s);
					let loc = LineLocation{pos: l.pos-1, len: 0};

					g.insert(i-1, b);
					if o.is_some() {
						let o = o.unwrap();
						if (!o.is_binary()) && (!o.is_left_associative()) {
							g.insert(i-1, Token::PreOperator(
								loc,
								String::from("i*")
							));
						}
					}
					g.insert(i-1, a);
				},

				// Insert implicit multiplications for left-unary operators.
				(Token::PreOperator(_,s), Token::PreQuantity(l,_))
				| (Token::PreOperator(_,s), Token::PreGroup(l,_))
				| (Token::PreOperator(_,s), Token::PreWord(l,_))
				=> {
					let o = Operator::from_string(s);
					let loc = LineLocation{pos: l.pos-1, len: 0};

					g.insert(i-1, b);
					if o.is_some() {
						let o = o.unwrap();
						if (!o.is_binary()) && o.is_left_associative() {
							g.insert(i-1, Token::PreOperator(
								loc,
								String::from("i*")
							));
						}
					}
					g.insert(i-1, a);
				},

				// The following are syntax errors
				(Token::PreQuantity(la,_), Token::PreQuantity(lb,_))
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


pub fn groupify(
	mut g: VecDeque<Token>
) -> Result<
	Token,
	(LineLocation, ParserError)
> {
	// Vector of grouping levels
	let mut levels: Vec<(LineLocation, VecDeque<Token>)> = Vec::with_capacity(8);
	levels.push((LineLocation{pos: 0, len: 0}, VecDeque::with_capacity(8)));

	// Makes sure parenthesis are matched
	let mut i_level = 0;

	while g.len() > 0 {
		let t = g.pop_front().unwrap();
		let (l_now, v_now) = levels.last_mut().unwrap();

		match t {
			Token::PreGroupStart(l) => {
				levels.push((l, VecDeque::with_capacity(8)));
				i_level += 1;
			},

			Token::PreGroupEnd(l) => {
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

				v_now.push_back(Token::PreGroup(l, v));
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

		v_now.push_back(Token::PreGroup(l, v));
	}


	let (_, mut v) = levels.pop().unwrap();
	lookback(&mut v)?;

	return Ok(Token::PreGroup(LineLocation{pos:0, len:0}, v));
}