use std::collections::VecDeque;
use crate::parser::tokenize::Token;

enum OperatorType {
	Binary,    // A binary operator, like a + b
	UnaryLeft, // A left operator, like a!
	UnaryRight // A right operator, like -a
}

fn fold_operators_once(
	g_main: &mut Token, // Must be a group
	op_type: &OperatorType,
	check: fn(&str) -> bool,
	new_token: fn(&str, VecDeque<Token>) -> Token,
) -> Result<(), ()> {

	// Groups to process
	let mut t_vec: VecDeque<&mut Token> = VecDeque::with_capacity(32);
	t_vec.push_back(g_main);

	while t_vec.len() > 0 {

		// The group we're currently working with
		let g: &mut Token = t_vec.pop_front().unwrap();
		let g_inner: &mut VecDeque<Token> = match g {
			Token::PreGroup(_, ref mut x) |
			Token::Multiply(ref mut x) |
			Token::Divide(ref mut x) |
			Token::Add(ref mut x) |
			Token::Subtract(ref mut x) |
			Token::Factorial(ref mut x) |
			Token::Negative(ref mut x) |
			Token::Power(ref mut x) |
			Token::Modulo(ref mut x) => x,
			_ => panic!()
		};

		let mut new: VecDeque<Token> = VecDeque::with_capacity(8);

		// Build new group array
		while g_inner.len() > 0 {
			let mut t: Token = match g_inner.pop_front() {
				Some(o) => o,
				None => break
			};

			let s: &str;
			if let Token::PreOperator(_, ref x) = t {
				s = x;
			} else {
				match t {
					Token::PreGroup(_, _) |
					Token::Multiply(_) |
					Token::Divide(_) |
					Token::Add(_) |
					Token::Subtract(_) |
					Token::Factorial(_) |
					Token::Negative(_) |
					Token::Power(_) |
					Token::Modulo(_) => {
						fold_operators_once(&mut t, op_type, check, new_token)?;
					},
					_ => {}
				};
				new.push_back(t);
				continue;
			}

			if check(s) {
				match op_type {
					OperatorType::UnaryLeft => {
						let mut last: Token = new.pop_back().unwrap();

						if let Token::PreGroup(_, _) = last {
							fold_operators_once(&mut last, op_type, check, new_token)?;
						}

						let mut new_token_args: VecDeque<Token> = VecDeque::with_capacity(1);
						new_token_args.push_back(last);
						new.push_back(new_token(s, new_token_args));
					},
					OperatorType::UnaryRight => {
						let mut next: Token = g_inner.pop_front().unwrap();

						if let Token::PreGroup(_, _) = next {
							fold_operators_once(&mut next, op_type, check, new_token)?;
						}

						let mut new_token_args: VecDeque<Token> = VecDeque::with_capacity(1);
						new_token_args.push_back(next);
						new.push_back(new_token(s, new_token_args));
					},
					OperatorType::Binary => {
						let mut last: Token = new.pop_back().unwrap();
						let mut next: Token = g_inner.pop_front().unwrap();

						// TODO: append to t_vec and do this without recursion.
						if let Token::PreGroup(_, _) = last {
							fold_operators_once(&mut last, op_type, check, new_token)?;
						}
						if let Token::PreGroup(_, _) = next {
							fold_operators_once(&mut next, op_type, check, new_token)?;
						}

						let mut new_token_args: VecDeque<Token> = VecDeque::with_capacity(2);
						new_token_args.push_back(last);
						new_token_args.push_back(next);
						new.push_back(new_token(s, new_token_args));
					}
				};
			} else {
				if let Token::PreGroup(_, _) = t {
					fold_operators_once(&mut t, op_type, check, new_token)?;
				}
				new.push_back(t);
			}
		}

		*g_inner = new;
	}

	return Ok(());
}

pub fn fold_operators(exp: &mut Token) -> Result<(), ()> {
	fold_operators_once(
		exp, &OperatorType::UnaryLeft,
		|s| s=="!",
		|_s, x| Token::Factorial(x)
	)?;
	fold_operators_once(
		exp, &OperatorType::UnaryRight,
		|s| s=="neg",
		|_s, x| Token::Negative(x)
	)?;

	fold_operators_once(
		exp, &OperatorType::Binary,
		|s| s=="^",
		|_s, x| Token::Power(x)
	)?;
	fold_operators_once(
		exp, &OperatorType::Binary,
		|s| s=="%",
		|_s, x| Token::Modulo(x)
	)?;

	fold_operators_once(
		exp, &OperatorType::Binary,
		|s| s=="*" || s == "/",
		|s, x| match s {
			"*" => Token::Multiply(x),
			"/" => Token::Divide(x),
			_=>panic!()
		}
	)?;
	fold_operators_once(
		exp, &OperatorType::Binary,
		|s| s=="+" || s == "-",
		|s, x| match s {
			"+" => Token::Add(x),
			"-" => Token::Subtract(x),
			_=>panic!()
		}
	)?;

	fold_operators_once(
		exp, &OperatorType::Binary,
		|s| s=="mod",
		|_s, x| Token::Modulo(x)
	)?;

	return Ok(());
}