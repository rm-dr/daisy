use crate::parser::Token;
use crate::parser::Constant;
use crate::quantity::Quantity;

use super::operator::op_apply;
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

				let e: Token = match p {
					Token::Quantity(_) => { p.clone() },
					Token::Constant(c) => {
						match c {
							// Mathematical constants
							// 100 digits of each.
							Constant::Pi => { Token::Quantity(Quantity::new_float_from_string(
								"3.141592653589793238462643383279502884197169399375105820974944592307816406286208998628034825342117067"
							).unwrap())},

							Constant::E => { Token::Quantity(Quantity::new_float_from_string(
								"2.718281828459045235360287471352662497757247093699959574966967627724076630353547594571382178525166427"
							).unwrap()) },

							Constant::Phi => { Token::Quantity(Quantity::new_float_from_string(
								"1.618033988749894848204586834365638117720309179805762862135448622705260462818902449707207204189391137"
							).unwrap()) },
						}
					},
					Token::Operator(o,v) => { op_apply(o, &v)? }
				};

				//let e = p.eval()?;
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