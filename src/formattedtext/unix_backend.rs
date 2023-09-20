use super::FormattedText;
use std::io::Write;
use crate::context::Context;

use termion::raw::RawTerminal;
use termion::color;
use termion::style;
use termion::clear;
use termion::cursor;

fn format_map_ansi(c: char) -> Option<String> {
	Some(match c {
		'n' => { // Normal text
			format!("{}{}", color::Fg(color::Reset), color::Bg(color::Reset))
		},
		'i' => { // Normal italic text
			format!("{}{}", color::Fg(color::Reset), color::Bg(color::Reset))
		},
		't' => { // Title text (should be cyan)
			format!("{}{}", color::Fg(color::AnsiValue(6)), color::Bg(color::Reset))
		},
		'a' => { // Colored text (should be pink)
			format!("{}{}", color::Fg(color::AnsiValue(5)), color::Bg(color::Reset))
		},
		'e' => { // Error titles (should be red)
			format!("{}{}", color::Fg(color::AnsiValue(1)), color::Bg(color::Reset))
		},
		'c' => { // Console text (inverted black on white)
			format!("{}{}", color::Fg(color::AnsiValue(0)), color::Bg(color::AnsiValue(7)))
		},
		'p' => { // Input prompt (how ==> is styled) (should be blue)
			format!("{}{}", color::Fg(color::AnsiValue(4)), color::Bg(color::Reset))
		},
		's' => { // Repeat prompt (how => is styled) (should be pink)
			format!("{}{}", color::Fg(color::AnsiValue(5)), color::Bg(color::Reset))
		},
		'r' => { // Result prompt (how = is styled) (should be green)
			format!("{}{}", color::Fg(color::AnsiValue(2)), color::Bg(color::Reset))
		},

		_ => { return None }
	})
}



fn format_map_none(c: char) -> Option<String> {
	Some(match c {
		'n'|'i'|'t'|'a'|
		'e'|'c'|'s'|'r'|
		'p'
		=> { "".to_string() },
		_ => { return None }
	})
}

// style::reset also resets color.
// Make sure color comes AFTER style reset.
fn format_map_full(c: char) -> Option<String> {
	Some(match c {
		'n' => { // Normal text
			format!("{}{}", style::Reset, color::Fg(color::Reset))
		},
		'i' => { // Normal italic text
			format!("{}{}", color::Fg(color::Reset), style::Italic)
		},
		't' => { // Title text
			format!("{}{}", color::Fg(color::Magenta), style::Bold)
		},
		'a' => { // Colored text
			format!("{}{}", style::Reset, color::Fg(color::Magenta))
		},
		'e' => { // Error titles
			format!("{}{}", color::Fg(color::Red), style::Bold)
		},
		'c' => { // Console text
			format!("{}{}", color::Fg(color::LightBlack), style::Italic)
		},
		'p' => { // Input prompt (how ==> is styled)
			format!("{}{}", color::Fg(color::Blue), style::Bold)
		},
		's' => { // Repeat prompt (how => is styled)
			format!("{}{}", color::Fg(color::Magenta), style::Bold)
		},
		'r' => { // Result prompt (how = is styled)
			format!("{}{}", color::Fg(color::Green), style::Bold)
		},


		_ => { return None }
	})
}

impl FormattedText {
	pub fn newline(stdout: &mut RawTerminal<std::io::Stdout>) -> Result<(), std::io::Error> {
		write!(stdout, "\n")?;
		return Ok(());
	}

	pub fn format_map(c: char, context: &Context) -> Option<String> {
		match context.config.term_color_type {
			0 => format_map_none(c),
			1 => format_map_ansi(c),
			2 => format_map_full(c),
			_ => unreachable!("Invalid term_color_type")
		}
	}

	pub fn write(&self, context: &Context, stdout: &mut RawTerminal<std::io::Stdout>) -> Result<(), std::io::Error> {

		if self.text == "[clear]" {
			write!(
				stdout,
				"{}{}",
				clear::All,
				cursor::Goto(1, 1)
			)?;
			return Ok(());
		}


		let mut s = String::new();
		let mut chars = self.text.chars();

		while let Some(c) = chars.next() {
			match c {
				'[' => {
					let a = chars.next().unwrap();

					// Treat double [[ as escaped [
					if a == '[' { s.push('['); }

					let b = chars.next().unwrap();

					match (a, b) {
						(c, ']') => { // Normal text

							let q = Self::format_map(c, context);

							if q.is_some() {
								s.push_str(&q.unwrap());
							} else {
								s.push('[');
								s.push(a);
								s.push(b);
							}
						},

						_ => {
							s.push('[');
							s.push(a);
							s.push(b);
						}
					}
				},
				'\n' => { s.push_str("\r\n") },
				_ => s.push(c)
			}
		}

		write!(stdout, "\r{}", s)?;
		return Ok(());
	}
}