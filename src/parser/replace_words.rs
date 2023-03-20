use std::collections::VecDeque;
use crate::parser::tokenize::Token;

pub fn replace_words(g: &mut Token) -> Result<(), ()> {
	let g_inner: &mut VecDeque<Token> = match g {
		Token::PreGroup(ref mut x) => x,
		_ => panic!()
	};

	let mut new: VecDeque<Token> = VecDeque::with_capacity(8);

	while g_inner.len() > 0 {
		let mut t: Token = match g_inner.pop_front() {
			Some(o) => o,
			None => break
		};

		match t {
			Token::PreGroup(_) => {
				replace_words(&mut t)?;
				new.push_back(t);
			},
			Token::PreWord(ref s) => {
				if s == "to" {
					new.push_back(Token::PreOperator(String::from("to")));
				} else if s == "mod" {
					new.push_back(Token::PreOperator(String::from("mod")));
				} else {
					new.push_back(t);
				}
			},
			_ => { new.push_back(t); }
		};
	}

	*g_inner = new;

	return Ok(());
}