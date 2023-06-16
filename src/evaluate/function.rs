use std::collections::VecDeque;

use crate::parser::Expression;
use crate::parser::Function;
use super::EvalError;


pub fn eval_function(f: &Function, args: &VecDeque<Expression>) -> Result<Expression, EvalError> {
	if args.len() != 1 {panic!()};
	let a = &args[0];
	let Expression::Quantity(q) = a else {panic!()};


	match f {
		Function::NoUnit => { return Ok(Expression::Quantity(q.without_unit())); }
		Function::ToBase => { return Ok(Expression::Quantity(q.convert_to_base())); }
		_ => {}
	}

	if !q.unitless() {
		return Err(EvalError::IncompatibleUnit);
	}

	match f {
		Function::Abs => { return Ok(Expression::Quantity(q.abs())); },
		Function::Floor => { return Ok(Expression::Quantity(q.floor())); },
		Function::Ceil => { return Ok(Expression::Quantity(q.ceil())); },
		Function::Round => { return Ok(Expression::Quantity(q.round())); },

		Function::NaturalLog => { return Ok(Expression::Quantity(q.ln())); },
		Function::TenLog => { return Ok(Expression::Quantity(q.log10())); },

		Function::Sin => { return Ok(Expression::Quantity(q.sin())); },
		Function::Cos => { return Ok(Expression::Quantity(q.cos())); },
		Function::Tan => { return Ok(Expression::Quantity(q.tan())); },
		Function::Asin => { return Ok(Expression::Quantity(q.asin())); },
		Function::Acos => { return Ok(Expression::Quantity(q.acos())); },
		Function::Atan => { return Ok(Expression::Quantity(q.atan())); },

		Function::Csc => { return Ok(Expression::Quantity(q.csc())); },
		Function::Sec => { return Ok(Expression::Quantity(q.sec())); },
		Function::Cot => { return Ok(Expression::Quantity(q.cot())); },

		Function::Sinh => { return Ok(Expression::Quantity(q.sinh())); },
		Function::Cosh => { return Ok(Expression::Quantity(q.cosh())); },
		Function::Tanh => { return Ok(Expression::Quantity(q.tanh())); },
		Function::Asinh => { return Ok(Expression::Quantity(q.asinh())); },
		Function::Acosh => { return Ok(Expression::Quantity(q.acosh())); },
		Function::Atanh => { return Ok(Expression::Quantity(q.atanh())); },

		Function::Csch => { return Ok(Expression::Quantity(q.csch())); },
		Function::Sech => { return Ok(Expression::Quantity(q.sech())); },
		Function::Coth => { return Ok(Expression::Quantity(q.coth())); },

		Function::ToBase
		| Function::NoUnit
		=> panic!()
	}
}