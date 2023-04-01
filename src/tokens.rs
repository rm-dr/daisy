use std::collections::VecDeque;

use crate::quantity::Quantity;

/// Tokens represent logical objects in an expession.
/// 
/// Tokens starting with `Pre*` are intermediate tokens, and
/// will never show up in a fully-parsed expression tree.
#[derive(Debug)]
#[derive(Clone)]
pub enum Token {
	Number(Quantity),

	Constant(Quantity, String),

	Operator(
		Operator,
		VecDeque<Token>
	),
}

impl Token {

	pub fn print(&self) -> String {
		match self {
			Token::Number(v) => v.to_string(),
			Token::Constant(_,s) => s.clone(),
			Token::Operator(o,a) => o.print(a)
		}
	}

	#[inline(always)]
	pub fn get_args(&self) -> Option<&VecDeque<Token>> {
		match self {
			Token::Operator(_, ref a) => Some(a),
			_ => None
		}
	}

	#[inline(always)]
	pub fn get_args_mut(&mut self) -> Option<&mut VecDeque<Token>> {
		match self {
			Token::Operator(_, ref mut a) => Some(a),
			_ => None
		}
	}

	#[inline(always)]
	pub fn eval(&self) -> Result<Token, ()> {
		Ok(match self {
			Token::Number(_) => { self.clone() },
			Token::Constant(v,_) => { Token::Number(v.clone()) },
			Token::Operator(o,v) => { o.apply(&v)? }
		})
	}

	// Temporary solution
	#[inline(always)]
	pub fn as_number(&self) -> Token {
		match self {
			Token::Number(v) => { Token::Number(v.clone()) },
			Token::Constant(v,_) => { Token::Number(v.clone()) },
			_ => panic!()
		}
	}

}

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
#[derive(Debug)]
#[derive(Copy, Clone)]
pub enum Function {
	Abs,
	Floor,
	Ceil,
	Round,
	
	// TODO: Add arbitrary log
	NaturalLog,
	TenLog,

	Sin,
	Cos,
	Tan,
	Asin,
	Acos,
	Atan,
	Csc,
	Sec,
	Cot,

	Sinh,
	Cosh,
	Tanh,
	Asinh,
	Acosh,
	Atanh,
	Csch,
	Sech,
	Coth,
}

impl Function {
	pub fn to_string(&self) -> String {
		match self {
			Function::Abs => { String::from("abs") },
			Function::Floor => { String::from("floor") },
			Function::Ceil => { String::from("ceil") },
			Function::Round => { String::from("round") },
			Function::NaturalLog => { String::from("ln") },
			Function::TenLog => { String::from("log") },
			Function::Sin => { String::from("sin") },
			Function::Cos => { String::from("cos") },
			Function::Tan => { String::from("tan") },
			Function::Asin => { String::from("asin") },
			Function::Acos => { String::from("acos") },
			Function::Atan => { String::from("atan") },
			Function::Csc => { String::from("csc") },
			Function::Sec => { String::from("sec") },
			Function::Cot => { String::from("cot") },
			Function::Sinh => { String::from("sinh") },
			Function::Cosh => { String::from("cosh") },
			Function::Tanh => { String::from("tanh") },
			Function::Asinh => { String::from("asinh") },
			Function::Acosh => { String::from("acosh") },
			Function::Atanh => { String::from("atanh") },
			Function::Csch => { String::from("csch") },
			Function::Sech => { String::from("sech") },
			Function::Coth => { String::from("coth") },
		}
	}

	pub fn apply(&self, args: &VecDeque<Token>) -> Result<Token, ()> {
		if args.len() != 1 {panic!()};
		let a = args[0].as_number();
		let Token::Number(q) = a else {panic!()};

		match self {
			Function::Abs => { return Ok(Token::Number(q.abs())); },
			Function::Floor => { return Ok(Token::Number(q.floor())); },
			Function::Ceil => { return Ok(Token::Number(q.ceil())); },
			Function::Round => { return Ok(Token::Number(q.round())); },

			Function::NaturalLog => { return Ok(Token::Number(q.ln())); },
			Function::TenLog => { return Ok(Token::Number(q.log10())); },

			Function::Sin => { return Ok(Token::Number(q.sin())); },
			Function::Cos => { return Ok(Token::Number(q.cos())); },
			Function::Tan => { return Ok(Token::Number(q.tan())); },
			Function::Asin => { return Ok(Token::Number(q.asin())); },
			Function::Acos => { return Ok(Token::Number(q.acos())); },
			Function::Atan => { return Ok(Token::Number(q.atan())); },

			Function::Csc => {
				return Ok(
					Token::Operator(
						Operator::Flip,
						VecDeque::from(vec!(Token::Number(q.sin())))
					).eval()?
				);
			},
			Function::Sec => {
				return Ok(
					Token::Operator(
						Operator::Flip,
						VecDeque::from(vec!(Token::Number(q.cos())))
					).eval()?
				);
			},
			Function::Cot => {
				return Ok(
					Token::Operator(
						Operator::Flip,
						VecDeque::from(vec!(Token::Number(q.tan())))
					).eval()?
				);
			},


			Function::Sinh => { return Ok(Token::Number(q.sinh())); },
			Function::Cosh => { return Ok(Token::Number(q.cosh())); },
			Function::Tanh => { return Ok(Token::Number(q.tanh())); },
			Function::Asinh => { return Ok(Token::Number(q.asinh())); },
			Function::Acosh => { return Ok(Token::Number(q.acosh())); },
			Function::Atanh => { return Ok(Token::Number(q.atanh())); },

			Function::Csch => {
				return Ok(
					Token::Operator(
						Operator::Flip,
						VecDeque::from(vec!(Token::Number(q.sinh())))
					).eval()?
				);
			},
			Function::Sech => {
				return Ok(
					Token::Operator(
						Operator::Flip,
						VecDeque::from(vec!(Token::Number(q.cosh())))
					).eval()?
				);
			},
			Function::Coth => {
				return Ok(
					Token::Operator(
						Operator::Flip,
						VecDeque::from(vec!(Token::Number(q.tanh())))
					).eval()?
				);
			},

		}
	}
}


impl Operator {

	#[inline(always)]
	fn add_parens_to_arg(&self, arg: &Token) -> String {
		let mut astr: String = arg.print();
		if let Token::Operator(o,_) = arg {
			if o.as_int() < self.as_int() {
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
					self.add_parens_to_arg(&args[0]),
					self.add_parens_to_arg(&args[1])
				);
			},

			Operator::Factorial => {
				return format!("{}!", self.add_parens_to_arg(&args[0]));
			},



			Operator::Add => {
				let a = &args[0];
				let mut b = &args[1];
				let mut sub = false;

				if let Token::Operator(o,ar) = b {
					if let Operator::Negative = o {
						sub = true;
						b = &ar[0];
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


				let mut astr: String = a.print();
				if let Token::Operator(o,_) = a {
					if o.as_int() < self.as_int() {
						astr = format!("({})", astr);
					}
				}

				let mut bstr: String = b.print();
				if let Token::Operator(o,_) = b {
					if o.as_int() < self.as_int() {
						bstr = format!("({})", astr);
					}
				}

				if div {
					return format!("{} ÷ {}", astr, bstr);
				} else {
					return format!("{} × {}", astr, bstr);
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
				let b = Token::Operator(Operator::Negative, VecDeque::from(vec!(b)));

				Token::Operator(
					Operator::Add,
					VecDeque::from(vec!(a,b))
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
					VecDeque::from(vec!(a, Token::Number(Quantity::new_rational(1,2))))
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
					return Ok(Token::Number(Quantity::new_rational(1,1)/v));
				} else { panic!(); }
			},

			Operator::Add => {
				let mut sum = Quantity::new_rational(0,1);
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
				let mut prod = Quantity::new_rational(1,1);
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
						if vb <= Quantity::new_rational(1,1) { return Err(()); } 
						if va.fract() != Quantity::new_rational(0,1) { return Err(()); }
						if vb.fract() != Quantity::new_rational(0,1) { return Err(()); }

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
					if v >= Quantity::new_rational(100, 1) { return Err(()); }

					let mut prod = Quantity::new_rational(1, 1);
					let mut u = v.clone();
					while u > Quantity::new_rational(0, 1) {
						prod *= u.clone();
						u = u - Quantity::new_rational(1, 1);
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