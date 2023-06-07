pub mod tokens;
pub mod parser;
pub mod command;
pub mod quantity;

//use crate::tokens::Token;
//use crate::parser::ParserError;
//use crate::parser::LineLocation;

mod entry;
use crate::entry::main_e;

fn main() -> Result<(), std::io::Error> {
	return main_e();
}


#[cfg(test)]
mod tests;