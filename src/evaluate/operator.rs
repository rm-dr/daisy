use std::collections::VecDeque;

use crate::quantity::Quantity;
use crate::parser::Operator;
use crate::parser::Expression;
use super::EvalError;
use crate::context::Context;

pub fn eval_operator(op: &Operator, args: &VecDeque<Expression>, context: &mut Context) -> Result<Option<Expression>, EvalError> {
	match op {
		Operator::Function(_) => unreachable!("Functions are handled seperately."),

		Operator::Define => {
			if args.len() != 2 { panic!() };
			let b = &args[1];

			if let Expression::Variable(s) = &args[0] {
				context.push_var(s.clone(), b.clone());
				return Ok(Some(b.clone()));
			} else { return Err(EvalError::BadDefineName); }
		},

		Operator::Negative => {
			if args.len() != 1 { panic!() };
			let args = &args[0];

			if let Expression::Quantity(v) = args {
				return Ok(Some(Expression::Quantity(-v.clone())));
			} else { return Ok(None); }
		},

		Operator::Add => {
			let mut sum: Quantity;
			if let Expression::Quantity(s) = &args[0] {
				sum = s.clone();
			} else { return Ok(None); };

			let mut i: usize = 1;
			while i < args.len() {
				let j = &args[i];
				if let Expression::Quantity(v) = j {

					if !sum.unit.compatible_with(&v.unit) {
						return Err(EvalError::IncompatibleUnit);
					}

					sum += v.clone();
				} else { return Ok(None); }
				i += 1;
			}
			return Ok(Some(Expression::Quantity(sum)));
		},

		Operator::Subtract => {
			if args.len() != 2 { panic!() };
			let a = &args[0];
			let b = &args[1];

			if let Expression::Quantity(a) = a {
				if let Expression::Quantity(b) = b {
					return Ok(Some(Expression::Quantity(a.clone() - b.clone())));
				}
			}

			return Ok(None);
		},


		Operator::Divide |
		Operator::DivideLong => {
			if args.len() != 2 { panic!() };
			let a = &args[0];
			let b = &args[1];

			if let Expression::Quantity(a) = a {
				if let Expression::Quantity(b) = b {
					if b.is_zero() { return Err(EvalError::ZeroDivision); }
					return Ok(Some(Expression::Quantity(a.clone() / b.clone())));
				}
			}

			return Ok(None);
		},

		Operator::ImplicitMultiply |
		Operator::Multiply => {
			let mut prod = Quantity::new_rational(1f64).unwrap();
			for i in args.iter() {
				let j = i;
				if let Expression::Quantity(v) = j {
					prod *= v.clone();
				} else { return Ok(None); }
			}
			return Ok(Some(Expression::Quantity(prod)));
		},

		Operator::ModuloLong
		| Operator::Modulo => {
			if args.len() != 2 { panic!() };
			let a = &args[0];
			let b = &args[1];

			if let Expression::Quantity(va) = a {
				if let Expression::Quantity(vb) = b {

					if !(va.unitless() && vb.unitless()) {
						return Err(EvalError::IncompatibleUnit);
					}

					if vb <= &Quantity::new_rational(1f64).unwrap() { return Err(EvalError::BadMath); }
					if va.fract() != Quantity::new_rational(0f64).unwrap() { return Err(EvalError::BadMath); }
					if vb.fract() != Quantity::new_rational(0f64).unwrap() { return Err(EvalError::BadMath); }

					return Ok(Some(Expression::Quantity(va.clone() % vb.clone())));
				} else { return Ok(None); }
			} else { return Ok(None); }
		},

		Operator::UnitConvert => {
			if args.len() != 2 { panic!() };
			let a = &args[0];
			let b = &args[1];

			if let Expression::Quantity(va) = a {
				if let Expression::Quantity(vb) = b {
					let n = va.clone().convert_to(vb.clone());
					if n.is_none() {
						return Err(EvalError::IncompatibleUnit);
					}
					return Ok(Some(Expression::Quantity(n.unwrap())));
				} else { return Ok(None); }
			} else { return Ok(None); }
		},


		Operator::Sqrt => {
			if args.len() != 1 { panic!() }
			let a = &args[0];

			if let Expression::Quantity(va) = a {
				if va.is_negative() { return Err(EvalError::BadMath); }
				let p = va.pow(Quantity::new_rational_from_string("0.5").unwrap());
				if p.is_nan() {return Err(EvalError::BadMath);}
				return Ok(Some(Expression::Quantity(p)));
			} else { return Ok(None); }
		},

		Operator::Power => {
			if args.len() != 2 {panic!()};
			let a = &args[0];
			let b = &args[1];

			if let Expression::Quantity(va) = a {
				if let Expression::Quantity(vb) = b {

					if !vb.unitless() {
						return Err(EvalError::IncompatibleUnit);
					}

					if va.is_zero() && vb.is_negative() {
						return Err(EvalError::ZeroDivision);
					}

					let p = va.pow(vb.clone());
					if p.is_nan() {return Err(EvalError::BadMath);}
					return Ok(Some(Expression::Quantity(p)));
				} else { return Ok(None); }
			} else { return Ok(None); }
		},

		Operator::Factorial => {
			if args.len() != 1 {panic!()};
			let args = &args[0];

			if let Expression::Quantity(v) = args {

				if !v.unitless() {
					return Err(EvalError::IncompatibleUnit);
				}

				if !v.fract().is_zero() { return Err(EvalError::BadMath); }
				if v > &Quantity::new_rational(50_000f64).unwrap() { return Err(EvalError::TooBig); }

				let mut prod = Quantity::new_rational(1f64).unwrap();
				let mut u = v.clone();
				while u > Quantity::new_rational(0f64).unwrap() {
					prod *= u.clone();
					u = u - Quantity::new_rational(1f64).unwrap();
				}

				return Ok(Some(Expression::Quantity(prod)));
			} else { return Ok(None); }
		}
	};
}