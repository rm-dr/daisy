use std::collections::VecDeque;

#[derive(Debug)]
#[derive(Copy, Clone)]
pub struct LineLocation {
	pos: usize,
	len: usize
}

#[derive(Debug)]
pub enum Token {

	// Only used after tokenizing
	PreGroup(LineLocation, VecDeque<Token>),
	PreOperator(LineLocation, String),
	PreNumber(LineLocation, String),
	PreWord(LineLocation, String),

	// All PreGroups should vanish after operator folding
	// All PreOperators should become Operators
	// All PreNumbers should become Numbers
	// All PreWords should become TODO.

	// Only used in tree

	Number(f64),

	// Functions

	// Operators	
	Multiply(VecDeque<Token>),
	Divide(VecDeque<Token>),
	Add(VecDeque<Token>),
	Subtract(VecDeque<Token>),
	Factorial(VecDeque<Token>),
	Negative(VecDeque<Token>),
	Power(VecDeque<Token>),
	Modulo(VecDeque<Token>),

	//Function(String, VecDeque<Token>),
}

#[inline(always)]
fn update_line_location(mut t: Token, stop_i: usize) -> Token {
	match t {
		Token::PreGroup(ref mut l, _) |
		Token::PreOperator(ref mut l, _) |
		Token::PreNumber(ref mut l, _) |
		Token::PreWord(ref mut l, _) => {
			let LineLocation{pos, .. } = l;
			*l = LineLocation{
				pos: *pos,
				len: stop_i - *pos,
			};
		},
		_ => panic!()
	};

	return t;
}


/// Turn a string into a set of tokens.
/// Does not check syntax. Fails if `input` contains an invalid character.
//
// # Arguments:
// `input`: A string like `(-3*2.2)/3`
//
// # Returns:
// * `Ok(Vec<token>)` if we were successful.
// * `Err(())` if we couldn't tokenize this string.
pub fn tokenize(input: &String) -> Result<Token, ()> {
	let mut t: Option<Token> = None; // The current token we're reading
	let mut g: Vec<Token> = Vec::with_capacity(8); // Vector of "grouping levels"
	g.push(Token::PreGroup(LineLocation{pos: 0, len: 0}, VecDeque::with_capacity(8)));


	for (i, c) in input.chars().enumerate() {

		// The grouping level we're on now
		let g_now: &mut VecDeque<Token> = match g.last_mut().unwrap() {
			Token::PreGroup(_, ref mut x) => x,
			_ => panic!()
		};

		match c {
			'!' => {
				if t.is_some() { g_now.push_back(update_line_location(t.unwrap(), i)); t = None; }
				g_now.push_back(
					Token::PreOperator(
						LineLocation{pos: i, len: 1},
						String::from("!")
					)
				);
			},

			// Minus sign can be both a Negative and an Operator.
			// Needs special treatment.
			'-' => {
				if t.is_some() { g_now.push_back(update_line_location(t.unwrap(), i)); t = None; }
				match g_now.back() {
					// If previous token was any of the following,
					// this is the "minus" operator
					Some(Token::PreNumber(_, _)) |
					Some(Token::PreGroup(_, _)) |
					Some(Token::PreWord(_, _)) => {
						g_now.push_back(
							Token::PreOperator(
								LineLocation{pos: i, len: 1},
								String::from(c)
							)
						);
					},

					// Otherwise, this is a negative sign.
					_ => {
						g_now.push_back(
							Token::PreOperator(
								LineLocation{pos: i, len: 1},
								String::from("neg")
							)
						);
					}
				};
			},

			// Number.
			// Commas act just like dots.
			',' | '.' | '0'..='9' => {
				match &mut t {
					// If we're already building a number,
					// append.
					Some(Token::PreNumber(_, val)) => {
						val.push(if c == ',' {'.'} else {c});
					},

					// If we're not building a number, finalize
					// previous token and start one.
					_ => {
						if t.is_some() { g_now.push_back(update_line_location(t.unwrap(), i)); }
						t = Some(Token::PreNumber(LineLocation{pos: i, len: 0}, String::from(c)));
					}
				};
			},

			// Word
			'A'..='Z' |
			'a'..='z' => {
				match &mut t {
					// If we're already building a number,
					// append.
					Some(Token::PreWord(_, val)) => {
						val.push(c);
					},

					// If we're not building a number, finalize
					// previous token and start one.
					_ => {
						if t.is_some() { g_now.push_back(update_line_location(t.unwrap(), i)); }
						t = Some(Token::PreWord(LineLocation{pos: i, len: 0}, String::from(c)));
					}
				};
			},


			// Operation
			// Always one character
			'+' |
			'*' |
			'/' |
			'^' |
			'%' => {
				// Finalize previous token
				if t.is_some() { g_now.push_back(update_line_location(t.unwrap(), i)); t = None; }
				g_now.push_back(Token::PreOperator(LineLocation{pos: i, len: 1}, String::from(c)));
			}
			
			// Groups
			// Always one character
			'(' => {
				if t.is_some() { g_now.push_back(update_line_location(t.unwrap(), i)); t = None; }
				g.push(Token::PreGroup(LineLocation{pos: i, len: 0}, VecDeque::with_capacity(8)));
			},
			')' => {
				if t.is_some() { g_now.push_back(update_line_location(t.unwrap(), i)); t = None; }
				let new_group: Token = g.pop().unwrap();

				let g_now: &mut VecDeque<Token> = match g.last_mut().unwrap() {
					Token::PreGroup(_, ref mut x) => x,
					_ => panic!()
				};
		
				g_now.push_back(update_line_location(new_group, i));
			},

			// Space. Basic seperator.
			' ' => {
				if t.is_some() { g_now.push_back(update_line_location(t.unwrap(), i)); t = None; }
			}

			// Invalid token
			_ => { return Err(()); }
		};
	}

	
	let g_now: &mut VecDeque<Token> = match g.last_mut().unwrap() {
		Token::PreGroup(_, ref mut x) => x,
		_ => panic!()
	};
	if t.is_some() { g_now.push_back(update_line_location(t.unwrap(), input.len())); }

	return Ok(g.pop().unwrap());
}