pub mod tokens;
pub mod parser;
pub mod evaluate;
pub mod quantity;

//use crate::tokens::Token;
//use crate::parser::ParserError;
//use crate::parser::LineLocation;


/*
 Greeter ascii art:

  ######  @@@@@@
 #     ##@@     @
 ##     #@     @@
   @@@@@@@@@@@@@
 @@     @#     ##
 @     @@##     #
  @@@@@@  ######

   Daisy 0.0.0
*/

mod entry;
use crate::entry::main_e;

fn main() -> Result<(), std::io::Error> {
	return main_e();
}


#[cfg(test)]
mod tests;