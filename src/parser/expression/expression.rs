use std::collections::VecDeque;
use crate::quantity::Quantity;

use super::Operator;
use super::Constant;
use super::super::LineLocation;


/// Expressions represent logical objects in an expession.
#[derive(Debug)]
#[derive(Clone)]
pub enum Expression {
	Variable(LineLocation, String),
	Quantity(LineLocation, Quantity),
	Constant(LineLocation, Constant),
	Operator(LineLocation, Operator, VecDeque<Expression>),
}

impl ToString for Expression {
	fn to_string(&self) -> String {
		match self {
			Expression::Quantity(_, v) => v.to_string(),
			Expression::Constant(_, c) => c.to_string(),
			Expression::Variable(_, s) => s.clone(),
			Expression::Operator(_, o,a) => o.print(a)
		}
	}
}

impl Expression {
	// This is called only when this is the outermost Expression.
	// This sometimes leads to different--usually more verbose--behavior.
	pub fn to_string_outer(&self) -> String {
		match self {
			Expression::Quantity(_, v) => v.to_string_outer(),
			Expression::Constant(_, c) => c.to_string(),
			Expression::Variable(_, s) => s.clone(),
			Expression::Operator(_, o,a) => o.print(a)
		}
	}

	pub fn is_quantity(&self) -> bool {
		match self {
			Expression::Quantity(_,_) => true,
			_ => false
		}
	}

	// True if this is a power operator applied to a constant or variable
	// and an integer.
	// Examples: pi^2, x ^ 3
	pub fn is_poly_power(&self) -> bool {
		match self {
			Expression::Operator(_, Operator::Power, a) => {
				// Assuming len(a) = 2, which should be true in this case
				assert!(a.len() == 2);

				let base = &a[0];
				let power = &a[1];

				// Make sure base ks const or variable
				match base {
					Expression::Constant(_, _)
					| Expression::Variable(_, _)
					=> {},

					_ => { return false; }
				};

				// Make sure power is an integer
				match power {
					Expression::Quantity(_, q) => {
						return q.unitless() && q.fract().is_zero();
					},
					_ => { return false; }
				}


			},
			_ => { return false; }
		};
	}

	#[inline(always)]
	pub fn get_args_mut(&mut self) -> Option<&mut VecDeque<Expression>> {
		match self {
			Expression::Operator(_, _, ref mut a) => Some(a),
			_ => None
		}
	}

	#[inline(always)]
	pub fn get_args(&self) -> Option<&VecDeque<Expression>> {
		match self {
			Expression::Operator(_, _, ref a) => Some(a),
			_ => None
		}
	}

	#[inline(always)]
	pub fn get_at_coords<'a, 'b, I>(&'a self, coords: I) -> Option<&'a Expression>
	where I: IntoIterator<Item = &'b usize> + Sized {
		let mut g = self;
		for t in coords.into_iter() {
			let args = g.get_args();
			let Some(args) = args else { return None; };
			g = &args[*t];
		}
		return Some(g);
	}

	#[inline(always)]
	pub fn get_at_coords_mut<'a, 'b, I>(&'a mut self, coords: I) -> Option<&'a mut Expression>
	where I: IntoIterator<Item = &'b usize> + Sized {
		let mut g = self;
		for t in coords.into_iter() {
			let args = g.get_args_mut();
			let Some(args) = args else { return None; };
			g = &mut args[*t];
		}
		return Some(g);
	}

	pub fn get_linelocation(&self) -> LineLocation {
		match self {
			Expression::Quantity(l, _)
			| Expression::Constant(l, _)
			| Expression::Variable(l, _)
			| Expression::Operator(l, _,_)
			=> { *l }
		}
	}

	pub fn set_linelocation(&mut self, loc: &LineLocation) {
		match self {
			Expression::Quantity(l, _) => { *l = *loc },
			Expression::Constant(l, _) => { *l = *loc },
			Expression::Variable(l, _) => { *l = *loc },
			Expression::Operator(l, _,_) => { *l = *loc },
		}
	}
}