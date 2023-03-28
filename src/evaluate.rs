use crate::tokens::Token;

#[inline(always)]
fn get_at_coords<'a>(g: &'a mut Token, coords: &Vec<usize>) -> &'a mut Token {
	let mut h = &mut *g;

	for t in coords.iter() {
		let inner = h.get_args().unwrap();
		h = &mut inner[*t];
	}

	return h;
}


pub fn evaluate(
	mut g: Token,
) -> Result<Token, ()> {
	let mut coords: Vec<usize> = Vec::with_capacity(16);
	coords.push(0);

	'outer: loop {

		let mut h = &mut g;
		for t in coords.iter() {
			let inner = h.get_args();

			if inner.is_none() || *t >= inner.as_ref().unwrap().len() {
				coords.pop();


				let p = get_at_coords(&mut g, &coords);
				let e = p.eval();
				*p = e;

				if coords.len() == 0 { break 'outer; }
				let l = coords.pop().unwrap();
				coords.push(l + 1);
				continue 'outer;
			}

			h = &mut inner.unwrap()[*t];
		}

		match h {
			Token::Operator(_,_,_)
			=> {
				coords.push(0);
				continue 'outer;
			},

			Token::Constant(_,_,_) |
			Token::Number(_,_) => {
				let l = coords.pop().unwrap();
				coords.push(l + 1);
				continue 'outer;
			}
		};
	}

	return Ok(g);
}