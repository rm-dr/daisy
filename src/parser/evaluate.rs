use std::collections::VecDeque;

use crate::parser::Token;
use crate::parser::Eval;
use crate::parser::LineLocation;
use crate::parser::ParserError;

fn get_at_coords<'a>(g: &'a mut Token, coords: &Vec<usize>) -> &'a mut Token {
	let mut h = &mut *g;

	for t in coords.iter() {
		let inner = match h {
			Token::Multiply(ref mut v) => v,
			Token::Divide(ref mut v) => v,
			Token::Add(ref mut v) => v,
			Token::Factorial(ref mut v) => v,
			Token::Negative(ref mut v) => v,
			Token::Power(ref mut v) => v,
			Token::Modulo(ref mut v) => v,
			Token::Root(ref mut v) => v,
			_ => panic!()
		};

		h = &mut inner[*t];
	}

	return h;
}


pub fn evaluate(
	mut g: Token,
) -> Result<Token, (LineLocation, ParserError)> {

	let mut coords: Vec<usize> = Vec::with_capacity(16);
	coords.push(0);

	'outer: loop {

		let mut h = &mut g;
		for t in coords.iter() {
			let inner = match h {
				Token::Multiply(ref mut v) => v,
				Token::Divide(ref mut v) => v,
				Token::Add(ref mut v) => v,
				Token::Factorial(ref mut v) => v,
				Token::Negative(ref mut v) => v,
				Token::Power(ref mut v) => v,
				Token::Modulo(ref mut v) => v,
				Token::Root(ref mut v) => v,
				_ => panic!()
			};

			if *t >= inner.len() {
				coords.pop();
				if coords.len() == 0 { break 'outer; }


				let p = get_at_coords(&mut g, &coords);
				let e = p.eval();
				*p = e;

				let l = coords.pop().unwrap();
				coords.push(l + 1);
				continue 'outer;
			}

			h = &mut inner[*t];
		}

		match h {
			Token::Multiply(v) |
			Token::Divide(v) |
			Token::Add(v) |
			Token::Factorial(v) |
			Token::Negative(v) |
			Token::Power(v) |
			Token::Modulo(v)
			=> {
				coords.push(0);
				continue 'outer;
			},

			Token::Number(_,_) => {
				let l = coords.pop().unwrap();
				coords.push(l + 1);
				continue 'outer;
			}
			_ => panic!()
		};
	}


	return Ok(g);
}