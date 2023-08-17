use std::io::stdout;
use std::io::stdin;
use std::env;

use termion::{
	event::Key,
	input::TermRead,
	raw::IntoRawMode,
	color::DetectColors
};

use super::promptbuffer::PromptBuffer;
use crate::command;
use crate::context::Context;
use crate::FormattedText;

#[inline(always)]
pub fn main() -> Result<(), std::io::Error> {
	let mut stdout = stdout().into_raw_mode().unwrap();
	let mut pb: PromptBuffer = PromptBuffer::new(64);
	let mut context = Context::new();

	// Set color compatibilty
	let term_colors = stdout.available_colors().unwrap_or(0);
	if term_colors >= 256 {
		context.config.term_color_type = 2
	} else if term_colors >= 8 {
		context.config.term_color_type = 1
	} else {
		context.config.term_color_type = 0
	}

	context.config.check();



	// Handle command-line arguments
	let args: Vec<String> = env::args().collect();
	if args.iter().any(|s| s == "--help") {
		let t = command::do_command(&mut context, &String::from("help"));
		t.write(&context, &mut stdout)?;
		return Ok(());
	} else if args.iter().any(|s| s == "--version") {
		let t = FormattedText::new(format!(
			"Daisy v{}\n", env!("CARGO_PKG_VERSION")
		));
		t.write(&context, &mut stdout)?;
		return Ok(());
	} else if args.iter().any(|s| s == "--debug") {
		let t = FormattedText::new(format!(
			concat!(
				"Daisy v{}\n",
				"Your terminal supports {} colors.\n"
			),
			env!("CARGO_PKG_VERSION"),
			term_colors
		));
		t.write(&context, &mut stdout)?;
		return Ok(());
	}

	'outer: loop {

		pb.write_prompt(&mut context, &mut stdout)?;

		let stdin = stdin();
		for c in stdin.keys() {
			if let Key::Char(q) = c.as_ref().unwrap() {
				match q {
					'\n' => {
						// Print again without cursor, in case we pressed enter
						// while inside a substitution
						pb.write_prompt_nocursor(&mut context, &mut stdout)?;
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
					_ => { pb.add_char(*q); }
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

			pb.write_prompt(&mut context, &mut stdout)?;
		}
	}

	FormattedText::newline(&mut stdout)?;
	return Ok(());
}