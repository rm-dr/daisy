pub mod tokenize;
mod replace_pre;
mod fold_operators;
mod unwrap_groups;

use crate::parser::tokenize::Token;
use crate::parser::replace_pre::replace_pre;
use crate::parser::fold_operators::fold_operators;
use crate::parser::unwrap_groups::unwrap_groups;


pub fn parse(g: &mut Token) -> Result<(), ()> {
	replace_pre(g)?;
	fold_operators(g)?;
	unwrap_groups(g)?;

	return Ok(());
}