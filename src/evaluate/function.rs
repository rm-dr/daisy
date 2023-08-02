use crate::parser::Expression;
use crate::parser::Function;
use crate::parser::Operator;
use crate::parser::LineLocation;
use crate::quantity::FreeUnit;
use crate::quantity::WholeUnit;
use crate::quantity::Quantity;
use crate::quantity::Scalar;
use super::EvalError;


// If unitless, do nothing
// If compatible with radians, convert to radians and return unitless
// Otherwise, error.
//
// Used for trig functions.
fn to_radians(q: Quantity) -> Result<Quantity, ()> {
	if q.unitless() { return Ok(q); }

	let mut r = Quantity::new_rational(1f64).unwrap();
	r.insert_unit(FreeUnit::from_whole(WholeUnit::Radian), Scalar::new_rational(1f64).unwrap());
	let Some(q) = q.convert_to(r) else { return Err(()) };

	return Ok(q.without_unit());
}



pub fn eval_function(g: &Expression) -> Result<Expression, (LineLocation, EvalError)> {

	let Expression::Operator(loc, Operator::Function(f), args) = g else {panic!()};

	if args.len() != 1 {panic!()};
	let a = &args[0];
	let Expression::Quantity(l, q) = a else {panic!()};


	match f {
		Function::NoUnit => { return Ok(Expression::Quantity(*loc + *l, q.without_unit())); }
		Function::ToBase => { return Ok(Expression::Quantity(*loc + *l, q.convert_to_base())); }

		// Trigonometry
		Function::Sin => {
			let Ok(q) = to_radians(q.clone()) else { return Err((*loc + *l, EvalError::IncompatibleUnit)); };
			return Ok(Expression::Quantity(*loc + *l, q.sin()));
		},
		Function::Cos => {
			let Ok(q) = to_radians(q.clone()) else { return Err((*loc + *l, EvalError::IncompatibleUnit)); };
			return Ok(Expression::Quantity(*loc + *l, q.cos()));
		},
		Function::Tan => {
			let Ok(q) = to_radians(q.clone()) else { return Err((*loc + *l, EvalError::IncompatibleUnit)); };
			return Ok(Expression::Quantity(*loc + *l, q.tan()));
		},
		Function::Csc => {
			let Ok(q) = to_radians(q.clone()) else { return Err((*loc + *l, EvalError::IncompatibleUnit)); };
			return Ok(Expression::Quantity(*loc + *l, q.csc()));
		},
		Function::Sec => {
			let Ok(q) = to_radians(q.clone()) else { return Err((*loc + *l, EvalError::IncompatibleUnit)); };
			return Ok(Expression::Quantity(*loc + *l, q.sec()));
		},
		Function::Cot => {
			let Ok(q) = to_radians(q.clone()) else { return Err((*loc + *l, EvalError::IncompatibleUnit)); };
			return Ok(Expression::Quantity(*loc + *l, q.cot()));
		},
		Function::Sinh => {
			let Ok(q) = to_radians(q.clone()) else { return Err((*loc + *l, EvalError::IncompatibleUnit)); };
			return Ok(Expression::Quantity(*loc + *l, q.sinh()));
		},
		Function::Cosh => {
			let Ok(q) = to_radians(q.clone()) else { return Err((*loc + *l, EvalError::IncompatibleUnit)); };
			return Ok(Expression::Quantity(*loc + *l, q.cosh()));
		},
		Function::Tanh => {
			let Ok(q) = to_radians(q.clone()) else { return Err((*loc + *l, EvalError::IncompatibleUnit)); };
			return Ok(Expression::Quantity(*loc + *l, q.tanh()));
		},
		Function::Csch => {
			let Ok(q) = to_radians(q.clone()) else { return Err((*loc + *l, EvalError::IncompatibleUnit)); };
			return Ok(Expression::Quantity(*loc + *l, q.csch()));
		},
		Function::Sech => {
			let Ok(q) = to_radians(q.clone()) else { return Err((*loc + *l, EvalError::IncompatibleUnit)); };
			return Ok(Expression::Quantity(*loc + *l, q.sech()));
		},
		Function::Coth => {
			let Ok(q) = to_radians(q.clone()) else { return Err((*loc + *l, EvalError::IncompatibleUnit)); };
			return Ok(Expression::Quantity(*loc + *l, q.coth()));
		},
		_ => {}
	}

	if !q.unitless() {
		return Err((*loc + *l, EvalError::IncompatibleUnit));
	}

	match f {
		Function::Abs => { return Ok(Expression::Quantity(*loc + *l, q.abs())); },
		Function::Floor => { return Ok(Expression::Quantity(*loc + *l, q.floor())); },
		Function::Ceil => { return Ok(Expression::Quantity(*loc + *l, q.ceil())); },
		Function::Round => { return Ok(Expression::Quantity(*loc + *l, q.round())); },

		Function::NaturalLog => { return Ok(Expression::Quantity(*loc + *l, q.ln())); },
		Function::TenLog => { return Ok(Expression::Quantity(*loc + *l, q.log10())); },

		Function::Asin => { return Ok(Expression::Quantity(*loc + *l, q.asin())); },
		Function::Acos => { return Ok(Expression::Quantity(*loc + *l, q.acos())); },
		Function::Atan => { return Ok(Expression::Quantity(*loc + *l, q.atan())); },

		Function::Asinh => { return Ok(Expression::Quantity(*loc + *l, q.asinh())); },
		Function::Acosh => { return Ok(Expression::Quantity(*loc + *l, q.acosh())); },
		Function::Atanh => { return Ok(Expression::Quantity(*loc + *l, q.atanh())); },

		Function::ToBase
		| Function::NoUnit
		| Function::Sin
		| Function::Cos
		| Function::Tan
		| Function::Csc
		| Function::Sec
		| Function::Cot
		| Function::Sinh
		| Function::Cosh
		| Function::Tanh
		| Function::Csch
		| Function::Sech
		| Function::Coth
		=> unreachable!()
	}
}