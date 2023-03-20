pub mod tokenize;
mod replace_words;
mod fold_operators;

use crate::parser::tokenize::Token;
use crate::parser::replace_words::replace_words;
use crate::parser::fold_operators::fold_operators;

pub fn parse(g: &mut Token) -> Result<(), ()> {
	replace_words(g)?;
	fold_operators(g)?;

	return Ok(());
}