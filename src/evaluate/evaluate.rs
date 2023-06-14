use crate::parser::Token;
use crate::parser::Operator;
use crate::context::Context;


use super::operator::eval_operator;
use super::function::eval_function;
use super::EvalError;

pub fn evaluate(t: &Token, context: &Context) -> Result<Token, EvalError> {
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
						Token::Constant(c) => { evaluate(&c.value(), context).unwrap() },
						Token::Operator(Operator::Function(f), v) => { eval_function(&f, &v)? },
						Token::Operator(o, v) => { eval_operator(&o, &v)? },
						Token::Variable(s) => {
							if let Some(t) = context.get_variable(s) { t } else {
								return Err(EvalError::NoHistory);
							}
						}
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
			Token::Operator(_,_)
			| Token::Constant(_)
			| Token::Variable(_)
			=> { coords.push(0); },

			Token::Quantity(_) => {
				let l = coords.pop().unwrap();
				coords.push(l + 1);
			}
		};
	}

	return Ok(g);
}