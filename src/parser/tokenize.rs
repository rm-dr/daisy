use std::collections::VecDeque;


#[derive(Debug)]
#[derive(Clone)]
pub enum Token {

	// Only used after tokenizing
	PreGroup(VecDeque<Token>),
	PreOperator(String),
	PreNumber(String),
	PreWord(String),

	// Only used in tree
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
	g.push(Token::PreGroup(VecDeque::with_capacity(8)));


	for c in input.chars() {

		// The grouping level we're on now
		let g_now: &mut VecDeque<Token> = match g.last_mut().unwrap() {
			Token::PreGroup(ref mut x) => x,
			_ => panic!()
		};

		match c {
			'!' => {
				if t.is_some() { g_now.push_back(t.unwrap()); t = None; }
				g_now.push_back(Token::PreOperator(String::from("!")));
			},

			// Minus sign can be both a Negative and an Operator.
			// Needs special treatment.
			'-' => {
				if t.is_some() { g_now.push_back(t.unwrap()); t = None; }
				match g_now.back() {
					// If previous token was any of the following,
					// this is the "minus" operator
					Some(Token::PreNumber(_)) |
					Some(Token::PreGroup(_)) |
					Some(Token::PreWord(_)) => {
						g_now.push_back(Token::PreOperator(String::from(c)));
					},

					// Otherwise, this is a negative sign.
					_ => { g_now.push_back(Token::PreOperator(String::from("neg"))); }
				};
			},

			// Number.
			// Commas act just like dots.
			',' | '.' | '0'..='9' => {
				match &mut t {
					// If we're already building a number,
					// append.
					Some(Token::PreNumber(val)) => {
						val.push(if c == ',' {'.'} else {c});
					},

					// If we're not building a number, finalize
					// previous token and start one.
					_ => {
						if t.is_some() { g_now.push_back(t.unwrap()); }
						t = Some(Token::PreNumber(String::from(c)));
					}
				};
			},

			// Word
			'A'..='Z' |
			'a'..='z' => {
				match &mut t {
					// If we're already building a number,
					// append.
					Some(Token::PreWord(val)) => {
						val.push(c);
					},

					// If we're not building a number, finalize
					// previous token and start one.
					_ => {
						if t.is_some() { g_now.push_back(t.unwrap()); }
						t = Some(Token::PreWord(String::from(c)));
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
				if t.is_some() { g_now.push_back(t.unwrap()); t = None; }
				g_now.push_back(Token::PreOperator(String::from(c)));
			}
			
			// Groups
			// Always one character
			'(' => {
				if t.is_some() { g_now.push_back(t.unwrap()); t = None; }
				g.push(Token::PreGroup(VecDeque::with_capacity(8)));
			},
			')' => {
				if t.is_some() { g_now.push_back(t.unwrap()); t = None; }
				let new_group: Token = g.pop().unwrap();

				let g_now: &mut VecDeque<Token> = match g.last_mut().unwrap() {
					Token::PreGroup(ref mut x) => x,
					_ => panic!()
				};
		
				g_now.push_back(new_group);
			},

			// Space. Basic seperator.
			' ' => {
				if t.is_some() { g_now.push_back(t.unwrap()); t = None; }
			}

			// Invalid token
			_ => { return Err(()); }
		};
	}

	
	let g_now: &mut VecDeque<Token> = match g.last_mut().unwrap() {
		Token::PreGroup(ref mut x) => x,
		_ => panic!()
	};
	if t.is_some() { g_now.push_back(t.unwrap()); }

	return Ok(g.pop().unwrap());
}