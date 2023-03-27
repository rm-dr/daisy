use std::collections::VecDeque;

use crate::tokens::Token;
use crate::tokens::Operator;
use crate::tokens::LineLocation;

use crate::parser::ParserError;

fn treeify_binary(
	i: usize,
	g_inner: &mut VecDeque<Token>,
	left_associative: bool
) -> Result<(), (LineLocation, ParserError)> {

	let this: &Token = &g_inner[i];

	if i == 0 {
		// This binary operator is at the end of an expression.
		let l = match this {
			Token::PreOperator(l, _) => l,
			_ => panic!()
		};
		return Err((*l, ParserError::Syntax));
	}

	let next: &Token;
	if left_associative {
		next = {
			if i < g_inner.len()-1 {
				&g_inner[i+1]
			} else {
				let l = match this {
					Token::PreOperator(l, _) => l,
					_ => panic!()
				};
				return Err((*l, ParserError::Syntax));
			}
		};
	} else {
		next = {
			if i > 0 {
				&g_inner[i-1]
			} else {
				let l = match this {
					Token::PreOperator(l, _) => l,
					_ => panic!()
				};
				return Err((*l, ParserError::Syntax));
			}
		};
	}



	if let Token::PreOperator(l, o) = next {
		if {
			(!o.is_binary()) &&
			!(o.is_left_associative() && left_associative)
		} {
			// Only right-associative unary operators can follow a binary operator
			return Ok(());
		} else {
			let tl = *this.get_line_location();
			return Err((
				LineLocation{pos: tl.pos, len: l.pos - tl.pos + l.len},
				ParserError::Syntax
			));
		}
	} else {

		// Precedence of this operator
		let this_val: isize = match this {
			Token::PreOperator(_, q) => *q as isize,
			_ => panic!()
		};

		// Precedence of the operators contesting our arguments
		let left_val = if i > 1 {
			match &g_inner[i-2] {
				Token::PreOperator(_, q) => Some(*q as isize),
				_ => panic!()
			}
		} else { None };
		let right_val = if i < g_inner.len()-2 {
			match &g_inner[i+2] {
				Token::PreOperator(_, q) => Some(*q as isize),
				_ => panic!()
			}
		} else { None };

		if {
			(left_val.is_none() || this_val >= left_val.unwrap()) &&
			(right_val.is_none() || this_val >= right_val.unwrap())
		} {
			// This operator has higher precedence, it takes both arguments
			let mut left = g_inner.remove(i-1).unwrap();
			let this = g_inner.remove(i-1).unwrap();
			let mut right = g_inner.remove(i-1).unwrap();
			if let Token::PreGroup(_, _) = right { right = p_treeify(right)?; }
			if let Token::PreGroup(_, _) = left { left = p_treeify(left)?; }

			let k = match this {
				Token::PreOperator(_, k) => k,
				_ => panic!()
			};

			let mut new_token_args: VecDeque<Token> = VecDeque::with_capacity(3);
			new_token_args.push_back(left);
			new_token_args.push_back(right);

			g_inner.insert(i-1, k.into_token(new_token_args));

			return Ok(());
		} else {
			return Ok(());
		};
	};
}


fn treeify_unary(
	i: usize,
	g_inner: &mut VecDeque<Token>,
	left_associative: bool
) -> Result<(), (LineLocation, ParserError)> {

	let this: &Token = &g_inner[i];
	let next: &Token;
	if left_associative {
		next = {
			if i > 0 {
				&g_inner[i-1]
			} else {
				let l = match this {
					Token::PreOperator(l, _) => l,
					_ => panic!()
				};
				return Err((*l, ParserError::Syntax));
			}
		};
	} else {
		next = {
			if i < g_inner.len()-1 {
				&g_inner[i+1]
			} else {
				let l = match this {
					Token::PreOperator(l, _) => l,
					_ => panic!()
				};
				return Err((*l, ParserError::Syntax));
			}
		};
	}

	// We need to check the element after unary operators too.
	// Bad syntax like `3!3` won't be caught otherwise.
	let prev: Option<&Token>;
	if left_associative {
		prev = if i < g_inner.len()-1 { Some(&g_inner[i+1]) } else {None};

	} else {
		prev = if i > 0 { Some(&g_inner[i-1]) } else {None};

	}

	if prev.is_some() {
		if let Token::PreOperator(l, o) = prev.unwrap() {
			match o {
				// Left unary operators
				Operator::Negative => {
					let tl = *this.get_line_location();
					return Err((
						LineLocation{pos: tl.pos, len: l.pos - tl.pos + l.len},
						ParserError::Syntax
					));
				},
				_ => {},
			};
		} else {
			return Err((
				*this.get_line_location(),
				ParserError::Syntax
			));
		}
	}



	if let Token::PreOperator(l, _) = next {
		let tl = *this.get_line_location();
		return Err((
			LineLocation{pos: tl.pos, len: l.pos - tl.pos + l.len},
			ParserError::Syntax
		));
	} else {

		// Precedence of this operator
		let this_val: isize = match this {
			Token::PreOperator(_, q) => *q as isize,
			_ => panic!()
		};

		// Precedence of the operator contesting its argument
		let next_val = if left_associative {
			if i > 1 {
				match &g_inner[i-2] {
					Token::PreOperator(_, q) => Some(*q as isize),
					_ => panic!()
				}
			} else { None }
		} else {
			if i < g_inner.len()-2 {
				match &g_inner[i+2] {
					Token::PreOperator(_, q) => Some(*q as isize),
					_ => panic!()
				}
			} else { None }
		};

		if next_val.is_none() || this_val > next_val.unwrap() {
			let this = g_inner.remove(i).unwrap();
			let mut next;
			if left_associative {
				next = g_inner.remove(i-1).unwrap();
			} else {
				next = g_inner.remove(i).unwrap();
			}
			if let Token::PreGroup(_, _) = next { next = p_treeify(next)?; }

			let k = match this {
				Token::PreOperator(_, k) => k,
				_ => panic!()
			};

			let mut new_token_args: VecDeque<Token> = VecDeque::with_capacity(3);
			new_token_args.push_back(next);

			if left_associative {
				g_inner.insert(i-1, k.into_token(new_token_args));
			} else {
				g_inner.insert(i, k.into_token(new_token_args));
			}

			return Ok(());
		} else {
			// The operator to the right has higher precedence.
			// Move on, don't to anything yet.
			return Ok(());
		};
	};
}

pub fn p_treeify(
	mut g: Token,
) -> Result<Token, (LineLocation, ParserError)> {

	let g_inner: &mut VecDeque<Token> = match g {
		Token::PreGroup(_, ref mut x) => x,
		_ => panic!()
	};

	let mut left_associative = true;
	let mut j: i64 = 0;
	while g_inner.len() > 1 {


		if j <= -1 {
			left_associative = true;
			j = 0;
		} else if j >= g_inner.len() as i64 {
			left_associative = false;
			j = (g_inner.len() - 1) as i64;
		}

		let i = j as usize;
		let this_op = match &g_inner[i] {
			Token::PreOperator(_, o) => o,
			_ => {
				if left_associative { j += 1 } else { j -= 1 };
				continue;
			}
		};

		if left_associative {
			if this_op.is_left_associative() {
				if this_op.is_binary() {
					treeify_binary(i, g_inner, left_associative)?;
				} else {
					treeify_unary(i, g_inner, left_associative)?;
				}
			}
			j += 1 
		} else {
			if !this_op.is_left_associative() {
				if this_op.is_binary() {
					treeify_binary(i, g_inner, left_associative)?;
				} else {
					treeify_unary(i, g_inner, left_associative)?;
				}
			}
			j -= 1
		}
	}

	g = g_inner.pop_front().unwrap();

	// Catch edge cases
	match g {
		Token::PreOperator(l, _) => {
			return Err((l, ParserError::Syntax));
		},
		Token::PreGroup(_,_) => {
			g = p_treeify(g)?;
		}
		_ => {}
	};

	return Ok(g);
}