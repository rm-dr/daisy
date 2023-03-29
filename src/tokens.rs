use std::collections::VecDeque;

/// Tokens represent logical objects in an expession.
/// 
/// Tokens starting with `Pre*` are intermediate tokens, and
/// will never show up in a fully-parsed expression tree.
#[derive(Debug)]
pub enum Token {
	Number(f64),
	Constant(f64, String),

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
			Token::Number(v) => { Token::Number(*v) },
			Token::Constant(v,_) => { Token::Number(*v) },
			Token::Operator(o,v) => { o.apply(v)? }
		})
	}

	// Temporary solution
	#[inline(always)]
	pub fn as_number(&self) -> Token {
		match self {
			Token::Number(v) => { Token::Number(*v) },
			Token::Constant(v,_) => { Token::Number(*v) },
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

	Function(String),

	// Not accessible from prompt
	Flip,
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
			Operator::Flip |
			Operator::Subtract => { panic!() }

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
				return format!("{}({})", s, args[0].print());
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
			"sin"    => {Some( Operator::Function(String::from("sin")) )}
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
					VecDeque::from(vec!(a, Token::Number(0.5)))
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
					if v == 0f64 { return Err(()); }
					return Ok(Token::Number(1f64/v));
				} else { panic!(); }
			},

			Operator::Add => {
				let mut sum: f64 = 0f64;
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
				let mut prod: f64 = 1f64;
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
						if vb <= 1f64 { return Err(()); } 
						if va.fract() != 0f64 { return Err(()); }
						if vb.fract() != 0f64 { return Err(()); }

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
						let p = va.powf(vb);
						if p.is_nan() {return Err(());}
						return Ok(Token::Number(p));
					} else { panic!(); }
				} else { panic!(); }
			},

			Operator::Factorial => {
				if args.len() != 1 {panic!()};
				let args = args[0].as_number();

				if let Token::Number(v) = args {
					if v.fract() != 0f64 { return Err(()); }
					if v >= 100f64 { return Err(()); }

					let mut prod = 1f64;
					let mut u = v;
					while u > 0f64 {
						prod *= u;
						u -= 1f64;
					}

					return Ok(Token::Number(prod));
				} else { panic!(); }
			},

			Operator::Function(s) => {
				match &s[..] {
					"sin" => {
						if args.len() != 1 {panic!()};
						let a = args[0].as_number();
						let Token::Number(v) = a else {panic!()};
						return Ok(Token::Number(v.sin()));
					}

					_ => panic!()
				}
			}
		};
	}

}