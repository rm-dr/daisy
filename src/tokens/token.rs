use std::collections::VecDeque;

use crate::tokens::Operator;
use crate::quantity::Quantity;

/// Tokens represent logical objects in an expession.
///
/// Tokens starting with `Pre*` are intermediate tokens, and
/// will never show up in a fully-parsed expression tree.
#[derive(Debug)]
#[derive(Clone)]
pub enum Token {
	Number(Quantity),

	Constant(Quantity, String),

	Operator(
		Operator,
		VecDeque<Token>
	),
}

impl ToString for Token {
	fn to_string(&self) -> String {
		match self {
			Token::Number(v) => v.to_string(),
			Token::Constant(_, s) => s.clone(),
			Token::Operator(o,a) => o.print(a)
		}
	}
}

impl Token {

	// This is called only when this is the outermost token.
	// This sometimes leads to different--usually more verbose--behavior.
	pub fn to_string_outer(&self) -> String {
		match self {
			Token::Number(v) => v.to_string_outer(),
			Token::Constant(_, s) => s.clone(),
			Token::Operator(o,a) => o.print(a)
		}
	}


	#[inline(always)]
	pub fn get_args(&self) -> Option<&VecDeque<Token>> {
		match self {
			Token::Operator(_, ref a) => Some(a),
			_ => None
		}
	}

	#[inline(always)]
	pub fn get_args_mut(&mut self) -> Option<&mut VecDeque<Token>> {
		match self {
			Token::Operator(_, ref mut a) => Some(a),
			_ => None
		}
	}

	#[inline(always)]
	pub fn eval(&self) -> Result<Token, ()> {
		Ok(match self {
			Token::Number(_) => { self.clone() },
			Token::Constant(v,_) => { Token::Number(v.clone()) },
			Token::Operator(o,v) => { o.apply(&v)? }
		})
	}

	// Temporary solution
	#[inline(always)]
	pub fn as_number(&self) -> Token {
		match self {
			Token::Number(v) => { Token::Number(v.clone()) },
			Token::Constant(v,_) => { Token::Number(v.clone()) },
			_ => panic!()
		}
	}

}


// Evaluate
impl Token {
	#[inline(always)]
	fn get_at_coords<'a>(g: &'a mut Token, coords: &Vec<usize>) -> &'a mut Token {
		let mut h = &mut *g;

		for t in coords.iter() {
			let inner = h.get_args_mut().unwrap();
			h = &mut inner[*t];
		}

		return h;
	}


	pub fn evaluate(&self) -> Result<Token, ()> {
		let mut g = self.clone();
		let mut coords: Vec<usize> = Vec::with_capacity(16);
		coords.push(0);

		'outer: loop {

			let mut h = &mut g;
			for t in coords.iter() {
				let inner = h.get_args_mut();

				if inner.is_none() || *t >= inner.as_ref().unwrap().len() {
					coords.pop();


					let p = Token::get_at_coords(&mut g, &coords);
					let e = p.eval()?;
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
				=> {
					coords.push(0);
					continue 'outer;
				},

				Token::Constant(_,_) |
				Token::Number(_) => {
					let l = coords.pop().unwrap();
					coords.push(l + 1);
					continue 'outer;
				}
			};
		}

		return Ok(g);
	}
}