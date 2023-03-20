use std::collections::VecDeque;

use crate::parser::tokenize::Token;

fn treefold_one(
	exp: &mut Token, // Must be a group
	check: fn(&Token) -> bool,
	op_type: u8,
	new_token: fn(VecDeque<Token>) -> Token,
) -> Result<(), ()> {

	// Groups to process
	let mut t_vec: VecDeque<&mut Token> = VecDeque::with_capacity(32);
	t_vec.push_back(exp);

	while t_vec.len() > 0 {

		// The group we're currently working with
		let g: &mut Token = t_vec.pop_front().unwrap();
		let g_inner: &mut VecDeque<Token> = match g {
			Token::Group(ref mut x) => x,
			_ => panic!()
		};

		let mut new: VecDeque<Token> = VecDeque::with_capacity(8);

		// Build new group array
		while g_inner.len() > 0 {
			let t: Token = match g_inner.pop_front() {
				Some(o) => o,
				None => break
			};

			if check(&t) {
				match op_type {
					0 => {
						let mut last: Token = new.pop_back().unwrap();

						if let Token::Group(_) = last {
							treefold_one(&mut last, check, op_type, new_token).unwrap();
						}

						let mut new_token_args: VecDeque<Token> = VecDeque::with_capacity(1);
						new_token_args.push_back(last);
						new.push_back(new_token(new_token_args));
					},
					1 => {
						let mut next: Token = g_inner.pop_front().unwrap().clone();

						if let Token::Group(_) = next {
							treefold_one(&mut next, check, op_type, new_token).unwrap();
						}

						let mut new_token_args: VecDeque<Token> = VecDeque::with_capacity(1);
						new_token_args.push_back(next);
						new.push_back(new_token(new_token_args));
					},
					2 => {
						let mut last: Token = new.pop_back().unwrap();
						let mut next: Token = g_inner.pop_front().unwrap().clone();

						// TODO: append to t_vec, do this without recursion.
						if let Token::Group(_) = last {
							treefold_one(&mut last, check, op_type, new_token).unwrap();
						}
						if let Token::Group(_) = next {
							treefold_one(&mut next, check, op_type, new_token).unwrap();
						}

						let mut new_token_args: VecDeque<Token> = VecDeque::with_capacity(2);
						new_token_args.push_back(last);
						new_token_args.push_back(next);
						new.push_back(new_token(new_token_args));
					},
					_ => panic!()
				};
			} else {
				new.push_back(t.clone());
			}
		}

		*g_inner = new;
	}

	Ok(())
}


fn is_mult(t: &Token) -> bool {
	match t {
		Token::Operator(s) => {s == "*"},
		_ => false
	}
}
fn new_mult(v: VecDeque<Token>) -> Token { Token::Multiply(v) }

fn is_add(t: &Token) -> bool {
	match t {
		Token::Operator(s) => {s == "+"},
		_ => false
	}
}
fn new_add(v: VecDeque<Token>) -> Token { Token::Add(v) }

fn is_div(t: &Token) -> bool {
	match t {
		Token::Operator(s) => {s == "/"},
		_ => false
	}
}
fn new_div(v: VecDeque<Token>) -> Token { Token::Divide(v) }

fn is_sub(t: &Token) -> bool {
	match t {
		Token::Operator(s) => {s == "-"},
		_ => false
	}
}
fn new_sub(v: VecDeque<Token>) -> Token { Token::Subtract(v) }

fn is_fac(t: &Token) -> bool {
	match t {
		Token::Factorial => true,
		_ => false
	}
}
fn new_fac(v: VecDeque<Token>) -> Token { Token::Fac(v) }

fn is_neg(t: &Token) -> bool {
	match t {
		Token::Negative => true,
		_ => false
	}
}
fn new_neg(v: VecDeque<Token>) -> Token { Token::Neg(v) }

pub fn treefold(exp: &mut Token) -> Result<(), ()> {
	treefold_one(exp, is_fac, 0, new_fac)?;
	treefold_one(exp, is_neg, 1, new_neg)?;
	treefold_one(exp, is_mult, 2, new_mult)?;
	treefold_one(exp, is_div, 2, new_div)?;
	treefold_one(exp, is_add, 2, new_add)?;
	treefold_one(exp, is_sub, 2, new_sub)?;
	Ok(())
}