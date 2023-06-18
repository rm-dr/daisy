use crate::parser::Expression;
use crate::parser::Operator;
use crate::context::Context;


use super::operator::eval_operator;
use super::function::eval_function;
use super::EvalError;

pub fn evaluate(t: &Expression, context: &mut Context) -> Result<Expression, EvalError> {

	// Keeps track of our position in the expression tree.
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
				Expression::Quantity(_) => None,

				Expression::Constant(c) => { Some(evaluate(&c.value(), context).unwrap()) },
				Expression::Variable(s) => { context.get_variable(&s) },
				Expression::Operator(Operator::Function(f), v) => { Some(eval_function(&f, &v)?) },
				Expression::Operator(o, v) => { eval_operator(&o, &v, context)? },
			};

			if new.is_some() { *g = new.unwrap()}


			// Move up the tree
			coords.pop();
			if coords.len() != 0 {
				*coords.last_mut().unwrap() += 1;
			} else { break; }

		} else {
			// Move down the tree

			// Don't evaluate the first argument of a define.
			// This prevents variables from being expanded before a re-assignment.
			if let Expression::Operator(Operator::Define, _) = g {
					*coords.last_mut().unwrap() += 1;
				coords.push(0);
				continue;
				}

			coords.push(0);
		}
	}

	return Ok(root);
}