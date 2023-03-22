use crate::parser::Token;
use crate::parser::LineLocation;
use crate::parser::ParserError;


pub fn replace_pre(g: &mut Token) -> Result<(), (LineLocation, ParserError)> {

	match g {
		Token::PreGroup(_, ref mut vec) => {
			for i in vec.iter_mut() {
				replace_pre(i)?;
			}
		},
		Token::PreNumber(l, s) => {
			let n = match s.parse() {
				Ok(n) => n,
				Err(_) => return Err((*l, ParserError::BadNumber))
			};
			*g = Token::Number(n);
		}
		Token::PreWord(l, ref s) => {
			if s == "mod" {
				*g = Token::PreOperator(*l, String::from("mod"));
			} else {
				return Err((*l, ParserError::Syntax));
			}
		},
		Token::PreOperator(_, _) => {},
		_ => { panic!(); }
	};

	return Ok(());
}