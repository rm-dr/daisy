mod stage;

mod pretoken;
mod parsererror;
mod token;

use self::{
	pretoken::PreToken,
	parsererror::ParserError,
	parsererror::LineLocation
};

pub use self::{
	token::Token,
	token::Constant,
	token::Operator,
	token::Function,
};

use crate::context::Context;


pub fn parse(
	s: &String, context: &Context
) -> Result<Token, (LineLocation, ParserError)> {

	let tokens = stage::tokenize(s);
	let (_, tokens) = stage::find_subs(tokens);
	let g = stage::groupify(tokens)?;

	let g = stage::treeify(g, context)?;

	return Ok(g);
}

pub fn parse_no_context(s: &String) -> Result<Token, (LineLocation, ParserError)> {
	parse(s, &Context::new())
}


pub fn substitute(
	s: &String, // The string to substitute
	c: usize    // Location of the cursor right now
) -> (
	usize,  // Location of cursor in substituted string
	String  // String with substitutions
) {
	if s == "" { return (c, s.clone()) }
	let mut new_s = s.clone();

	let l = s.chars().count();
	let tokens = stage::tokenize(s);
	let (subs, _) = stage::find_subs(tokens);
	let mut new_c = l - c;

	for r in subs.iter() {
		// find_subs gives substitutions in reverse order.

		if { // Don't substitute if our cursor is inside the substitution
			c >= r.0.pos &&
			c < r.0.pos+r.0.len
		} { continue; }

		if c < r.0.pos {
			let ct = r.1.chars().count();
			if ct >= r.0.len {
				if new_c >= ct - r.0.len {
					new_c += ct - r.0.len
				}
			} else {
				new_c -= r.0.len - ct
			}
		}

		new_s.replace_range(
			r.0.pos..r.0.pos+r.0.len,
			&r.1[..]
		)
	}

	return (new_c, new_s);
}