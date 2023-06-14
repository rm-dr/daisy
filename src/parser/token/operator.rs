use std::cmp::Ordering;
use std::collections::VecDeque;
use crate::quantity::Quantity;

use super::Token;
use super::Function;


/// Operator types, in order of increasing priority.
#[derive(Debug)]
#[derive(Clone)]
#[repr(usize)]
pub enum Operator {
	ModuloLong = 0, // Mod invoked with "mod"
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

	// Not accessible from prompt
	Flip,
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
		match s {
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

			"abs"     => {Some( Operator::Function(Function::Abs) )},
			"floor"   => {Some( Operator::Function(Function::Floor) )},
			"ceil"    => {Some( Operator::Function(Function::Ceil) )},
			"round"   => {Some( Operator::Function(Function::Round) )},
			"ln"      => {Some( Operator::Function(Function::NaturalLog) )},
			"log"     => {Some( Operator::Function(Function::TenLog) )},
			"sin"     => {Some( Operator::Function(Function::Sin) )},
			"cos"     => {Some( Operator::Function(Function::Cos) )},
			"tan"     => {Some( Operator::Function(Function::Tan) )},
			"asin"    => {Some( Operator::Function(Function::Asin) )},
			"acos"    => {Some( Operator::Function(Function::Acos) )},
			"atan"    => {Some( Operator::Function(Function::Atan) )},
			"csc"     => {Some( Operator::Function(Function::Csc) )},
			"secant"  => {Some( Operator::Function(Function::Sec) )},
			"cot"     => {Some( Operator::Function(Function::Cot) )},
			"sinh"    => {Some( Operator::Function(Function::Sinh) )},
			"cosh"    => {Some( Operator::Function(Function::Cosh) )},
			"tanh"    => {Some( Operator::Function(Function::Tanh) )},
			"asinh"   => {Some( Operator::Function(Function::Asinh) )},
			"acosh"   => {Some( Operator::Function(Function::Acosh) )},
			"atanh"   => {Some( Operator::Function(Function::Atanh) )},
			"csch"    => {Some( Operator::Function(Function::Csch) )},
			"sech"    => {Some( Operator::Function(Function::Sech) )},
			"coth"    => {Some( Operator::Function(Function::Coth) )},
			_ => None
		}
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
	pub fn into_token(self, mut args: VecDeque<Token>) -> Token {
		match self {
			Operator::Subtract => {
				if args.len() != 2 { panic!() }
				let a = args.pop_front().unwrap();
				let b = args.pop_front().unwrap();

				let b_new;
				if let Token::Quantity(q) = b {
					b_new = Token::Quantity(-q);
				} else {
					b_new = Token::Operator(Operator::Negative, VecDeque::from(vec!(b)));
				}

				Token::Operator(
					Operator::Add,
					VecDeque::from(vec!(a,b_new))
				)
			},

			Operator::DivideLong |
			Operator::Divide => {
				if args.len() != 2 { panic!() }
				let a = args.pop_front().unwrap();
				let b = args.pop_front().unwrap();
				let b = Token::Operator(Operator::Flip, VecDeque::from(vec!(b)));

				Token::Operator(
					Operator::Multiply,
					VecDeque::from(vec!(a,b))
				)
			},

			Operator::Sqrt => {
				if args.len() != 1 { panic!() }
				let a = args.pop_front().unwrap();

				Token::Operator(
					Operator::Power,
					VecDeque::from(vec!(a, Token::Quantity(Quantity::new_rational_from_string("0.5").unwrap())))
				)
			},

			Operator::ImplicitMultiply
			=> { Token::Operator(Operator::Multiply, args) },

			Operator::Function(_)
			| Operator::Factorial
			| Operator::Negative
			| Operator::Flip
			| Operator::Add
			| Operator::Multiply
			| Operator::Modulo
			| Operator::Power
			| Operator::ModuloLong
			| Operator::UnitConvert
			=> { Token::Operator(self, args) },
		}
	}


	#[inline(always)]
	fn add_parens_to_arg(&self, arg: &Token) -> String {
		let mut astr: String = arg.to_string();
		if let Token::Operator(o,_) = arg {
			if o < self {
				astr = format!("({})", astr);
			}
		}
		return astr;
	}

	#[inline(always)]
	fn add_parens_to_arg_strict(&self, arg: &Token) -> String {
		let mut astr: String = arg.to_string();
		if let Token::Operator(o,_) = arg {
			if o <= self {
				astr = format!("({})", astr);
			}
		}
		return astr;
	}


	pub fn print(&self, args: &VecDeque<Token>) -> String {
		match self {
			Operator::ImplicitMultiply |
			Operator::Sqrt |
			Operator::Divide |
			Operator::Subtract => { panic!() }

			Operator::Flip => {
				return format!("{}⁻¹", Operator::Divide.add_parens_to_arg(&args[0]));
			},

			Operator::Negative => {
				return format!("-{}", self.add_parens_to_arg(&args[0]));
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
				let a = &args[0];

				let b; let sub;
				if let Token::Operator(o, ar) = &args[1] {
					if let Operator::Negative = o {
						sub = true;
						b = &ar[0];
					} else { sub = false; b = &args[1]; }
				} else { sub = false; b = &args[1]; }

				if sub {
					return format!(
						"{} - {}",
						self.add_parens_to_arg(a),
						self.add_parens_to_arg(b)
					);
				} else {
					return format!(
						"{} + {}",
						self.add_parens_to_arg(a),
						self.add_parens_to_arg(b)
					);
				}
			},

			Operator::Multiply => {
				let a = &args[0];

				let b; let div;
				if let Token::Operator(o, ar) = &args[1] {
					if let Operator::Flip = o {
						div = true;
						b = &ar[0];
					} else { div = false; b = &args[1]; }
				} else { div = false; b = &args[1]; }

				// Division symbol case
				if div {
					return format!("{} ÷ {}",
						self.add_parens_to_arg_strict(a),
						self.add_parens_to_arg_strict(b)
					);
				}


				// Omit times sign when we have a number
				// multiplied by a unit (like 10 m)
				// Times sign should stay in all other cases.
				let no_times = {
					if let Token::Quantity(p) = a {
						if let Token::Quantity(q) = b {
							p.unitless() && !q.unitless()
						} else {false}
					} else {false}
				};

				if no_times {
					let Token::Quantity(u) = b else {panic!()};
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

			Operator::Function(s) => {
				return format!("{}({})", s.to_string(), args[0].to_string());
			}
		};
	}


}