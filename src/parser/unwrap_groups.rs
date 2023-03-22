use crate::parser::Token;
use crate::parser::ParserError;
use crate::parser::LineLocation;

pub fn unwrap_groups(g: &mut Token) -> Result<(), (LineLocation, ParserError)> {

	match g {
		// If g is a PreGroup, unwrap it
		Token::PreGroup(l, ref mut vec) => {
			if vec.len() != 1 {
				return Err((*l, ParserError::Syntax));
			}
			
			let mut i = vec.pop_front().unwrap();
			unwrap_groups(&mut i)?;
			*g = i;
		},

		// If g has sub-elements, recursive call
		Token::Multiply(ref mut vec) |
		Token::Divide(ref mut vec) |
		Token::Add(ref mut vec) |
		Token::Subtract(ref mut vec) |
		Token::Factorial(ref mut vec) |
		Token::Negative(ref mut vec) |
		Token::Power(ref mut vec) |
		Token::Modulo(ref mut vec) => {
			for i in vec.iter_mut() {
				unwrap_groups(i)?;
			}
		},

		// Otherwise, skip g.
		_ => {}
	};

	return Ok(());
}