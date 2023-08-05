use std::io::Write;
use std::io::stdout;
use std::io::stdin;
use std::env;

use termion::{
	event::Key,
	input::TermRead,
	raw::IntoRawMode
};

use super::promptbuffer::PromptBuffer;
use crate::command;
use crate::context::Context;


#[inline(always)]
pub fn main() -> Result<(), std::io::Error> {
	let mut stdout = stdout().into_raw_mode().unwrap();
	let mut pb: PromptBuffer = PromptBuffer::new(64);
	let mut context: Context = Context::new();


	// Handle command-line arguments
	let args: Vec<String> = env::args().collect();
	if args.iter().any(|s| s == "--help") {
		let t = command::do_command(&String::from("help"), &mut context);
		t.write(&mut stdout)?;
		return Ok(());
	} else if args.iter().any(|s| s == "--version") {
		write!(stdout, "Daisy v{}\r\n", env!("CARGO_PKG_VERSION"))?;
		return Ok(());
	}

	'outer: loop {

		pb.write_prompt(&mut stdout, &context)?;

		let stdin = stdin();
		for c in stdin.keys() {
			if let Key::Char(q) = c.as_ref().unwrap() {
				match q {
					'\n' => {
						// Print again without cursor, in case we pressed enter
						// while inside a substitution
						pb.write_prompt_nocursor(&mut stdout, &context)?;
						let in_str = pb.enter();
						write!(stdout, "\r\n")?;
						if in_str == "" { break; }

						if in_str.trim() == "quit" {
							break 'outer;
						} else {
							let r = crate::do_string(&in_str, &mut context);

							match r {
								Ok(t) | Err(t) => {
									t.write(&mut stdout).unwrap();
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

			pb.write_prompt(&mut stdout, &context)?;
		}
	}

	write!(stdout, "\r\n")?;
	return Ok(());
}