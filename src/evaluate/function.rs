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

	let Expression::Operator(loc, Operator::Function(f), args) = g else {unreachable!()};

	if args.len() != 1 {panic!()};
	let a = &args[0];
	let Expression::Quantity(l, q) = a else {panic!()};


	match f {
		Function::NoUnit => { return Ok(Expression::Quantity(*loc + *l, q.without_unit())); }
		Function::ToBase => { return Ok(Expression::Quantity(*loc + *l, q.convert_to_base())); }



		Function::Abs => {
			if !q.unitless() { return Err((*loc + *l, EvalError::IncompatibleUnit));}
			return Ok(Expression::Quantity(*loc + *l, q.abs()));
		},
		Function::Floor => {
			if !q.unitless() { return Err((*loc + *l, EvalError::IncompatibleUnit));}
			return Ok(Expression::Quantity(*loc + *l, q.floor()));
		},
		Function::Ceil => {
			if !q.unitless() { return Err((*loc + *l, EvalError::IncompatibleUnit));}
			return Ok(Expression::Quantity(*loc + *l, q.ceil()));
		},
		Function::Round => {
			if !q.unitless() { return Err((*loc + *l, EvalError::IncompatibleUnit));}
			return Ok(Expression::Quantity(*loc + *l, q.round()));
		},
		Function::NaturalLog => {
			if !q.unitless() { return Err((*loc + *l, EvalError::IncompatibleUnit));}
			return Ok(Expression::Quantity(*loc + *l, q.ln()));
		},
		Function::TenLog => {
			if !q.unitless() { return Err((*loc + *l, EvalError::IncompatibleUnit));}
			return Ok(Expression::Quantity(*loc + *l, q.log10()));
		},



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
		Function::Asin => {
			if !q.unitless() { return Err((*loc + *l, EvalError::IncompatibleUnit));}
			return Ok(Expression::Quantity(*loc + *l, q.asin()));
		},
		Function::Acos => {
			if !q.unitless() { return Err((*loc + *l, EvalError::IncompatibleUnit));}
			return Ok(Expression::Quantity(*loc + *l, q.acos()));
		},
		Function::Atan => {
			if !q.unitless() { return Err((*loc + *l, EvalError::IncompatibleUnit));}
			return Ok(Expression::Quantity(*loc + *l, q.atan()));
		},
		Function::Asinh => {
			if !q.unitless() { return Err((*loc + *l, EvalError::IncompatibleUnit));}
			return Ok(Expression::Quantity(*loc + *l, q.asinh()));
		},
		Function::Acosh => {
			if !q.unitless() { return Err((*loc + *l, EvalError::IncompatibleUnit));}
			return Ok(Expression::Quantity(*loc + *l, q.acosh()));
		},
		Function::Atanh => {
			if !q.unitless() { return Err((*loc + *l, EvalError::IncompatibleUnit));}
			return Ok(Expression::Quantity(*loc + *l, q.atanh()));
		},



		Function::ToCelsius => {
			let mut k = Quantity::new_rational(1f64).unwrap();
			k.insert_unit(FreeUnit::from_whole(WholeUnit::Kelvin), Scalar::new_rational(1f64).unwrap());
			let Some(q) = q.convert_to(k) else { return Err((*loc + *l, EvalError::IncompatibleUnit)) };

			let mut r = q.without_unit();
			r += Quantity::new_rational(-273.15f64).unwrap();

			return Ok(Expression::Quantity(*loc + *l, r));
		},
		Function::ToFahrenheit => {
			let mut k = Quantity::new_rational(1f64).unwrap();
			k.insert_unit(FreeUnit::from_whole(WholeUnit::Kelvin), Scalar::new_rational(1f64).unwrap());
			let Some(q) = q.convert_to(k) else { return Err((*loc + *l, EvalError::IncompatibleUnit)) };

			let mut r = q.without_unit();
			r *= Quantity::new_rational_from_frac(9i64, 5i64).unwrap();
			r += Quantity::new_rational(-459.67).unwrap();


			return Ok(Expression::Quantity(*loc + *l, r));
		},
		Function::FromCelsius => {
			if !q.unitless() { return Err((*loc + *l, EvalError::IncompatibleUnit));}

			let mut r = Quantity::new_rational(273.15f64).unwrap();
			r += q.clone();
			r.insert_unit(FreeUnit::from_whole(WholeUnit::Kelvin), Scalar::new_rational(1f64).unwrap());

			return Ok(Expression::Quantity(*loc + *l, r));
		},
		Function::FromFahrenheit => {
			if !q.unitless() { return Err((*loc + *l, EvalError::IncompatibleUnit));}

			let mut r = q.clone();
			r += Quantity::new_rational(459.67).unwrap();
			r *= Quantity::new_rational_from_frac(5i64, 9i64).unwrap();
			r.insert_unit(FreeUnit::from_whole(WholeUnit::Kelvin), Scalar::new_rational(1f64).unwrap());

			return Ok(Expression::Quantity(*loc + *l, r));
		}
	}
}