pub mod parser;
pub mod command;
pub mod quantity;
pub mod evaluate;


mod entry;
use crate::entry::main_e;

fn main() -> Result<(), std::io::Error> {
	return main_e();
}


#[cfg(test)]
mod tests;