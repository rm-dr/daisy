use crate::parser::Token;
use crate::parser::Operator;

use super::operator::eval_operator;
use super::constant::eval_constant;
use super::function::eval_function;
use super::EvalError;


pub fn evaluate(t: &Token) -> Result<Token, EvalError> {
	let mut g = t.clone();
	let mut coords: Vec<usize> = Vec::with_capacity(16);
	coords.push(0);

	'outer: loop {

		let mut h = &mut g;
		for t in coords.iter() {
			let inner = h.get_args_mut();

			if inner.is_none() || *t >= inner.as_ref().unwrap().len() {
				coords.pop();


				let p = Token::get_at_coords(&mut g, &coords);
				let mut e = p.clone();

				// Evaluate until we get a single number.
				// This loop is necessary because some eval_* functions
				// May return an incomplete result.
				// ( For example, csc(x) is treated as 1/sin(x) )
				loop {
					e = match e {
						Token::Quantity(_) => { break; },
						Token::Constant(c) => { eval_constant(&c)? }
						Token::Operator(Operator::Function(f), v) => { eval_function(&f, &v)? }
						Token::Operator(o, v) => { eval_operator(&o, &v)? }
					};
				}

				*p = e;

				if coords.len() == 0 { break 'outer; }
				let l = coords.pop().unwrap();
				coords.push(l + 1);
				continue 'outer;
			}

			h = &mut inner.unwrap()[*t];
		}



		match h {
			Token::Operator(_,_) => {
				coords.push(0);
				continue 'outer;
			},

			Token::Constant(_) => {
				coords.push(0);
				continue 'outer;
			},

			Token::Quantity(_) => {
				let l = coords.pop().unwrap();
				coords.push(l + 1);
				continue 'outer;
			}
		};
	}

	return Ok(g);
}