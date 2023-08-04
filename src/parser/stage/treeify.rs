use std::collections::VecDeque;
use crate::context::Context;
use crate::errors::DaisyError;

use super::super::{
	Token,
	LineLocation,
	Expression,
	Operator
};

fn treeify_binary(
	i: usize,
	g_inner: &mut VecDeque<Token>,
	context: &Context
) -> Result<bool, (LineLocation, DaisyError)> {

	let this: &Token = &g_inner[i];

	if i == 0 {
		// This binary operator is at the end of an expression.
		let l = match this {
			Token::Operator(l, _) => l,
			_ => panic!()
		};
		return Err((*l, DaisyError::Syntax)); // left argument is empty
	}


	let left = {
		if i > 0 {
			&g_inner[i-1]
		} else {
			let l = match this {
				Token::Operator(l, _) => l,
				_ => panic!()
			};
			return Err((*l, DaisyError::Syntax)); // Left argument is empty
		}
	};

	let right = {
		if i < g_inner.len()-1 {
			&g_inner[i+1]
		} else {
			let l = match this {
				Token::Operator(l, _) => l,
				_ => panic!()
			};
			return Err((*l, DaisyError::Syntax)); // right argument is empty
		}
	};





	if let Token::Operator(l, s) = left {
		let o = Operator::from_string(s);
		if o.is_none() { return Err((*l, DaisyError::Syntax)); } // Bad string
		let o = o.unwrap();

		if {
			(!o.is_binary()) &&
			o.is_left_associative()
		} {
			return Ok(false);
		} else {
			let tl = *this.get_line_location() + *l;
			return Err((tl, DaisyError::Syntax)); // left argument isn't valid
		}
	}

	if let Token::Operator(l, s) = right {
		let o = Operator::from_string(s);
		if o.is_none() { return Err((*l, DaisyError::Syntax)); } // Bad string
		let o = o.unwrap();

		if {
			(!o.is_binary()) &&
			!o.is_left_associative()
		} {
			return Ok(false);
		} else {
			let tl = *this.get_line_location() + *l;
			return Err((tl, DaisyError::Syntax)); // right argument isn't valid (two operators next to each other)
		}
	}


	// This operator
	let this_op = {
		let Token::Operator(l, s) = this else {panic!()};
		let o = Operator::from_string(s);
		if o.is_none() { return Err((*l, DaisyError::Syntax)); } // bad operator string
		o.unwrap()
	};

	// The operators contesting our arguments
	let left_op = if i > 1 {
		let Token::Operator(l, s) = &g_inner[i-2] else {panic!()};
		let o = Operator::from_string(s);
		if o.is_none() { return Err((*l, DaisyError::Syntax)); } // Bad operator string
		Some(o.unwrap())
	} else { None };

	let right_op = if i < g_inner.len()-2 {
		let Token::Operator(l, s) = &g_inner[i+2] else {panic!()};
		let o = Operator::from_string(s);
		if o.is_none() { return Err((*l, DaisyError::Syntax)); } // Bad operator string
		Some(o.unwrap())
	} else { None };


	if {
		(left_op.is_none() || this_op >= left_op.unwrap()) &&
		(right_op.is_none() || this_op >= right_op.unwrap())
	} {
		// This operator has higher precedence, it takes both arguments
		let left_pre = g_inner.remove(i-1).unwrap();
		let this_pre = g_inner.remove(i-1).unwrap();
		let right_pre = g_inner.remove(i-1).unwrap();
		let mut left: Expression; let mut right: Expression;
		if let Token::Group(l, _) = right_pre {
			right = treeify(right_pre, context)?;
			right.set_linelocation(&(right.get_linelocation() + l));
		} else {
			right = right_pre.to_expression(context)?;
		}

		if let Token::Group(l, _) = left_pre {
			left = treeify(left_pre, context)?;
			left.set_linelocation(&(left.get_linelocation() + l));
		} else {
			left = left_pre.to_expression(context)?;
		}

		let (l, o) = {
			let Token::Operator(l, s) = this_pre else {panic!()};
			let o = Operator::from_string(&s);
			if o.is_none() { panic!() }
			(l, o.unwrap())
		};

		let mut new_token_args: VecDeque<Expression> = VecDeque::with_capacity(2);
		new_token_args.push_back(left);
		new_token_args.push_back(right);

		g_inner.insert(i-1, Token::Container(Expression::Operator(l, o, new_token_args)));

		return Ok(true);
	} else {
		return Ok(false);
	};
}

fn treeify_unary(
	i: usize,
	g_inner: &mut VecDeque<Token>,
	left_associative: bool,
	context: &Context
) -> Result<bool, (LineLocation, DaisyError)> {

	let this: &Token = &g_inner[i];
	let next: &Token;
	if left_associative {
		next = {
			if i > 0 {
				&g_inner[i-1]
			} else {
				let l = match this {
					Token::Operator(l, _) => l,
					_ => panic!()
				};
				return Err((*l, DaisyError::Syntax)); // argument is missing
			}
		};
	} else {
		next = {
			if i < g_inner.len()-1 {
				&g_inner[i+1]
			} else {
				let l = match this {
					Token::Operator(l, _) => l,
					_ => panic!()
				};
				return Err((*l, DaisyError::Syntax)); // argument is missing
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
		if let Token::Operator(_,_) = prev.unwrap() {
		} else {
			// Previous operator is invalid
			return Err((
				*this.get_line_location(),
				DaisyError::Syntax
			));
		}
	}

	if let Token::Operator(l, _) = next {
		let tl = *this.get_line_location() + *l;
		// Argument is invalid
		return Err((tl, DaisyError::Syntax));
	} else {

		// This operator
		let this_op = {
			let Token::Operator(l, s) = this else {panic!()};
			let o = Operator::from_string(s);
				if o.is_none() { return Err((*l, DaisyError::Syntax)); } // Bad string
			o.unwrap()
		};

		// The operator contesting our argument
		let next_op = if left_associative {
			if i > 1 {
				let Token::Operator(l, s) = &g_inner[i-2] else {panic!()};
				let o = Operator::from_string(s);
				if o.is_none() { return Err((*l, DaisyError::Syntax)); } // Bad string
				Some(o.unwrap())
			} else { None }
		} else {
			if i < g_inner.len()-2 {
				let Token::Operator(l, s) = &g_inner[i+2] else {panic!()};
				let o = Operator::from_string(s);
				if o.is_none() { return Err((*l, DaisyError::Syntax)); } // Bad string
				Some(o.unwrap())
			} else { None }
		};

		if next_op.is_none() || this_op > next_op.unwrap() {
			let this_pre = g_inner.remove(i).unwrap();
			let next_pre: Token; let mut next: Expression;
			if left_associative {
				next_pre = g_inner.remove(i-1).unwrap();
			} else {
				next_pre = g_inner.remove(i).unwrap();
			}
			if let Token::Group(l, _) = next_pre {
				next = treeify(next_pre, context)?;
				next.set_linelocation(&(next.get_linelocation() + l));
			} else {
				next = next_pre.to_expression(context)?;
			}


			let (l, o) = {
				let Token::Operator(l, s) = this_pre else {panic!()};
				let o = Operator::from_string(&s);
				if o.is_none() { panic!() }
				(l, o.unwrap())
			};

			let mut new_token_args: VecDeque<Expression> = VecDeque::with_capacity(3);
			new_token_args.push_back(next);

			if left_associative {
				g_inner.insert(i-1, Token::Container(Expression::Operator(l, o, new_token_args)));
			} else {
				g_inner.insert(i, Token::Container(Expression::Operator(l, o, new_token_args)));
			}

			return Ok(true);
		} else {
			// The operator to the right has higher precedence.
			// Move on, don't to anything yet.
			return Ok(false);
		};
	};
}


pub fn treeify(
	mut g: Token,
	context: &Context
) -> Result<Expression, (LineLocation, DaisyError)> {
	let (l, g_inner): (LineLocation, &mut VecDeque<Token>) = match g {
		Token::Group(l, ref mut x) => (l, x),
		_ => panic!()
	};

	if g_inner.len() == 0 {
		// This shouldn't ever happen.
		return Err((l, DaisyError::EmptyGroup));
	}

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

		// Convert operators
		// If not an operator, move on.
		let this_op = match &g_inner[i] {
			Token::Operator(l, s) => {
				let o = Operator::from_string(&s);
				if o.is_none() { return Err((*l, DaisyError::Syntax)); }
				o.unwrap()
			},
			_ => {
				if left_associative { j += 1 } else { j -= 1 };
				continue;
			}
		};

		if left_associative {
			let mut changed = false;
			if this_op.is_left_associative() {
				if this_op.is_binary() {
					changed = treeify_binary(i, g_inner, context)?;
				} else {
					changed = treeify_unary(i, g_inner, left_associative, context)?;
				}
			}

			// We only need to change j if we don't treeify.
			// If the array length changes, j will point to the next
			// element automatically.
			if !changed { j += 1; }

		} else {
			if !this_op.is_left_associative() {
				if this_op.is_binary() {
					treeify_binary(i, g_inner, context)?;
				} else {
					treeify_unary(i, g_inner, left_associative, context)?;
				}
			}
			j -= 1
		}
	}


	let g = g_inner.pop_front().unwrap();
	return match g {
		// Catch edge cases
		Token::Operator(l, _) => {
			Err((l, DaisyError::Syntax))
		},
		Token::Group(_,_) => {
			treeify(g, context)
		},

		_ => { Ok(g.to_expression(context)?) }
	};
}
