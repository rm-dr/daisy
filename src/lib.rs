pub mod parser;
pub mod command;
pub mod quantity;

use crate::parser::substitute;
use crate::parser::LineLocation;


mod context;
mod formattedtext;
mod errors;
mod evaluate;
mod promptbuffer;

pub use crate::formattedtext::FormattedText;
pub use crate::context::Context;
pub use crate::errors::DaisyError;
pub use crate::evaluate::evaluate;
pub use crate::promptbuffer::PromptBuffer;



cfg_if::cfg_if! {
	if #[cfg(target_arch = "wasm32")] {
		use wasm_bindgen::prelude::*;

		#[derive(Debug)]
		pub struct State {
			pub context: Context,
			pub promptbuffer: PromptBuffer
		}

		#[wasm_bindgen]
		pub extern fn daisy_init() -> *mut State {
			Box::into_raw(Box::new(State {
				context: Context::new(),
				promptbuffer: PromptBuffer::new(64)
			}))
		}

		#[wasm_bindgen]
		pub extern fn daisy_free(state: *mut State) {
			unsafe { drop(Box::from_raw(state)) };
		}

		#[wasm_bindgen]
		pub fn daisy_prompt(state: *mut State) -> String {
			let t = unsafe { (*state).promptbuffer.write_prompt(&mut (*state).context) };
			return t.write();
		}


		#[wasm_bindgen]
		pub fn daisy_char(state: *mut State, s: String) -> String {
			let mut out = FormattedText::new("".to_string());

			match &s[..] {
				"\r" => {
					// Print again without cursor, in case we pressed enter
					// while inside a substitution
					let t = unsafe { (*state).promptbuffer.write_prompt_nocursor(&mut (*state).context) };
					out += t;


					let in_str = unsafe { (*state).promptbuffer.enter() };
					out += FormattedText::new("\n".to_string());
					if in_str == "" {
						return format!("\r\n{}", daisy_prompt(state));
					}


					let r = crate::do_string( unsafe { &mut (*state).context }, &in_str);

					match r {
						Ok(t) | Err(t) => {
							out += t;
						}
					}
				},

				"\x7F" => { unsafe { (*state).promptbuffer.backspace(); } },
				"\x1B[3~" => { unsafe { (*state).promptbuffer.delete(); } },
				"\x1B[D" => { unsafe { (*state).promptbuffer.cursor_left(); } },
				"\x1B[C" => { unsafe { (*state).promptbuffer.cursor_right(); } },
				"\x1B[A" => { unsafe { (*state).promptbuffer.hist_up(); } },
				"\x1B[B" => { unsafe { (*state).promptbuffer.hist_down(); } },

				//'\x04' | '\x03'
				//=> { break 'outer; },

				// Only process sane characters

				_ => {
					let c = s.chars().next().unwrap();
					match c {
						'a'..='z' | 'A'..='Z' | '0'..='9'
						|'!'|'@'|'#'|'$'|'%'|'^'|'&'|'*'|'('|')'
						|'?'|'~'|','|'.'|'['|']'
						|'<'|'>'|'/'|'_'|'-'|':'|'|'|'='|'+'|';'
						=> { unsafe { (*state).promptbuffer.add_char(c); } },

						_ => {}
					}
				},
			};
			
			let t = unsafe { (*state).promptbuffer.write_prompt(&mut (*state).context) };
			return (out + t).write();
		}
	}
}

#[inline(always)]
pub fn do_string(
	context: &mut Context,
	s: &String
) -> Result<FormattedText, FormattedText> {

	let r: (LineLocation, DaisyError);
	if command::is_command(s) {
		return Ok(command::do_command(context, s));
	} else if s.contains("=") {
		let x = do_assignment(context, s);
		match x {
			Ok(t) => { return Ok(t) },
			Err(t) => { r = t }
		};
	} else {
		let x = do_expression(context, s);
		match x {
			Ok((t, e)) => { context.push_hist(e); return Ok(t) },
			Err(t) => { r = t }
		};
	}

	let (l, e) = r;
	let mut t = FormattedText::new("".to_string());
	if l.zero() {
		t.push(&format!(
			"\n  {}\n\n",
			e.text().to_string(),
		));
	} else {
		t.push(&format!(
			concat!(
				"{}[e]{}[n]\n",
				"  {}\n\n"
			),
			" ".repeat(l.pos + 4),
			"^".repeat(l.len),
			e.text().to_string(),
		));
	}

	return Err(t);
}

// Handle a simple evaluation string.
// Returns a FormattedText with output that should be printed.
#[inline(always)]
fn do_expression(
	context: &mut Context,
	s: &String
) -> Result<(FormattedText, parser::Expression), (LineLocation, DaisyError)> {

	let mut output = FormattedText::new("".to_string());

	let g = parser::parse(context, &s)?;
	let g_evaluated = evaluate::evaluate(context, &g)?;

	// Display parsed string
	output.push(&format!(
		" [s]=>[n] {}\n\n",
		g.display(context)
	));

	// Display result
	output.push(&format!(
		"  [r]=[n] {}\n\n",
		g_evaluated.display_outer(context),
	));

	return Ok((output, g_evaluated));
}


// Handle a variable or function definition string.
// Returns a FormattedText with output that should be printed.
#[inline(always)]
fn do_assignment(
	context: &mut Context,
	s: &String
) -> Result<FormattedText, (LineLocation, DaisyError)> {

	let mut output = FormattedText::new("".to_string());

	let parts = s.split("=").collect::<Vec<&str>>();
	if parts.len() != 2 {
		return Err((
			LineLocation::new_zero(),
			DaisyError::Syntax
		));
	}

	// Index of first non-whitespace character in left
	// (relative to whole prompt)
	let starting_left = parts[0]
		.char_indices()
		.find(|(_, ch)| !(ch.is_whitespace() && *ch != '\n'))
		.map(|(i, _)| i)
		.unwrap_or_else(|| parts[0].len());

	// Index of first non-whitespace character in right
	// (relative to whole prompt)
	// +1 accounts for equals sign
	let starting_right = parts[0].chars().count() + 1 +
		parts[1]
			.char_indices()
			.find(|(_, ch)| !(ch.is_whitespace() && *ch != '\n'))
			.map(|(i, _)| i)
			.unwrap_or_else(|| parts[0].len());


	let left = substitute(context, &parts[0].trim().to_string());
	let right = substitute(context, &parts[1].trim().to_string());
	let is_function = left.contains("(");

	// The order of methods below is a bit odd.
	// This is intentional, since we want to check a definition's
	// variable name before even attempting to parse its content.
	if is_function {
		let mut mode = 0;
		let mut name = String::new();
		let mut args = String::new();
		for c in left.chars() {
			match mode {

				// Mode 0: reading function name
				0 => {
					if c == '(' {
						mode = 1; continue;
					} else { name.push(c); }
				},

				// Mode 1: reading arguments
				1 => {
					if c == ')' {
						mode = 2; continue;
					} else { args.push(c); }
				},

				// Mode 2: we should be done by now.
				// That close paren should've been the last character.
				2 => {
					return Err((
						LineLocation{ pos: starting_left, len: left.chars().count() },
						DaisyError::Syntax
					));
				},

				_ => unreachable!()
			}
		}


		let args = args
			.split(",").collect::<Vec<&str>>()
			.iter().map(|x| x.trim().to_string()).collect::<Vec<String>>();

		if name.len() == 0 {
			return Err((
				LineLocation{ pos: starting_left, len: left.chars().count() },
				DaisyError::Syntax
			));
		};

		if !context.valid_function(&name) {
			return Err((
				LineLocation{ pos: starting_left, len: left.chars().count() },
				DaisyError::BadFunction
			));
		};

		if args.iter().find(|x| &x[..] == "").is_some() {
			return Err((
				LineLocation{ pos: starting_left, len: left.chars().count() },
				DaisyError::Syntax
			));
		};

		for a in &args {
			if !context.valid_varible(a) {
				return Err((
					LineLocation{ pos: starting_left, len: left.chars().count() },
					DaisyError::BadVariable
				));
			}
		}

		// Parse right hand side
		let g = parser::parse(context, &right);
		let Ok(g) = g else {
			let Err((l, e)) = g else { unreachable!() };
			return Err((
				LineLocation{ pos: l.pos + starting_right, len: l.len},
				e
			));
		};

		// Display parsed string
		output.push(&format!(
			" [s]=>[n] {left} = {}\n\n",
			g.display(context)
		));

		// Evaluate expression with shadow variables
		for a in &args { context.add_shadow(a.to_string(), None);}
		let g_evaluated = evaluate::evaluate(context, &g);
		context.clear_shadow();
		let Ok(_g_evaluated) = g_evaluated else {
			let Err((l, e)) = g_evaluated else { unreachable!() };
			return Err((
				LineLocation{ pos: l.pos + starting_right, len: l.len},
				e
			));
		};

		// We could push g_evaluated instead, but an un-evaluated string
		// makes the 'vars' command prettier.
		//
		// We still need to evaluate g above, though, to make sure it works.
		context.push_function(name, args, g).unwrap();
	} else {

		if !context.valid_varible(&left) {
			return Err((
				LineLocation{ pos: starting_left, len: left.chars().count() },
				DaisyError::BadVariable
			));
		}

		// Parse right hand side
		let g = parser::parse(context, &right);
		let Ok(g) = g else {
			let Err((l, e)) = g else { unreachable!() };
			return Err((
				LineLocation{ pos: l.pos + starting_right, len: l.len},
				e
			));
		};

		// Display parsed string
		output.push(&format!(
			" [t]=>[n] {left} = {}\n\n",
			g.display(context)
		));

		// Evaluate expression
		let g_evaluated = evaluate::evaluate(context, &g);
		let Ok(g_evaluated) = g_evaluated else {
			let Err((l, e)) = g_evaluated else { unreachable!() };
			return Err((
				LineLocation{ pos: l.pos + starting_right, len: l.len},
				e
			));
		};

		context.push_variable(left.to_string(), g_evaluated).unwrap();
	}

	return Ok(output);

}


