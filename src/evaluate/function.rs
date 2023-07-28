use crate::parser::Expression;
use crate::parser::Function;
use crate::parser::Operator;
use crate::parser::LineLocation;
use super::EvalError;

pub fn eval_function(g: &Expression) -> Result<Expression, (LineLocation, EvalError)> {

	let Expression::Operator(loc, Operator::Function(f), args) = g else {panic!()};

	if args.len() != 1 {panic!()};
	let a = &args[0];
	let Expression::Quantity(l, q) = a else {panic!()};


	match f {
		Function::NoUnit => { return Ok(Expression::Quantity(*loc + *l, q.without_unit())); }
		Function::ToBase => { return Ok(Expression::Quantity(*loc + *l, q.convert_to_base())); }
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

		Function::Sin => { return Ok(Expression::Quantity(*loc + *l, q.sin())); },
		Function::Cos => { return Ok(Expression::Quantity(*loc + *l, q.cos())); },
		Function::Tan => { return Ok(Expression::Quantity(*loc + *l, q.tan())); },
		Function::Asin => { return Ok(Expression::Quantity(*loc + *l, q.asin())); },
		Function::Acos => { return Ok(Expression::Quantity(*loc + *l, q.acos())); },
		Function::Atan => { return Ok(Expression::Quantity(*loc + *l, q.atan())); },

		Function::Csc => { return Ok(Expression::Quantity(*loc + *l, q.csc())); },
		Function::Sec => { return Ok(Expression::Quantity(*loc + *l, q.sec())); },
		Function::Cot => { return Ok(Expression::Quantity(*loc + *l, q.cot())); },

		Function::Sinh => { return Ok(Expression::Quantity(*loc + *l, q.sinh())); },
		Function::Cosh => { return Ok(Expression::Quantity(*loc + *l, q.cosh())); },
		Function::Tanh => { return Ok(Expression::Quantity(*loc + *l, q.tanh())); },
		Function::Asinh => { return Ok(Expression::Quantity(*loc + *l, q.asinh())); },
		Function::Acosh => { return Ok(Expression::Quantity(*loc + *l, q.acosh())); },
		Function::Atanh => { return Ok(Expression::Quantity(*loc + *l, q.atanh())); },

		Function::Csch => { return Ok(Expression::Quantity(*loc + *l, q.csch())); },
		Function::Sech => { return Ok(Expression::Quantity(*loc + *l, q.sech())); },
		Function::Coth => { return Ok(Expression::Quantity(*loc + *l, q.coth())); },

		Function::ToBase
		| Function::NoUnit
		=> unreachable!()
	}
}