mod stage;

mod token;
mod expression;
mod linelocation;

use self::token::Token;

pub use self::{
	expression::Expression,
	expression::Constant,
	expression::Operator,
	expression::Function,
	linelocation::LineLocation,
};

use crate::context::Context;
use crate::errors::DaisyError;

pub fn parse(
	context: &Context, s: &String
) -> Result<Expression, (LineLocation, DaisyError)> {

	let mut expressions = stage::tokenize(context, s);
	if context.config.enable_substituion {
		(_, expressions) = stage::find_subs(expressions);
	}
	let g = stage::groupify(context, expressions)?;
	let g = stage::treeify(context, g)?;

	return Ok(g);
}

pub fn parse_no_context(s: &String) -> Result<Expression, (LineLocation, DaisyError)> {
	parse(&Context::new(), s)
}



// Substitiution replaces certain string with pretty unicode characters.
// When it is enabled, ALL input strings are substituted. Variable and
// operator tokens use the replaced string value. Make sure both the
// original and the replaced strings are handled correctly by the parser.
pub fn substitute(context: &Context, s: &String) -> String {
	if !context.config.enable_substituion { return s.clone(); }
	let (_, s) = substitute_cursor(context, s, s.chars().count());
	return s;
}

pub fn substitute_cursor(
	context: &Context,
	s: &String, // The string to substitute
	c: usize    // Location of the cursor right now
) -> (
	usize,  // New cursor
	String  // String with substitutions
) {

	if !context.config.enable_substituion { return (c, s.clone()); }
	if s == "" { return (c, s.clone()) }
	
	
	let mut new_s = s.clone();

	let expressions = stage::tokenize(context, s);
	let (mut subs, _) = stage::find_subs(expressions);
	let mut new_c = c.clone();

	while subs.len() > 0 {
		// Apply substitutions in reverse order
		// r is the current substitution: (linelocation, string)
		let r = subs.pop_back().unwrap();

		if { // Don't substitute if our cursor is inside the substitution
			c >= r.0.pos &&
			c < r.0.pos+r.0.len
		} { continue; }

		// If this substitution is before our cursor,
		// we need to adjust our cursor's position.
		if c > r.0.pos {
			let c_o = r.0.len; // Old length 
			let c_n = r.1.chars().count(); // New length

			if c_n > c_o {
				// Move cursor right by difference
				new_c += c_n - c_o;

			} else if c_n < c_o {
				// Move cursor left by difference
				if new_c >= c_o - c_n {
					new_c -= c_o - c_n;
				} else { new_c = 0; }
			}
		}

		new_s.replace_range(
			r.0.pos..r.0.pos+r.0.len,
			&r.1[..]
		)
	}

	return (new_c, new_s);
}