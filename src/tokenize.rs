#[derive(Debug)]
pub enum Token {
	Negative,
	StartGroup,
	EndGroup,
	Number(String),
	Operator(String),
	Word(String),
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
pub fn tokenize(input: &String) -> Result<Vec<Token>, ()> {
	let mut v: Vec<Token> = Vec::new();
	let mut t: Option<Token> = None;
	for c in input.chars() {
		match c {
			// Minus sign can be both a Negative and an Operator.
			// Needs special treatment.
			'-' => {
				if t.is_some() { v.push(t.unwrap()); t = None; }
				match v.last() {
					// If previous token was any of the following,
					// this is the "minus" operator
					Some(Token::Number(_)) |
					Some(Token::EndGroup) |
					Some(Token::Word(_)) => {
						v.push(Token::Operator(String::from(c)));
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
						if t.is_some() { v.push(t.unwrap()); }
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
						if t.is_some() { v.push(t.unwrap()); }
						t = Some(Token::Word(String::from(c)));
					}
				};
			},


			// Operation
			// Always one character
			'+' | '*' | '/' | '^' => {
				// Finalize previous token
				if t.is_some() { v.push(t.unwrap()); t = None; }
				v.push(Token::Operator(String::from(c)));
			}
			
			// Groups
			// Always one character
			'(' => {
				if t.is_some() { v.push(t.unwrap()); t = None; }
				v.push(Token::StartGroup);
			},
			')' => {
				if t.is_some() { v.push(t.unwrap()); t = None; }
				v.push(Token::EndGroup);
			},

			// Space. Basic seperator.
			' ' => {
				if t.is_some() { v.push(t.unwrap()); t = None; }
			}

			// Invalid token
			_ => { return Err(()); }
		};
	}

	if t.is_some() { v.push(t.unwrap()); }
	return Ok(v);
}