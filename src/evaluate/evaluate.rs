use crate::parser::Token;
use crate::parser::Operator;
use crate::context::Context;


use super::operator::eval_operator;
use super::function::eval_function;
use super::EvalError;

pub fn evaluate(t: &Token, context: &mut Context) -> Result<Token, EvalError> {

	// Keeps track of our position in the token tree.
	// For example, the coordinates [0, 2, 1] are interpreted as follows:
	// Start at the root node,
	//    then move to that node's 0th child,
	//    then move to that node's 2nd child,
	//    then move to that node's 1st child.
	//
	let mut coords: Vec<usize> = Vec::with_capacity(16);
	let mut root = t.clone();

	// coords points to the *next* node we will move to.
	coords.push(0);

	// Repeats while we have coordinates to parse.
	// Exits when we finish parsing the root node.
	loop {
		// Current position in the tree
		let g = root.get_at_coords_mut(
			&coords[0 .. coords.len() - 1]
		).unwrap();

		// "Move up" step.
		// We move up if we're at a leaf or if we're out of children to move down to.
		if {
			g.is_quantity() ||
			g.get_args().is_none() ||
			(coords.len() != 0 && (*coords.last().unwrap() >= g.get_args().unwrap().len()))
		} {


			let new = match g {
				Token::Quantity(_) => None,

				Token::Constant(c) => { Some(evaluate(&c.value(), context).unwrap()) },
				Token::Variable(s) => { context.get_variable(&s) },
				Token::Operator(Operator::Function(f), v) => { Some(eval_function(&f, &v)?) },
				Token::Operator(o, v) => { eval_operator(&o, &v, context)? },
			};

			if new.is_some() { *g = new.unwrap()}


			// Move up the tree
			coords.pop();
			if coords.len() != 0 {
				*coords.last_mut().unwrap() += 1;
			} else { break; }

		} else {
			// Move down the tree
			coords.push(0);

			let n = root.get_at_coords(&coords[..]);
			if let Some(n) = n {
				if let Token::Operator(Operator::Define, _) = n {
					*coords.last_mut().unwrap() += 1;
				}
			}
		}
	}

	return Ok(root);
}