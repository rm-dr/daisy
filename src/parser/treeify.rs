use std::collections::VecDeque;

use crate::parser::Token;
use crate::parser::LineLocation;
use crate::parser::ParserError;
use crate::parser::Operators;


pub fn treeify(
	g: &mut Token,
) -> Result<(), (LineLocation, ParserError)> {

	let g_inner: &mut VecDeque<Token> = match g {
		Token::PreGroup(_, ref mut x) => x,
		_ => panic!()
	};

	let mut i = 1;
	while g_inner.len() > 1 {

		let a: isize;
		if i == 1 {
			a = Operators::Null as isize;
		} else {
			let q: Operators = match g_inner[i-2] {
				Token::PreOperator(_, x) => x,
				_ => panic!()
			};
			a = q as isize;
		}

		let b: isize = match g_inner[i] {
			Token::PreOperator(_, x) => x,
			_ => panic!()
		} as isize;

		let c: isize;
		if i >= g_inner.len()-2 {
			c = Operators::Null as isize;
		} else {
			let q: Operators = match g_inner[i+2] {
				Token::PreOperator(_, x) => x,
				_ => panic!()
			};
			c = q as isize;
		}

		println!("{}, {:?}", i, g_inner);
		if b >= a && b >= c {
			// This operator owns both its arguments.
			let left = g_inner.remove(i-1).unwrap();
			let this = g_inner.remove(i-1).unwrap();
			let right = g_inner.remove(i-1).unwrap();

			let k = match this {
				Token::PreOperator(_, k) => k,
				_ => panic!()
			};

			let mut new_token_args: VecDeque<Token> = VecDeque::with_capacity(3);
			new_token_args.push_back(left);
			new_token_args.push_back(right);

			g_inner.insert(
				i-1,
				match k {
					Operators::Subtract => Token::Subtract(new_token_args),
					Operators::Add => Token::Add(new_token_args),
					Operators::Divide => Token::Divide(new_token_args),
					Operators::Multiply => Token::Multiply(new_token_args),
					Operators::ImplicitMultiply => Token::Multiply(new_token_args),
					Operators::Modulo => Token::Modulo(new_token_args),
					Operators::ModuloLong => Token::Modulo(new_token_args),
					Operators::Power => Token::Power(new_token_args),
					Operators::Null => panic!()
				}
			);
			if i >= 3 { i -= 2; }
		} else {
			// This operator has lower precedence than another.
			// skip it for now.
			i += 2;
		}
		println!("{}", i);
	}
	return Ok(());
}