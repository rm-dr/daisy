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
	#[inline(always)]

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
	ImplicitMultiply,
	Modulo, // Mod invoked with %
	Negative,
	Power,
	Sqrt,
	Factorial,

	// Not accessible from prompt
	Flip,
}

impl Operator {
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
			_ => None
		}
	}

	#[inline(always)]
	pub fn is_binary(&self) -> bool {
		match self {
			Operator::Negative
			| Operator::Factorial
			| Operator::Sqrt
			=> false,
			_ => true
		}
	}

	#[inline(always)]
	pub fn is_left_associative(&self) -> bool {
		match self {
			Operator::Negative
			| Operator::Sqrt
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

			Operator::ModuloLong
			=> { Token::Operator(Operator::Modulo, args) },

			Operator::Factorial
			| Operator::Negative
			| Operator::Flip
			| Operator::Add
			| Operator::Multiply
			| Operator::Modulo
			| Operator::Power
			=> { Token::Operator(self, args) },
		}
	}
}

impl Operator{
	pub fn apply(&self, args: &VecDeque<Token>) -> Result<Token, ()> {
		match self {
			Operator::ImplicitMultiply |
			Operator::Sqrt |
			Operator::ModuloLong |
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

			Operator::Modulo => {
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
		};
	}

}