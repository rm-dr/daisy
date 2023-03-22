use crate::parser::tokenize::Token;

pub fn replace_pre(g: &mut Token) -> Result<(), ()> {

	match g {
		Token::PreGroup(_, ref mut vec) => {
			for i in vec.iter_mut() {
				replace_pre(i)?;
			}
		},
		Token::PreNumber(_, s) => {
			let n = match s.parse() {
				Ok(n) => n,
				Err(_) => panic!()
			};
			*g = Token::Number(n);
		}
		Token::PreWord(l, ref s) => {
			if s == "mod" {
				*g = Token::PreOperator(*l, String::from("mod"));
			} else {
				return Err(());
				//new.push_back(t);
			}
		},
		Token::PreOperator(_, _) => {},
		_ => { panic!(); }
	};

	return Ok(());
}