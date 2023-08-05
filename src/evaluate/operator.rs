use std::collections::VecDeque;
use crate::parser::LineLocation;
use crate::quantity::Quantity;
use crate::parser::Operator;
use crate::parser::Expression;
use crate::context::Context;
use crate::errors::DaisyError;


pub fn eval_operator(g: &Expression, _context: &mut Context) -> Result<Option<Expression>, (LineLocation, DaisyError)> {

	let Expression::Operator(op_loc, op, args) = g else {panic!()};

	match op {
		Operator::Function(_) => unreachable!("Functions are handled seperately."),
		Operator::UserFunction(_) => unimplemented!(),

		Operator::Tuple => {
			if args.len() != 2 { panic!() };
			let a = &args[0];
			let b = &args[1];

			let mut loc = *op_loc;
			let mut vec: VecDeque<Expression> = VecDeque::new();
			if let Expression::Tuple(l, v) = a {
				loc += *l;
				for i in v { vec.push_back(i.clone()) }
			} else {
				loc += a.get_linelocation();
				vec.push_back(a.clone())
			}

			if let Expression::Tuple(l, v) = b {
				loc += *l;
				for i in v { vec.push_back(i.clone()) }
			} else {
				loc += b.get_linelocation();
				vec.push_back(b.clone())
			}

			return Ok(Some(Expression::Tuple(loc, vec)))
		}

		Operator::Negative => {
			if args.len() != 1 { panic!() };
			let args = &args[0];

			if let Expression::Quantity(l, v) = args {
				return Ok(Some(Expression::Quantity(*l + *op_loc, -v.clone())));
			} else { return Ok(None); }
		},

		Operator::Add => {
			if args.len() != 2 { panic!() };
			let a = &args[0];
			let b = &args[1];

			if let Expression::Quantity(la, a) = a {
				if let Expression::Quantity(lb, b) = b {
					if !a.unit.compatible_with(&b.unit) {
						return Err((
							*la + *lb + *op_loc,
							DaisyError::IncompatibleUnits(
								a.convert_to_base().unit.to_string(),
								b.convert_to_base().unit.to_string()
							)
						));
					}
					return Ok(Some(Expression::Quantity(*la + *lb + *op_loc, a.clone() + b.clone())));
				}
			}

			return Ok(None);
		},

		Operator::Subtract => {
			if args.len() != 2 { panic!() };
			let a = &args[0];
			let b = &args[1];

			if let Expression::Quantity(la, a) = a {
				if let Expression::Quantity(lb, b) = b {
					if !a.unit.compatible_with(&b.unit) {
						return Err((
							*la + *lb + *op_loc,
							DaisyError::IncompatibleUnits(
								a.convert_to_base().unit.to_string(),
								b.convert_to_base().unit.to_string()
							)
						));
					}
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
					if b.is_zero() { return Err((*la + *lb + *op_loc, DaisyError::ZeroDivision)); }
					return Ok(Some(Expression::Quantity(*la + *lb + *op_loc, a.clone() / b.clone())));
				}
			}

			return Ok(None);
		},

		Operator::ImplicitMultiply |
		Operator::Multiply => {
			if args.len() != 2 { panic!() };
			let a = &args[0];
			let b = &args[1];

			if let Expression::Quantity(la, a) = a {
				if let Expression::Quantity(lb, b) = b {
					return Ok(Some(Expression::Quantity(*la + *lb + *op_loc, a.clone() * b.clone())));
				}
			}

			return Ok(None);
		},

		Operator::ModuloLong
		| Operator::Modulo => {
			if args.len() != 2 { panic!() };
			let a = &args[0];
			let b = &args[1];

			if let Expression::Quantity(la, va) = a {
				if let Expression::Quantity(lb, vb) = b {

					if !(va.unitless() && vb.unitless()) {
						return Err((*la + *lb + *op_loc, DaisyError::IncompatibleUnit));
					}

					if vb <= &Quantity::new_rational(1f64).unwrap() { return Err((*la + *lb + *op_loc, DaisyError::BadMath)); }
					if va.fract() != Quantity::new_rational(0f64).unwrap() { return Err((*la + *lb + *op_loc, DaisyError::BadMath)); }
					if vb.fract() != Quantity::new_rational(0f64).unwrap() { return Err((*la + *lb + *op_loc, DaisyError::BadMath)); }

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
							DaisyError::IncompatibleUnits(
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
				if v.is_negative() { return Err((*l + *op_loc, DaisyError::BadMath)); }
				let p = v.pow(Quantity::new_rational_from_string("0.5").unwrap());
				if p.is_nan() {return Err((*l + *op_loc, DaisyError::BadMath));}
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
						return Err((*lb, DaisyError::IncompatibleUnit));
					}

					if va.is_zero() && vb.is_negative() {
						return Err((*la + *lb + *op_loc, DaisyError::ZeroDivision));
					}

					let p = va.pow(vb.clone());
					if p.is_nan() {return Err((*la + *lb + *op_loc, DaisyError::BadMath));}
					return Ok(Some(Expression::Quantity(*la + *lb + *op_loc, p)));
				} else { return Ok(None); }
			} else { return Ok(None); }
		},

		Operator::Factorial => {
			if args.len() != 1 {panic!()};
			let args = &args[0];

			if let Expression::Quantity(l, v) = args {

				if !v.unitless() {
					return Err((*l + *op_loc, DaisyError::IncompatibleUnit));
				}

				if !v.fract().is_zero() { return Err((*l + *op_loc, DaisyError::BadMath)); }
				if v > &Quantity::new_rational(50_000f64).unwrap() { return Err((*l + *op_loc, DaisyError::TooBig)); }

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