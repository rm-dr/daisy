use std::collections::VecDeque;

use crate::quantity::Quantity;
use crate::parser::Operator;
use crate::parser::Token;
use super::EvalError;

pub fn eval_operator(op: &Operator, args: &VecDeque<Token>) -> Result<Token, EvalError> {
	match op {

		// Handled seperately in evaluate.rs
		Operator::Function(_) |

		// These are never evaluated,
		// but are converted to one of the following instead.
		Operator::ImplicitMultiply |
		Operator::Sqrt |
		Operator::Divide |
		Operator::Subtract => { panic!() }

		Operator::Negative => {
			if args.len() != 1 {panic!()};
			let args = &args[0];

			if let Token::Quantity(v) = args {
				return Ok(Token::Quantity(-v.clone()));
			} else { panic!(); }
		},

		Operator::Flip => {
			if args.len() != 1 {panic!()};
			let args = &args[0];

			if let Token::Quantity(v) = args {
				if v.is_zero() { return Err(EvalError::ZeroDivision); }
				return Ok(Token::Quantity(
					Quantity::new_rational(1f64).unwrap()/v.clone()
				));
			} else { panic!(); }
		},

		Operator::Add => {
			let mut sum: Quantity;
			if let Token::Quantity(s) = &args[0] {
				sum = s.clone();
			} else {panic!()};

			let mut i: usize = 1;
			while i < args.len() {
				let j = &args[i];
				if let Token::Quantity(v) = j {

					if !sum.unit.compatible_with(&v.unit) {
						return Err(EvalError::IncompatibleUnit);
					}

					sum += v.clone();
				} else {
					panic!();
				}
				i += 1;
			}
			return Ok(Token::Quantity(sum));
		},

		Operator::Multiply => {
			let mut prod = Quantity::new_rational(1f64).unwrap();
			for i in args.iter() {
				let j = i;
				if let Token::Quantity(v) = j {
					prod *= v.clone();
				} else {
					panic!();
				}
			}
			return Ok(Token::Quantity(prod));
		},

		Operator::ModuloLong
		| Operator::Modulo => {
			if args.len() != 2 {panic!()};
			let a = &args[0];
			let b = &args[1];

			if let Token::Quantity(va) = a {
				if let Token::Quantity(vb) = b {

					if !(va.unitless() && vb.unitless()) {
						return Err(EvalError::IncompatibleUnit);
					}

					if vb <= &Quantity::new_rational(1f64).unwrap() { return Err(EvalError::BadMath); }
					if va.fract() != Quantity::new_rational(0f64).unwrap() { return Err(EvalError::BadMath); }
					if vb.fract() != Quantity::new_rational(0f64).unwrap() { return Err(EvalError::BadMath); }

					return Ok(Token::Quantity(va.clone() % vb.clone()));
				} else { panic!(); }
			} else { panic!(); }
		},

		Operator::UnitConvert
		=> {
			if args.len() != 2 {panic!()};
			let a = &args[0];
			let b = &args[1];

			if let Token::Quantity(va) = a {
				if let Token::Quantity(vb) = b {
					let n = va.clone().convert_to(vb.clone());
					if n.is_none() {
						return Err(EvalError::IncompatibleUnit);
					}
					return Ok(Token::Quantity(n.unwrap()));
				} else { panic!(); }
			} else { panic!(); }
		},

		Operator::Power => {
			if args.len() != 2 {panic!()};
			let a = &args[0];
			let b = &args[1];

			if let Token::Quantity(va) = a {
				if let Token::Quantity(vb) = b {

					if !vb.unitless() {
						return Err(EvalError::IncompatibleUnit);
					}

					if va.is_zero() && vb.is_negative() {
						return Err(EvalError::ZeroDivision);
					}

					let p = va.pow(vb.clone());
					if p.is_nan() {return Err(EvalError::BadMath);}
					return Ok(Token::Quantity(p));
				} else { panic!(); }
			} else { panic!(); }
		},

		Operator::Factorial => {
			if args.len() != 1 {panic!()};
			let args = &args[0];

			if let Token::Quantity(v) = args {

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

				return Ok(Token::Quantity(prod));
			} else { panic!(); }
		}
	};
}