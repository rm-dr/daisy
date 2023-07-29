use std::cmp::Ordering;
use std::collections::VecDeque;

use super::Expression;
use super::Function;


/// Operator types, in order of increasing priority.
#[derive(Debug)]
#[derive(Clone)]
#[repr(usize)]
pub enum Operator {
	Define = 0, // Variable and function definition
	ModuloLong, // Mod invoked with "mod"
	DivideLong,
	UnitConvert,
	Subtract,
	Add,
	Divide,
	Multiply,
	Modulo, // Mod invoked with %
	Negative,

	Sqrt,
	ImplicitMultiply,

	Power,
	Factorial,

	Function(Function),
}

impl PartialEq for Operator {
	fn eq(&self, other: &Self) -> bool {
		self.as_int() == other.as_int()
	}
}

impl PartialOrd for Operator {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		match (self, other) {
			(Operator::Add, Operator::Subtract)
			| (Operator::Subtract, Operator::Add)
			| (Operator::Multiply, Operator::Divide)
			| (Operator::Divide, Operator::Multiply)
			=> {Some(Ordering::Equal)}

			_ => { self.as_int().partial_cmp(&other.as_int()) }
		}
	}
}

impl Operator {
	#[inline(always)]
	pub fn as_int(&self) -> usize {
		unsafe { *<*const _>::from(self).cast::<usize>() }
	}

	#[inline(always)]
	pub fn from_string(s: &str) -> Option<Operator> {

		let f = Function::from_string(s);
		if let Some(f) = f {
			return Some(Operator::Function(f));
		}

		return match s {
			"="      => {Some( Operator::Define )},
			"+"      => {Some( Operator::Add )},
			"-"      => {Some( Operator::Subtract )},
			"neg"    => {Some( Operator::Negative )},
			"*"|"×"  => {Some( Operator::Multiply )},
			"/"|"÷"  => {Some( Operator::Divide )},
			"i*"     => {Some( Operator::ImplicitMultiply )},
			"%"      => {Some( Operator::Modulo )},
			"mod"    => {Some( Operator::ModuloLong )},
			"per"    => {Some( Operator::DivideLong )},
			"to"    => {Some( Operator::UnitConvert )},
			"^"|"**" => {Some( Operator::Power )},
			"!"      => {Some( Operator::Factorial )},
			"sqrt"|"rt"|"√" => {Some( Operator::Sqrt )},

			_ => None
		};
	}

	#[inline(always)]
	pub fn is_binary(&self) -> bool {
		match self {
			Operator::Negative
			| Operator::Factorial
			| Operator::Sqrt
			| Operator::Function(_)
			=> false,
			_ => true
		}
	}

	#[inline(always)]
	pub fn is_left_associative(&self) -> bool {
		match self {
			Operator::Negative
			| Operator::Sqrt
			| Operator::Function(_)
			=> false,
			_ => true
		}
	}

	#[inline(always)]
	fn add_parens_to_arg(&self, arg: &Expression) -> String {
		let mut astr: String = arg.to_string();
		if let Expression::Operator(_, o,_) = arg {
			if o < self {
				astr = format!("({})", astr);
			}
		}
		return astr;
	}

	#[inline(always)]
	fn add_parens_to_arg_strict(&self, arg: &Expression) -> String {
		let mut astr: String = arg.to_string();
		if let Expression::Operator(_, o,_) = arg {
			if o <= self {
				astr = format!("({})", astr);
			}
		}
		return astr;
	}


	pub fn print(&self, args: &VecDeque<Expression>) -> String {
		match self {
			Operator::Define => {
				return format!(
					"{} = {}",
					self.add_parens_to_arg(&args[0]),
					self.add_parens_to_arg(&args[1])
				);
			},

			Operator::Negative => {
				return format!("-{}", self.add_parens_to_arg(&args[0]));
			},

			Operator::Sqrt => {
				return format!(
					"√{}",
					self.add_parens_to_arg(&args[0]),
				);
			},

			Operator::ModuloLong => {
				return format!(
					"{} mod {}",
					self.add_parens_to_arg(&args[0]),
					self.add_parens_to_arg(&args[1])
				);
			},

			Operator::DivideLong => {
				return format!(
					"{} per {}",
					self.add_parens_to_arg(&args[0]),
					self.add_parens_to_arg(&args[1])
				);
			},

			Operator::UnitConvert => {
				return format!(
					"{} to {}",
					self.add_parens_to_arg(&args[0]),
					self.add_parens_to_arg(&args[1])
				);
			},

			Operator::Modulo => {
				return format!(
					"{} % {}",
					self.add_parens_to_arg(&args[0]),
					self.add_parens_to_arg(&args[1])
				);
			},

			Operator::Subtract => {
				return format!(
					"{} - {}",
					self.add_parens_to_arg(&args[0]),
					self.add_parens_to_arg(&args[1])
				);
			},

			Operator::Power => {
				return format!(
					"{}^{}",
					self.add_parens_to_arg_strict(&args[0]),
					self.add_parens_to_arg_strict(&args[1])
				);
			},

			Operator::Factorial => {
				return format!("{}!", self.add_parens_to_arg(&args[0]));
			},

			Operator::Add => {
				return format!(
					"{} + {}",
					self.add_parens_to_arg(&args[0]),
					self.add_parens_to_arg(&args[1])
				);
			},

			Operator::ImplicitMultiply |
			Operator::Multiply => {
				let a = &args[0];
				let b = &args[1];

				// Omit times sign when we have a number
				// multiplied by a unit (like 10 m)
				// Times sign should stay in all other cases.
				let no_times = {
					if let Expression::Quantity(_, p) = a {
						if let Expression::Quantity(_, q) = b {
							p.unitless() && !q.unitless()
						} else {false}
					} else {false}
				};

				if no_times {
					let Expression::Quantity(_, u) = b else {panic!()};
					if u.unit.no_space() {
						return format!("{}{}",
							self.add_parens_to_arg_strict(a),
							self.add_parens_to_arg_strict(b)
						);
					} else {
						return format!("{} {}",
							self.add_parens_to_arg_strict(a),
							self.add_parens_to_arg_strict(b)
						);
					}
				} else {
					return format!("{} × {}",
						self.add_parens_to_arg_strict(a),
						self.add_parens_to_arg_strict(b)
					);
				}
			},

			Operator::Divide => {
				let a = &args[0];
				let b = &args[1];

				if let Expression::Quantity(_, q) = a {
					if q.is_one() {
						return format!("{}⁻¹",
							self.add_parens_to_arg_strict(b)
						);
					}
				}

				return format!("{} ÷ {}",
					self.add_parens_to_arg_strict(a),
					self.add_parens_to_arg_strict(b)
				);
			},

			Operator::Function(s) => {
				return format!("{}({})", s.to_string(), args[0].to_string());
			}
		};
	}


}