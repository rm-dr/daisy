/// Specifies the location of a token in an input string.
/// Used to locate ParserErrors.
#[derive(Debug)]
#[derive(Copy, Clone)]
pub struct LineLocation {
	pub pos: usize,
	pub len: usize
}

/// Types of parser errors.
/// If we cannot parse a string, one of these is returned.
#[derive(Debug)]
pub enum ParserError {
	//MissingCloseParen,
	ExtraCloseParen,
	EmptyGroup,
	Syntax,
	Undefined(String),
	BadNumber
}

impl ToString for ParserError {
	fn to_string(&self) -> String {
		match self {
			//ParserError::MissingCloseParen => {
			//	String::from("This group is never closed")
			//},
			ParserError::ExtraCloseParen => {
				String::from("Extra close parenthesis")
			},
			ParserError::EmptyGroup => {
				String::from("Groups can't be empty")
			},
			ParserError::Syntax => {
				String::from("Syntax")
			},
			ParserError::Undefined(s) => {
				format!("\"{s}\" isn't defined")
			},
			ParserError::BadNumber => {
				String::from("Invalid number")
			}
		}
	}
}
