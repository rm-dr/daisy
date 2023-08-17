use std::collections::VecDeque;
use crate::quantity::Quantity;
use crate::context::Context;

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
	Tuple(LineLocation, VecDeque<Expression>),
}

impl Expression {
	pub fn display(&self, context: &Context) -> String {
		match self {
			Expression::Quantity(_, v) => v.display(context),
			Expression::Constant(_, c) => c.to_string(),
			Expression::Variable(_, s) => s.clone(),
			Expression::Operator(_, o,a) => o.display(context, a),
			Expression::Tuple(_, v) => {
				format!("({})",
					v.iter()
						.map(|x| x.display(context))
						.collect::<Vec<String>>()
						.join(", ")
				)
			}
		}
	}
}

impl Expression {
	// This is called only when this is the outermost Expression.
	// This sometimes leads to different--usually more verbose--behavior.
	pub fn display_outer(&self, context: &Context) -> String {
		match self {
			Expression::Quantity(_, v) => v.display_outer(context),
			Expression::Constant(_, c) => c.to_string(),
			Expression::Variable(_, s) => s.clone(),
			Expression::Operator(_, o,a) => o.display(context, a),
			Expression::Tuple(_, v) => {
				format!("({})",
					v.iter()
						.map(|x| x.display(context))
						.collect::<Vec<String>>()
						.join(", ")
				)
			}
		}
	}

	pub fn is_quantity(&self) -> bool {
		match self {
			Expression::Quantity(_,_) => true,
			_ => false
		}
	}

	// True if this is a unitless integer
	pub fn is_unitless_integer(&self) -> bool {
		match self {
			Expression::Quantity(_, q) => {
				return q.unitless() && q.fract().is_zero();
			},
			Expression::Operator(_, Operator::Negative, q) => {
				assert!(q.len() == 1);
				let Expression::Quantity(_, q) = &q[0] else { return false };
				return q.unitless() && q.fract().is_zero();
			}
			_ => { return false; }
		}
	}

	// True if this is a integer power operator applied to a constant or variable.
	// Examples: pi^2, x ^ 3
	pub fn is_poly_power(&self) -> bool {
		match self {
			Expression::Operator(_, Operator::Power, a) => {
				// Assuming len(a) = 2, which should be true in this case
				assert!(a.len() == 2);

				let base = &a[0];
				let power = &a[1];

				// Make sure base is const or variable
				match base {
					Expression::Constant(_, _)
					| Expression::Variable(_, _)
					=> {},

					_ => { return false; }
				};

				// Make sure power is an integer
				return power.is_unitless_integer();
			},
			_ => { return false; }
		};
	}

	#[inline(always)]
	pub fn get_args_mut(&mut self) -> Option<&mut VecDeque<Expression>> {
		match self {
			Expression::Operator(_, _, ref mut a) => Some(a),
			Expression::Tuple(_, ref mut a) => Some(a),
			_ => None
		}
	}

	#[inline(always)]
	pub fn get_args(&self) -> Option<&VecDeque<Expression>> {
		match self {
			Expression::Operator(_, _, ref a) => Some(a),
			Expression::Tuple(_, ref a) => Some(a),
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
			| Expression::Tuple(l, _)
			=> { *l }
		}
	}

	pub fn set_linelocation(&mut self, loc: &LineLocation) {
		match self {
			Expression::Quantity(l, _) => { *l = *loc },
			Expression::Constant(l, _) => { *l = *loc },
			Expression::Variable(l, _) => { *l = *loc },
			Expression::Operator(l, _,_) => { *l = *loc },
			Expression::Tuple(l, _) => { *l = *loc },
		}
	}
}