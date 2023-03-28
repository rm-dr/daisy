use std::collections::VecDeque;


/// Specifies the location of a token in an input string.
/// Used to locate ParserErrors.
#[derive(Debug)]
#[derive(Copy, Clone)]
pub struct LineLocation {
	pub pos: usize,
	pub len: usize
}

/// Tokens represent logical objects in an expession.
/// 
/// Tokens starting with `Pre*` are intermediate tokens, and
/// will never show up in a fully-parsed expression tree.
#[derive(Debug)]
pub enum Token {
	Number(LineLocation, f64),
	Constant(LineLocation, f64, String),

	Operator(
		LineLocation,
		Operator,
		VecDeque<Token>
	),
}

impl Token {
	#[inline(always)]
	pub fn get_args(&mut self) -> Option<&mut VecDeque<Token>> {
		match self {
			Token::Operator(_, _, ref mut a) => Some(a),
			_ => None
		}
	}

	#[inline(always)]
	pub fn get_line_location(&self) -> &LineLocation {
		match self {
			Token::Number(l, _)
			| Token::Operator(l, _, _)
			| Token::Constant(l, _, _)
			=> l,
		}
	}

	#[inline(always)]
	pub fn eval(&self) -> Token {
		match self {
			Token::Number(l,v) => { Token::Number(*l, *v) },
			Token::Constant(l,v,_) => { Token::Number(*l, *v) },
			Token::Operator(_,o,v) => { o.apply(v) }
		}
	}

	// Temporary solution
	#[inline(always)]
	pub fn as_number(&self) -> Token {
		match self {
			Token::Number(l,v) => { Token::Number(*l, *v) },
			Token::Constant(l,v,_) => { Token::Number(*l, *v) },
			_ => panic!()
		}
	}

}

/// Operator types, in order of increasing priority.
#[derive(Debug)]
#[derive(Copy, Clone)]
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
			_ => None
		}
	}

	#[inline(always)]
	pub fn is_binary(&self) -> bool {
		match self {
			Operator::Negative
			| Operator::Factorial
			=> false,
			_ => true
		}
	}

	#[inline(always)]
	pub fn is_left_associative(&self) -> bool {
		match self {
			Operator::Negative
			=> false,
			_ => true
		}
	}


	#[inline(always)]
	pub fn into_token(self, l: LineLocation, mut args: VecDeque<Token>) -> Token {
		match self {
			Operator::Subtract => {
				if args.len() != 2 { panic!() }
				let a = args.pop_front().unwrap();
				let b = args.pop_front().unwrap();
				let b = Token::Operator(l, Operator::Negative, VecDeque::from(vec!(b)));

				Token::Operator(
					l, Operator::Add,
					VecDeque::from(vec!(a,b))
				)
			},

			Operator::Divide => {
				if args.len() != 2 { panic!() }
				let a = args.pop_front().unwrap();
				let b = args.pop_front().unwrap();
				let b = Token::Operator(l, Operator::Flip, VecDeque::from(vec!(b)));

				Token::Operator(
					l, Operator::Multiply,
					VecDeque::from(vec!(a,b))
				)
			},

			Operator::ImplicitMultiply
			=> { Token::Operator(l, Operator::Multiply, args) },

			Operator::ModuloLong
			=> { Token::Operator(l, Operator::Modulo, args) },

			Operator::Factorial
			| Operator::Negative
			| Operator::Flip
			| Operator::Add
			| Operator::Multiply
			| Operator::Modulo
			| Operator::Power
			=> { Token::Operator(l, self, args) },
		}
	}
}

impl Operator{
	pub fn apply(&self, args: &VecDeque<Token>) -> Token {
		match self {
			Operator::ImplicitMultiply |
			Operator::ModuloLong |
			Operator::Divide |
			Operator::Subtract => { panic!() }

			Operator::Negative => {
				if args.len() != 1 {panic!()};
				let args = args[0].as_number();

				if let Token::Number(l, v) = args {
					Token::Number(l, -v)
				} else { panic!(); }
			},

			Operator::Flip => {
				if args.len() != 1 {panic!()};
				let args = args[0].as_number();

				if let Token::Number(l, v) = args {
					Token::Number(l, 1f64/v)
				} else { panic!(); }
			},

			Operator::Add => {
				let mut sum: f64 = 0f64;
				let mut new_pos: usize = 0;
				let mut new_len: usize = 0;
				for i in args.iter() {
					let j = i.as_number();
					if let Token::Number(l, v) = j {
						if new_pos == 0 {new_pos = l.pos};
						new_len = new_len + l.len;
						sum += v;
					} else {
						panic!();
					}
				}

				Token::Number(
					LineLocation { pos: new_pos, len: new_len },
					sum
				)
			},
			
			Operator::Multiply => {
				let mut prod: f64 = 1f64;
				let mut new_pos: usize = 0;
				let mut new_len: usize = 0;
				for i in args.iter() {
					let j = i.as_number();
					if let Token::Number(l, v) = j {
						if new_pos == 0 {new_pos = l.pos};
						new_len = new_len + l.len;
						prod *= v;
					} else {
						panic!();
					}
				}

				Token::Number(
					LineLocation { pos: new_pos, len: new_len },
					prod
				)
			},

			Operator::Modulo => {
				if args.len() != 2 {panic!()};
				let a = args[0].as_number();
				let b = args[1].as_number();

				if let Token::Number(la, va) = a {
					if let Token::Number(lb, vb) = b {
						Token::Number(
							LineLocation { pos: la.pos, len: lb.pos - la.pos + lb.len },
							va%vb
						)
					} else { panic!(); }
				} else { panic!(); }
			},

			Operator::Power => {
				if args.len() != 2 {panic!()};
				let a = args[0].as_number();
				let b = args[1].as_number();

				if let Token::Number(la, va) = a {
					if let Token::Number(lb, vb) = b {
						Token::Number(
							LineLocation { pos: la.pos, len: lb.pos - la.pos + lb.len },
							va.powf(vb)
						)
					} else { panic!(); }
				} else { panic!(); }
			},

			Operator::Factorial => { todo!() },
		}
	}

}