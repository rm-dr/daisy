#[derive(Debug)]
#[derive(Clone)]
pub enum Token {
	Negative,
	Number(String),
	Operator(String),
	Word(String),
	Group(Vec<Token>),
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
	let mut t: Option<Token> = None;
	let mut g: Vec<Token> = Vec::with_capacity(8);
	g.push(Token::Group(Vec::with_capacity(8)));


	for c in input.chars() {
		let v_now: &mut Vec<Token> = match g.last_mut().unwrap() {
			Token::Group(ref mut x) => x,
			_ => panic!()
		};

		match c {
			// Minus sign can be both a Negative and an Operator.
			// Needs special treatment.
			'-' => {
				if t.is_some() { v_now.push(t.unwrap()); t = None; }
				match v_now.last() {
					// If previous token was any of the following,
					// this is the "minus" operator
					Some(Token::Number(_)) |
					Some(Token::Group(_)) |
					Some(Token::Word(_)) => {
						v_now.push(Token::Operator(String::from(c)));
					},

					// Otherwise, this is a negative sign.
					_ => { t = Some(Token::Negative); }
				};
			},

			// Number.
			// Commas act just like dots.
			',' | '.' | '0'..='9' => {
				match &mut t {
					// If we're already building a number,
					// append.
					Some(Token::Number(val)) => {
						val.push(if c == ',' {'.'} else {c});
					},

					// If we're not building a number, finalize
					// previous token and start one.
					_ => {
						if t.is_some() { v_now.push(t.unwrap()); }
						t = Some(Token::Number(String::from(c)));
					}
				};
			},

			// Word
			'A'..='Z' |
			'a'..='z' => {
				match &mut t {
					// If we're already building a number,
					// append.
					Some(Token::Word(val)) => {
						val.push(c);
					},

					// If we're not building a number, finalize
					// previous token and start one.
					_ => {
						if t.is_some() { v_now.push(t.unwrap()); }
						t = Some(Token::Word(String::from(c)));
					}
				};
			},


			// Operation
			// Always one character
			'+' | '*' | '/' | '^' => {
				// Finalize previous token
				if t.is_some() { v_now.push(t.unwrap()); t = None; }
				v_now.push(Token::Operator(String::from(c)));
			}
			
			// Groups
			// Always one character
			'(' => {
				if t.is_some() { v_now.push(t.unwrap()); t = None; }
				g.push(Token::Group(Vec::with_capacity(8)));
			},
			')' => {
				if t.is_some() { v_now.push(t.unwrap()); t = None; }
				let new_group: Token = g.pop().unwrap();

				let v_now: &mut Vec<Token> = match g.last_mut().unwrap() {
					Token::Group(ref mut x) => x,
					_ => panic!()
				};
		
				v_now.push(new_group);
			},

			// Space. Basic seperator.
			' ' => {
				if t.is_some() { v_now.push(t.unwrap()); t = None; }
			}

			// Invalid token
			_ => { return Err(()); }
		};
	}

	
	let v_now: &mut Vec<Token> = match g.last_mut().unwrap() {
		Token::Group(ref mut x) => x,
		_ => panic!()
	};
	if t.is_some() { v_now.push(t.unwrap()); }

	return Ok(Token::Group(v_now.to_vec()));
}