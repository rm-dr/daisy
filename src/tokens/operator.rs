use std::collections::VecDeque;
use std::cmp::Ordering;

use crate::tokens::Token;
use crate::tokens::Function;
use crate::quantity::Quantity;

/// Operator types, in order of increasing priority.
#[derive(Debug)]
#[derive(Clone)]
#[repr(usize)]
pub enum Operator {
	ModuloLong = 0, // Mod invoked with "mod"
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
	fn add_parens_to_arg(&self, arg: &Token) -> String {
		let mut astr: String = arg.print();
		if let Token::Operator(o,_) = arg {
			if o < self {
				astr = format!("({})", astr);
			}
		}
		return astr;
	}

	#[inline(always)]
	fn add_parens_to_arg_strict(&self, arg: &Token) -> String {
		let mut astr: String = arg.print();
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
				return format!("1/{}", Operator::Divide.add_parens_to_arg(&args[0]));
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
				let mut b = &args[1];
				let mut sub = false;

				let tmp;

				if let Token::Operator(o,ar) = b {
					if let Operator::Negative = o {
						sub = true;
						b = &ar[0];
					}
				} else if let Token::Number(q) = b {
					if q.is_negative() {
						sub = true;
						tmp = Token::Number(-q.clone());
						b = &tmp;
					}
				}
				let (b, sub) = (b, sub);

				if sub {
					return format!("{} - {}", self.add_parens_to_arg(a), self.add_parens_to_arg(b));
				} else {
					return format!("{} + {}", self.add_parens_to_arg(a), self.add_parens_to_arg(b));
				}
			},

			Operator::Multiply => {
				let a = &args[0];
				let mut b = &args[1];
				let mut div = false;

				if let Token::Operator(o,ar) = b {
					if let Operator::Flip = o {
						div = true;
						b = &ar[0];
					}
				}
				let (b, div) = (b, div);

				if div {
					return format!("{} ÷ {}", self.add_parens_to_arg_strict(a), self.add_parens_to_arg_strict(b));
				} else {
					return format!("{} × {}", self.add_parens_to_arg_strict(a), self.add_parens_to_arg_strict(b));
				}
			},

			Operator::Function(s) => {
				return format!("{}({})", s.to_string(), args[0].print());
			}
		};
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
			"^"|"**" => {Some( Operator::Power )},
			"!"      => {Some( Operator::Factorial )},
			"sqrt"|"rt"|"√" => {Some( Operator::Sqrt )},

			"abs"    => {Some( Operator::Function(Function::Abs) )},
			"floor"    => {Some( Operator::Function(Function::Floor) )},
			"ceil"    => {Some( Operator::Function(Function::Ceil) )},
			"round"    => {Some( Operator::Function(Function::Round) )},
			"ln"    => {Some( Operator::Function(Function::NaturalLog) )},
			"log"    => {Some( Operator::Function(Function::TenLog) )},
			"sin"    => {Some( Operator::Function(Function::Sin) )},
			"cos"    => {Some( Operator::Function(Function::Cos) )},
			"tan"    => {Some( Operator::Function(Function::Tan) )},
			"asin"    => {Some( Operator::Function(Function::Asin) )},
			"acos"    => {Some( Operator::Function(Function::Acos) )},
			"atan"    => {Some( Operator::Function(Function::Atan) )},
			"csc"    => {Some( Operator::Function(Function::Csc) )},
			"sec"    => {Some( Operator::Function(Function::Sec) )},
			"cot"    => {Some( Operator::Function(Function::Cot) )},
			"sinh"    => {Some( Operator::Function(Function::Sinh) )},
			"cosh"    => {Some( Operator::Function(Function::Cosh) )},
			"tanh"    => {Some( Operator::Function(Function::Tanh) )},
			"asinh"    => {Some( Operator::Function(Function::Asinh) )},
			"acosh"    => {Some( Operator::Function(Function::Acosh) )},
			"atanh"    => {Some( Operator::Function(Function::Atanh) )},
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
	pub fn as_int(&self) -> usize {
		unsafe { *<*const _>::from(self).cast::<usize>() }
	}


	#[inline(always)]
	pub fn into_token(self, mut args: VecDeque<Token>) -> Token {
		match self {
			Operator::Subtract => {
				if args.len() != 2 { panic!() }
				let a = args.pop_front().unwrap();
				let b = args.pop_front().unwrap();

				let b_new;
				if let Token::Number(q) = b {
					b_new = Token::Number(-q);
				} else {
					b_new = Token::Operator(Operator::Negative, VecDeque::from(vec!(b)));
				}

				Token::Operator(
					Operator::Add,
					VecDeque::from(vec!(a,b_new))
				)
			},

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
					VecDeque::from(vec!(a, Token::Number(Quantity::new_rational(0.5).unwrap())))
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
			=> { Token::Operator(self, args) },
		}
	}
}

impl Operator{
	pub fn apply(&self, args: &VecDeque<Token>) -> Result<Token, ()> {
		match self {
			Operator::ImplicitMultiply |
			Operator::Sqrt |
			Operator::Divide |
			Operator::Subtract => { panic!() }

			Operator::Negative => {
				if args.len() != 1 {panic!()};
				let args = args[0].as_number();

				if let Token::Number(v) = args {
					return Ok(Token::Number(-v));
				} else { panic!(); }
			},

			Operator::Flip => {
				if args.len() != 1 {panic!()};
				let args = args[0].as_number();

				if let Token::Number(v) = args {
					if v.is_zero() { return Err(()); }
					return Ok(Token::Number(Quantity::new_rational(1f64).unwrap()/v));
				} else { panic!(); }
			},

			Operator::Add => {
				let mut sum = Quantity::new_rational(0f64).unwrap();
				for i in args.iter() {
					let j = i.as_number();
					if let Token::Number(v) = j {
						sum += v;
					} else {
						panic!();
					}
				}
				return Ok(Token::Number(sum));
			},

			Operator::Multiply => {
				let mut prod = Quantity::new_rational(1f64).unwrap();
				for i in args.iter() {
					let j = i.as_number();
					if let Token::Number(v) = j {
						prod *= v;
					} else {
						panic!();
					}
				}
				return Ok(Token::Number(prod));
			},

			Operator::ModuloLong
			| Operator::Modulo => {
				if args.len() != 2 {panic!()};
				let a = args[0].as_number();
				let b = args[1].as_number();

				if let Token::Number(va) = a {
					if let Token::Number(vb) = b {
						if vb <= Quantity::new_rational(1f64).unwrap() { return Err(()); }
						if va.fract() != Quantity::new_rational(0f64).unwrap() { return Err(()); }
						if vb.fract() != Quantity::new_rational(0f64).unwrap() { return Err(()); }

						return Ok(Token::Number(va%vb));
					} else { panic!(); }
				} else { panic!(); }
			},

			Operator::Power => {
				if args.len() != 2 {panic!()};
				let a = args[0].as_number();
				let b = args[1].as_number();

				if let Token::Number(va) = a {
					if let Token::Number(vb) = b {
						let p = va.pow(vb);
						if p.is_nan() {return Err(());}
						return Ok(Token::Number(p));
					} else { panic!(); }
				} else { panic!(); }
			},

			Operator::Factorial => {
				if args.len() != 1 {panic!()};
				let args = args[0].as_number();

				if let Token::Number(v) = args {
					if !v.fract().is_zero() { return Err(()); }
					if v > Quantity::new_rational(50_000f64).unwrap() { return Err(()); }

					let mut prod = Quantity::new_rational(1f64).unwrap();
					let mut u = v.clone();
					while u > Quantity::new_rational(0f64).unwrap() {
						prod *= u.clone();
						u = u - Quantity::new_rational(1f64).unwrap();
					}

					return Ok(Token::Number(prod));
				} else { panic!(); }
			},

			Operator::Function(f) => {
				return f.apply(args);
			}
		};
	}

}