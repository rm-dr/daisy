use crate::parser::Expression;
use crate::parser::Operator;
use crate::context::Context;
use crate::parser::LineLocation;

use super::operator::eval_operator;
use super::function::eval_function;
use super::EvalError;

pub fn evaluate(t: &Expression, context: &mut Context, allow_incomplete: bool) -> Result<Expression, (LineLocation, EvalError)> {

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

			// If true, move to the next node
			let mut move_up = true;

			let new = match g {
				Expression::Quantity(_, _) => None,

				Expression::Constant(_, c) => { Some(evaluate(&c.value(), context, false).unwrap()) },
				Expression::Variable(l, s) => {
					// Don't move up, re-evaluate
					// This makes variables containing floating variables work properly
					// (For example, try x = a + 2, a = 2, x. x should evaluate to 4.)
					move_up = false;
					let v = context.get_variable(&s);

					// Error if variable is undefined.
					// Comment this to allow floating varables.
					if v.is_none() { return Err((*l, EvalError::Undefined(s.clone()))); }

					v
				},
				Expression::Operator(_, Operator::Function(_), _) => { Some(eval_function(g)?) },
				Expression::Operator(_, _, _) => { eval_operator(g, context)? },
			};

			if let Some(mut new) = new {
				if let Expression::Constant(_,_) = g {
					// Fix constant line location.
					// Constant expansion does not change the location of a value,
					// but other operations might.
					new.set_linelocation(&g.get_linelocation());
				}
				*g = new;
			} else {

				if !allow_incomplete {
					if let Expression::Quantity(_, _) = g {}
					else {
						let l = g.get_linelocation();
						return Err((l, EvalError::EvaluationError))
					}
				}

				// Always move up if we couldn't evaluate this node.
				move_up = true;
			}


			if move_up {
				// Move up the tree
				coords.pop();
				if coords.len() != 0 {
					*coords.last_mut().unwrap() += 1;
				} else { break; }
			}

		} else {
			// Move down the tree
			coords.push(0);
		}
	}

	return Ok(root);
}