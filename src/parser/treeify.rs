
use std::collections::VecDeque;

use crate::parser::PreToken;
use crate::parser::ParserError;

use crate::tokens::Token;
use crate::tokens::Operator;
use crate::tokens::LineLocation;



fn treeify_binary(
	i: usize,
	g_inner: &mut VecDeque<PreToken>,
	left_associative: bool
) -> Result<(), (LineLocation, ParserError)> {

	let this: &PreToken = &g_inner[i];

	if i == 0 {
		// This binary operator is at the end of an expression.
		let l = match this {
			PreToken::PreOperator(l, _) => l,
			_ => panic!()
		};
		return Err((*l, ParserError::Syntax));
	}

	let next: &PreToken;
	if left_associative {
		next = {
			if i < g_inner.len()-1 {
				&g_inner[i+1]
			} else {
				let l = match this {
					PreToken::PreOperator(l, _) => l,
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
					PreToken::PreOperator(l, _) => l,
					_ => panic!()
				};
				return Err((*l, ParserError::Syntax));
			}
		};
	}



	if let PreToken::PreOperator(l, s) = next {
		let o = Operator::from_string(s);
		if o.is_none() { return Err((*l, ParserError::Syntax)); }
		let o = o.unwrap();

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
		let this_val = {
			let PreToken::PreOperator(l, s) = this else {panic!()};
			let o = Operator::from_string(s);
			if o.is_none() { return Err((*l, ParserError::Syntax)); }
			o.unwrap() as isize
		};

		// Precedence of the operators contesting our arguments
		let left_val = if i > 1 {
			let PreToken::PreOperator(l, s) = &g_inner[i-2] else {panic!()};
			let o = Operator::from_string(s);
			if o.is_none() { return Err((*l, ParserError::Syntax)); }
			Some(o.unwrap() as isize)
		} else { None };

		let right_val = if i < g_inner.len()-2 {
			let PreToken::PreOperator(l, s) = &g_inner[i+2] else {panic!()};
			let o = Operator::from_string(s);
			if o.is_none() { return Err((*l, ParserError::Syntax)); }
			Some(o.unwrap() as isize)
		} else { None };

		if {
			(left_val.is_none() || this_val >= left_val.unwrap()) &&
			(right_val.is_none() || this_val >= right_val.unwrap())
		} {
			// This operator has higher precedence, it takes both arguments
			let left_pre = g_inner.remove(i-1).unwrap();
			let this_pre = g_inner.remove(i-1).unwrap();
			let right_pre = g_inner.remove(i-1).unwrap();
			let left: Token; let right: Token;
			if let PreToken::PreGroup(_, _) = right_pre { right = treeify(right_pre)?; } else {right = right_pre.to_token()?;}
			if let PreToken::PreGroup(_, _) = left_pre { left = treeify(left_pre)?; } else {left = left_pre.to_token()?;}

			let (l, o) = {
				let PreToken::PreOperator(l, s) = this_pre else {panic!()};
				let o = Operator::from_string(&s);
				if o.is_none() { panic!() }
				(l, o.unwrap())
			};

			let mut new_token_args: VecDeque<Token> = VecDeque::with_capacity(2);
			new_token_args.push_back(left);
			new_token_args.push_back(right);

			g_inner.insert(i-1, PreToken::Container(o.into_token(l, new_token_args)));

			return Ok(());
		} else {
			return Ok(());
		};
	};
}

fn treeify_unary(
	i: usize,
	g_inner: &mut VecDeque<PreToken>,
	left_associative: bool
) -> Result<(), (LineLocation, ParserError)> {

	let this: &PreToken = &g_inner[i];
	let next: &PreToken;
	if left_associative {
		next = {
			if i > 0 {
				&g_inner[i-1]
			} else {
				let l = match this {
					PreToken::PreOperator(l, _) => l,
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
					PreToken::PreOperator(l, _) => l,
					_ => panic!()
				};
				return Err((*l, ParserError::Syntax));
			}
		};
	}

	// We need to check the element after unary operators too.
	// Bad syntax like `3!3` won't be caught otherwise.
	let prev: Option<&PreToken>;
	if left_associative {
		prev = if i < g_inner.len()-1 { Some(&g_inner[i+1]) } else {None};
	} else {
		prev = if i > 0 { Some(&g_inner[i-1]) } else {None};
	}

	if prev.is_some() {
		if let PreToken::PreOperator(l, s) = prev.unwrap() {
			let o = Operator::from_string(s);
			if o.is_none() { return Err((*l, ParserError::Syntax)); }
			let o = o.unwrap();

			if o.is_left_associative() && left_associative {
				return Err((*l, ParserError::Syntax));
			}
		} else {
			return Err((
				*this.get_line_location(),
				ParserError::Syntax
			));
		}
	}



	if let PreToken::PreOperator(l, _) = next {
		let tl = *this.get_line_location();
		return Err((
			LineLocation{pos: tl.pos, len: l.pos - tl.pos + l.len},
			ParserError::Syntax
		));

	} else {

		// Precedence of this operator
		let this_val = {
			let PreToken::PreOperator(l, s) = this else {panic!()};
			let o = Operator::from_string(s);
			if o.is_none() { return Err((*l, ParserError::Syntax)); }
			o.unwrap() as isize
		};

		// Precedence of the operator contesting its argument
		let next_val = if left_associative {
			if i > 1 {
				let PreToken::PreOperator(l, s) = &g_inner[i-2] else {panic!()};
				let o = Operator::from_string(s);
				if o.is_none() { return Err((*l, ParserError::Syntax)); }
				Some(o.unwrap() as isize)
			} else { None }
		} else {
			if i < g_inner.len()-2 {
				let PreToken::PreOperator(l, s) = &g_inner[i+2] else {panic!()};
				let o = Operator::from_string(s);
				if o.is_none() { return Err((*l, ParserError::Syntax)); }
				Some(o.unwrap() as isize)
			} else { None }
		};

		if next_val.is_none() || this_val > next_val.unwrap() {
			let this_pre = g_inner.remove(i).unwrap();
			let next_pre: PreToken; let next: Token;
			if left_associative {
				next_pre = g_inner.remove(i-1).unwrap();
			} else {
				next_pre = g_inner.remove(i).unwrap();
			}
			if let PreToken::PreGroup(_, _) = next_pre { next = treeify(next_pre)?; } else { next = next_pre.to_token()? }

			let (l, o) = {
				let PreToken::PreOperator(l, s) = this_pre else {panic!()};
				let o = Operator::from_string(&s);
				if o.is_none() { panic!() }
				(l, o.unwrap())
			};

			let mut new_token_args: VecDeque<Token> = VecDeque::with_capacity(3);
			new_token_args.push_back(next);

			if left_associative {
				g_inner.insert(i-1, PreToken::Container(o.into_token(l, new_token_args)));
			} else {
				g_inner.insert(i, PreToken::Container(o.into_token(l, new_token_args)));
			}

			return Ok(());
		} else {
			// The operator to the right has higher precedence.
			// Move on, don't to anything yet.
			return Ok(());
		};
	};
}


pub(in crate::parser) fn treeify(
	mut g: PreToken,
) -> Result<Token, (LineLocation, ParserError)> {

	let g_inner: &mut VecDeque<PreToken> = match g {
		PreToken::PreGroup(_, ref mut x) => x,
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
			PreToken::PreOperator(l, s) => {
				let o = Operator::from_string(&s);
				if o.is_none() { return Err((*l, ParserError::Syntax)); }
				o.unwrap()
			},
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


	let g = g_inner.pop_front().unwrap();
	return match g {
		// Catch edge cases
		PreToken::PreOperator(l, _) => {
			Err((l, ParserError::Syntax))
		},
		PreToken::PreGroup(_,_) => {
			treeify(g)
		},
		
		_ => { Ok(g.to_token()?) }
	};
}
