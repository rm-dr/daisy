use crate::parser::LineLocation;
use crate::quantity::Quantity;
use crate::parser::Operator;
use crate::parser::Expression;
use super::EvalError;
use crate::context::Context;


pub fn eval_operator(g: &Expression, _context: &mut Context) -> Result<Option<Expression>, (LineLocation, EvalError)> {

	let Expression::Operator(op_loc, op, args) = g else {panic!()};

	match op {
		Operator::Function(_) => unreachable!("Functions are handled seperately."),

		Operator::Negative => {
			if args.len() != 1 { panic!() };
			let args = &args[0];

			if let Expression::Quantity(l, v) = args {
				return Ok(Some(Expression::Quantity(*l + *op_loc, -v.clone())));
			} else { return Ok(None); }
		},

		Operator::Add => {
			let mut sum: Quantity;
			let mut loc: LineLocation;
			if let Expression::Quantity(l, s) = &args[0] {
				sum = s.clone();
				loc = *l;
			} else { return Ok(None); };


			// Flag that is set to true if we find incompatible units.
			// We don't stop right away because we need to add all linelocations
			// to show a pretty error.
			let mut incompatible_units = false;

			let mut i: usize = 1;
			while i < args.len() {
				let j = &args[i];
				if let Expression::Quantity(l, v) = j {

					if !sum.unit.compatible_with(&v.unit) {
						incompatible_units = true;
					}

					if !incompatible_units { sum += v.clone(); }
					loc += *l;
				} else {
					if incompatible_units {
						return Err((loc + *op_loc, EvalError::IncompatibleUnit));
					}
					return Ok(None);
				}
				i += 1;
			}

			if incompatible_units {
				return Err((loc + *op_loc, EvalError::IncompatibleUnit));
			}

			return Ok(Some(Expression::Quantity(loc + *op_loc, sum)));
		},

		Operator::Subtract => {
			if args.len() != 2 { panic!() };
			let a = &args[0];
			let b = &args[1];

			if let Expression::Quantity(la, a) = a {
				if let Expression::Quantity(lb, b) = b {
					return Ok(Some(Expression::Quantity(*la + *lb + *op_loc, a.clone() - b.clone())));
				}
			}

			return Ok(None);
		},


		Operator::Divide |
		Operator::DivideLong => {
			if args.len() != 2 { panic!() };
			let a = &args[0];
			let b = &args[1];

			if let Expression::Quantity(la, a) = a {
				if let Expression::Quantity(lb, b) = b {
					if b.is_zero() { return Err((*la + *lb + *op_loc, EvalError::ZeroDivision)); }
					return Ok(Some(Expression::Quantity(*la + *lb + *op_loc, a.clone() / b.clone())));
				}
			}

			return Ok(None);
		},

		Operator::ImplicitMultiply |
		Operator::Multiply => {
			let mut prod: Quantity;
			let mut loc: LineLocation;
			if let Expression::Quantity(l, s) = &args[0] {
				prod = s.clone();
				loc = *l;
			} else { return Ok(None); };

			let mut i: usize = 1;
			while i < args.len() {
				let j = &args[i];
				if let Expression::Quantity(l, v) = j {
					prod *= v.clone();
					loc += *l;
				} else { return Ok(None); }
				i += 1;
			}
			return Ok(Some(Expression::Quantity(loc + *op_loc, prod)));
		},

		Operator::ModuloLong
		| Operator::Modulo => {
			if args.len() != 2 { panic!() };
			let a = &args[0];
			let b = &args[1];

			if let Expression::Quantity(la, va) = a {
				if let Expression::Quantity(lb, vb) = b {

					if !(va.unitless() && vb.unitless()) {
						return Err((*la + *lb + *op_loc, EvalError::IncompatibleUnit));
					}

					if vb <= &Quantity::new_rational(1f64).unwrap() { return Err((*la + *lb + *op_loc, EvalError::BadMath)); }
					if va.fract() != Quantity::new_rational(0f64).unwrap() { return Err((*la + *lb + *op_loc, EvalError::BadMath)); }
					if vb.fract() != Quantity::new_rational(0f64).unwrap() { return Err((*la + *lb + *op_loc, EvalError::BadMath)); }

					return Ok(Some(Expression::Quantity(*la + *lb + *op_loc, va.clone() % vb.clone())));
				} else { return Ok(None); }
			} else { return Ok(None); }
		},

		Operator::UnitConvert => {
			if args.len() != 2 { panic!() };
			let a = &args[0];
			let b = &args[1];

			if let Expression::Quantity(la, va) = a {
				if let Expression::Quantity(lb, vb) = b {
					let n = va.clone().convert_to(vb.clone());
					if n.is_none() {
						return Err((
							*la + *lb + *op_loc,
							EvalError::IncompatibleUnits(
								va.convert_to_base().unit.to_string(),
								vb.convert_to_base().unit.to_string()
							)
						));
					}
					return Ok(Some(Expression::Quantity(*la + *lb + *op_loc, n.unwrap())));
				} else { return Ok(None); }
			} else { return Ok(None); }
		},


		Operator::Sqrt => {
			if args.len() != 1 { panic!() }
			let a = &args[0];

			if let Expression::Quantity(l, v) = a {
				if v.is_negative() { return Err((*l + *op_loc, EvalError::BadMath)); }
				let p = v.pow(Quantity::new_rational_from_string("0.5").unwrap());
				if p.is_nan() {return Err((*l + *op_loc, EvalError::BadMath));}
				return Ok(Some(Expression::Quantity(*l, p)));
			} else { return Ok(None); }
		},

		Operator::Power => {
			if args.len() != 2 {panic!()};
			let a = &args[0];
			let b = &args[1];

			if let Expression::Quantity(la, va) = a {
				if let Expression::Quantity(lb, vb) = b {

					if !vb.unitless() {
						return Err((*lb, EvalError::IncompatibleUnit));
					}

					if va.is_zero() && vb.is_negative() {
						return Err((*la + *lb + *op_loc, EvalError::ZeroDivision));
					}

					let p = va.pow(vb.clone());
					if p.is_nan() {return Err((*la + *lb + *op_loc, EvalError::BadMath));}
					return Ok(Some(Expression::Quantity(*la + *lb + *op_loc, p)));
				} else { return Ok(None); }
			} else { return Ok(None); }
		},

		Operator::Factorial => {
			if args.len() != 1 {panic!()};
			let args = &args[0];

			if let Expression::Quantity(l, v) = args {

				if !v.unitless() {
					return Err((*l + *op_loc, EvalError::IncompatibleUnit));
				}

				if !v.fract().is_zero() { return Err((*l + *op_loc, EvalError::BadMath)); }
				if v > &Quantity::new_rational(50_000f64).unwrap() { return Err((*l + *op_loc, EvalError::TooBig)); }

				let mut prod = Quantity::new_rational(1f64).unwrap();
				let mut u = v.clone();
				while u > Quantity::new_rational(0f64).unwrap() {
					prod *= u.clone();
					u = u - Quantity::new_rational(1f64).unwrap();
				}

				return Ok(Some(Expression::Quantity(*l + *op_loc, prod)));
			} else { return Ok(None); }
		}
	};
}