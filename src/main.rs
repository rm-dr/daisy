use std::io::stdout;
use std::io::stdin;
use std::env;

use termion::{
	event::Key,
	input::TermRead,
	raw::IntoRawMode,
	color::DetectColors
};

use daisycalc::PromptBuffer;
use daisycalc::command;
use daisycalc::Context;
use daisycalc::FormattedText;
use daisycalc::do_string;

#[cfg(test)]
mod tests;



#[inline(always)]
pub fn main() -> Result<(), std::io::Error> {
	let mut stdout = stdout().into_raw_mode().unwrap();
	let mut pb: PromptBuffer = PromptBuffer::new(64);
	let mut context = Context::new();

	// Detect color compatibilty
	// Currently unused, this is slow.
	/*
	let term_colors = stdout.available_colors().unwrap_or(0);
	if term_colors >= 256 {
		context.config.term_color_type = 2;
	} else if term_colors >= 8 {
		context.config.term_color_type = 1;
	} else {
		context.config.term_color_type = 0;
	}
	*/


	// Handle command-line arguments
	let args: Vec<String> = env::args().collect();
	if args.iter().any(|s| s == "--help") {
		let t = command::do_command(&mut context, &String::from("help"));
		t.write(&context, &mut stdout)?;
		let t = command::do_command(&mut context, &String::from("flags"));
		t.write(&context, &mut stdout)?;
		return Ok(());
	} else if args.iter().any(|s| s == "--version") {
		let t = FormattedText::new(format!(
			"Daisy v{}\n", env!("CARGO_PKG_VERSION")
		));
		t.write(&context, &mut stdout)?;
		return Ok(());
	} else if args.iter().any(|s| s == "--info") {
		let t = FormattedText::new(format!(
			concat!(
				"Daisy v{}\n",
				"Your terminal supports {} colors.\n"
			),
			env!("CARGO_PKG_VERSION"),
			stdout.available_colors().unwrap_or(0)
		));
		t.write(&context, &mut stdout)?;
		return Ok(());
	} else if args.iter().any(|s| s == "--256color") {
		context.config.term_color_type = 2;
	} else if args.iter().any(|s| s == "--8color") {
		context.config.term_color_type = 1;
	} else if args.iter().any(|s| s == "--0color") {
		context.config.term_color_type = 0;
	} else if args.iter().any(|s| s == "--nosub") {
		context.config.enable_substituion = false;
	} else if args.iter().any(|s| s == "--nosuper") {
		context.config.enable_super_powers = false;
	} else if args.iter().any(|s| s == "--nooneover") {
		context.config.enable_one_over_power = false;
	}

	context.config.check();


	'outer: loop {

		let t = pb.write_prompt(&mut context);
		t.write(&context, &mut stdout)?;

		let stdin = stdin();
		for c in stdin.keys() {
			if let Key::Char(q) = c.as_ref().unwrap() {
				match q {
					'\n' => {
						// Print again without cursor, in case we pressed enter
						// while inside a substitution
						let t = pb.write_prompt_nocursor(&mut context);
						t.write(&context, &mut stdout)?;


						let in_str = pb.enter();
						FormattedText::newline(&mut stdout)?;
						if in_str == "" { break; }

						if in_str.trim() == "quit" {
							break 'outer;
						} else {
							let r = crate::do_string(&mut context, &in_str);

							match r {
								Ok(t) | Err(t) => {
									t.write(&context, &mut stdout).unwrap();
								}
							}
						}

						break;
					},

					// Only process sane characters
					'a'..='z' | 'A'..='Z' | '0'..='9'
					|'!'|'@'|'#'|'$'|'%'|'^'|'&'|'*'|'('|')'
					|'?'|'~'|','|'.'|'['|']'|' '
					|'<'|'>'|'/'|'_'|'-'|':'|'|'|'='|'+'|';'
					=> { pb.add_char(*q); },

					_ => {}
				};
			} else {
				match c.unwrap() {
					Key::Backspace => { pb.backspace(); },
					Key::Delete => { pb.delete(); },
					Key::Left => { pb.cursor_left(); },
					Key::Right => { pb.cursor_right(); },
					Key::Up => { pb.hist_up(); },
					Key::Down => { pb.hist_down(); },

					Key::Ctrl('d') |
					Key::Ctrl('c') => { break 'outer; },
					_ => {}
				};
			};

			let t = pb.write_prompt(&mut context);
			t.write(&context, &mut stdout)?;
		}
	}

	FormattedText::newline(&mut stdout)?;
	return Ok(());
}