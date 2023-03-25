use std::collections::VecDeque;

use crate::parser::Token;
use crate::parser::LineLocation;
use crate::parser::ParserError;
use crate::parser::Operator;

#[inline(always)]
fn select_op(k: Operator, mut new_token_args: VecDeque<Token>) -> Token {
	match k {
		Operator::Subtract => {
			let a = new_token_args.pop_front().unwrap();
			let b = new_token_args.pop_front().unwrap();

			Token::Add(
			VecDeque::from(vec!(
					a,
					Token::Negative(VecDeque::from(vec!(b)))
			)))
		},
		Operator::Add => Token::Add(new_token_args),
		Operator::Divide => Token::Divide(new_token_args),
		Operator::Multiply => Token::Multiply(new_token_args),
		Operator::ImplicitMultiply => Token::Multiply(new_token_args),
		Operator::Modulo => Token::Modulo(new_token_args),
		Operator::ModuloLong => Token::Modulo(new_token_args),
		Operator::Power => Token::Power(new_token_args),
		Operator::Negative => Token::Negative(new_token_args),
		Operator::Factorial => Token::Factorial(new_token_args)
	}
}

fn treeify_binary(
	mut i: usize,
	g_inner: &mut VecDeque<Token>
) -> Result<usize, (LineLocation, ParserError)> {

	let this: &Token = &g_inner[i];

	if i == 0 {
		// This binary operator is at the end of an expression.
		let l = match this {
			Token::PreOperator(l, _) => l,
			_ => panic!()
		};
		return Err((*l, ParserError::Syntax));
	}

	let right: &Token = {
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


	if let Token::PreOperator(l, o) = right {
		match o {
			// Binary operators
			Operator::ModuloLong |
			Operator::Subtract |
			Operator::Add |
			Operator::Divide |
			Operator::Multiply |
			Operator::ImplicitMultiply |
			Operator::Modulo |
			Operator::Power |
			// Right unary operators
			Operator::Factorial
			=> {
				// Binary and right-unary operators cannot
				// follow a binary operator.
				let LineLocation { pos: posa, .. } = *this.get_line_location();
				let LineLocation { pos: posb, len: lenb } = *l;
				return Err((
					LineLocation{pos: posa, len: posb - posa + lenb},
					ParserError::Syntax
				));
			},

			// Left unary operators
			Operator::Negative => {
				i += 1;
				return Ok(i);
			}
		};
	} else {

		// Precedence of this operator
		let this_val: isize = match this {
			Token::PreOperator(_, q) => *q as isize,
			_ => panic!()
		};

		// Precedence of the operator contesting the right argument.
		let right_val = if i < g_inner.len()-2 {
			match &g_inner[i+2] {
				Token::PreOperator(_, q) => Some(*q as isize),
				_ => panic!()
			}
		} else { None };


		if right_val.is_none() || this_val > right_val.unwrap() {
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

			g_inner.insert(i-1, select_op(k, new_token_args));

			if i > 1 { i -= 2; } else { i = 0; }
			return Ok(i);
		} else {
			// The operator to the right has higher precedence.
			// Move on, don't to anything yet.
			i += 2;
			return Ok(i);
		};
	};
}


fn treeify_unaryleft(
	mut i: usize,
	g_inner: &mut VecDeque<Token>
) -> Result<usize, (LineLocation, ParserError)> {

	let this: &Token = &g_inner[i];
	let right: &Token = {
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


	if let Token::PreOperator(l, o) = right {
		match o {
			// Binary operators
			Operator::ModuloLong |
			Operator::Subtract |
			Operator::Add |
			Operator::Divide |
			Operator::Multiply |
			Operator::ImplicitMultiply |
			Operator::Modulo |
			Operator::Power |
			// Right unary operators
			Operator::Factorial
			=> {
				// Binary and right-unary operators cannot
				// follow a binary operator.
				let LineLocation { pos: posa, .. } = *this.get_line_location();
				let LineLocation { pos: posb, len: lenb } = *l;
				return Err((
					LineLocation{pos: posa, len: posb - posa + lenb},
					ParserError::Syntax
				));
			},

			// Left unary operators
			Operator::Negative => {
				i += 1;
				return Ok(i);
			}
		};
	} else {

		// Precedence of this operator
		let this_val: isize = match this {
			Token::PreOperator(_, q) => *q as isize,
			_ => panic!()
		};

		// Precedence of the operator contesting its argument
		let right_val = if i < g_inner.len()-2 {
			match &g_inner[i+2] {
				Token::PreOperator(_, q) => Some(*q as isize),
				_ => panic!()
			}
		} else { None };


		if right_val.is_none() || this_val > right_val.unwrap() {
			let this = g_inner.remove(i).unwrap();
			let mut right = g_inner.remove(i).unwrap();
			if let Token::PreGroup(_, _) = right { right = p_treeify(right)?; }

			let k = match this {
				Token::PreOperator(_, k) => k,
				_ => panic!()
			};

			let mut new_token_args: VecDeque<Token> = VecDeque::with_capacity(3);
			new_token_args.push_back(right);

			g_inner.insert(i, select_op(k, new_token_args));

			if i > 0 { i -= 1; } else { i = 0; }
			return Ok(i);
		} else {
			// The operator to the right has higher precedence.
			// Move on, don't to anything yet.
			i += 2;
			return Ok(i);
		};
	};
}

fn treeify_unaryright(
	mut i: usize,
	g_inner: &mut VecDeque<Token>
) -> Result<usize, (LineLocation, ParserError)> {

	let this: &Token = &g_inner[i];
	let left: &Token = {
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


	// We need to check the element after unary right operators too.
	// Bad syntax like `3!3` won't be caught otherwise.
	let right: Option<&Token> = {
		if i < g_inner.len()-1 {
			Some(&g_inner[i+1])
		} else {None}
	};

	if right.is_some() {
		if let Token::PreOperator(l, o) = right.unwrap() {
			match o {
				// Left unary operators
				Operator::Negative => {
					let LineLocation { pos: posa, .. } = *this.get_line_location();
					let LineLocation { pos: posb, len: lenb } = *l;
					return Err((
						LineLocation{pos: posa, len: posb - posa + lenb},
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

	if let Token::PreOperator(l, _) = left {
		let LineLocation { pos: posa, .. } = *this.get_line_location();
		let LineLocation { pos: posb, len: lenb } = *l;
		return Err((
			LineLocation{pos: posa, len: posb - posa + lenb},
			ParserError::Syntax
		));

	} else {

		// Precedence of this operator
		let this_val: isize = match this {
			Token::PreOperator(_, q) => *q as isize,
			_ => panic!()
		};

		// Precedence of the operator contesting its argument.
		let left_val = if i >= 2 {
			match &g_inner[i-2] {
				Token::PreOperator(_, q) => Some(*q as isize),
				_ => panic!()
			}
		} else { None };


		if left_val.is_none() || this_val > left_val.unwrap() {
			let this = g_inner.remove(i).unwrap();
			let mut left = g_inner.remove(i-1).unwrap();
			if let Token::PreGroup(_, _) = left { left = p_treeify(left)?; }

			let k = match this {
				Token::PreOperator(_, k) => k,
				_ => panic!()
			};

			let mut new_token_args: VecDeque<Token> = VecDeque::with_capacity(3);
			new_token_args.push_back(left);

			g_inner.insert(i-1, select_op(k, new_token_args));

			if i > 2 { i -= 2; } else { i = 0; }
			return Ok(i);
		} else {
			// The operator to the right has higher precedence.
			// Move on, don't to anything yet.
			i += 1;
			return Ok(i);
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

	let mut i: usize = 0;
	while g_inner.len() > 1 {
		let this_op = match &g_inner[i] {
			Token::PreOperator(_, o) => o,
			_ => { i+=1; continue; }
		};

		match this_op {
			Operator::ModuloLong |
			Operator::Subtract |
			Operator::Add |
			Operator::Divide |
			Operator::Multiply |
			Operator::ImplicitMultiply |
			Operator::Modulo |
			Operator::Power
			=> { i = treeify_binary(i, g_inner)?; },

			Operator::Negative
			=> { i = treeify_unaryleft(i, g_inner)?; },

			Operator::Factorial
			=> { i = treeify_unaryright(i, g_inner)?; }

		};
	}

	g = g_inner.pop_front().unwrap();

	// Catch the edge case where the entire group we're given
	// consists of one operator. This is always a syntax error.
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