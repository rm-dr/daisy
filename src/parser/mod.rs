mod stage;

mod token;
mod parsererror;
mod expression;
mod linelocation;

use self::{
	token::Token,
	parsererror::ParserError,
};

pub use self::{
	expression::Expression,
	expression::Constant,
	expression::Operator,
	expression::Function,
	linelocation::LineLocation,
};

use crate::context::Context;


pub fn parse(
	s: &String, context: &Context
) -> Result<Expression, (LineLocation, ParserError)> {

	let expressions = stage::tokenize(s);
	let (_, expressions) = stage::find_subs(expressions);
	let g = stage::groupify(expressions)?;
	let g = stage::treeify(g, context)?;

	return Ok(g);
}

pub fn parse_no_context(s: &String) -> Result<Expression, (LineLocation, ParserError)> {
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
	let expressions = stage::tokenize(s);
	let (mut subs, _) = stage::find_subs(expressions);
	let mut new_c = l - c;

	while subs.len() > 0 {
		let r = subs.pop_back().unwrap();
		// Apply substitutions in reverse order

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