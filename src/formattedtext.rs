use std::io::Write;
use termion::raw::RawTerminal;
use termion::color;
use termion::style;
use termion::clear;
use termion::cursor;
use std::ops::Add;
use crate::context::Context;


#[derive(Debug)]
#[derive(Clone)]
pub struct FormattedText {
	text: String
}

impl ToString for FormattedText {
	fn to_string(&self) -> String { return self.text.clone(); }
}


fn format_map_none(c: char) -> Option<String> {
	Some(match c {
		'n'|'i'|'t'|'a'|
		'e'|'c'|'s'|'r'
		=> { "".to_string() },
		_ => { return None }
	})
}


fn format_map_ansi(c: char) -> Option<String> {
	Some(match c {
		'n' => { // Normal text
			format!("{}{}", color::Fg(color::Reset), color::Bg(color::Reset))
		},
		'i' => { // Normal italic text
			format!("{}{}", color::Fg(color::Reset), color::Bg(color::Reset))
		},
		't' => { // Title text
			format!("{}{}", color::Fg(color::AnsiValue(6)), color::Bg(color::Reset))
		},
		'a' => { // Colored text
			format!("{}{}", color::Fg(color::AnsiValue(5)), color::Bg(color::Reset))
		},
		'e' => { // Error titles
			format!("{}{}", color::Fg(color::AnsiValue(1)), color::Bg(color::Reset))
		},
		'c' => { // Console text
			format!("{}{}", color::Fg(color::AnsiValue(0)), color::Bg(color::AnsiValue(7)))
		},
		's' => { // Repeat prompt (how => is styled)
			format!("{}{}", color::Fg(color::AnsiValue(2)), color::Bg(color::Reset))
		},
		'r' => { // Result prompt (how = is styled)
			format!("{}{}", color::Fg(color::AnsiValue(4)), color::Bg(color::Reset))
		},

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
		write!(
			stdout,
			"\r\n",
		)?;
		return Ok(());
	}
}


impl FormattedText {
	pub fn new(s: String) -> FormattedText {
		return FormattedText {
			text: s
		}
	}

	pub fn push(&mut self, s: &str) {
		self.text.push_str(s);
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

					// Handle double [[ as escaped [
					if a == '[' { s.push('['); }

					let b = chars.next().unwrap();

					match (a, b) {
						(c, ']') => { // Normal text

							let q = match context.config.term_color_type {
								0 => format_map_none(c),
								1 => format_map_ansi(c),
								2 => format_map_full(c),
								_ => unreachable!("Invalid term_color_type")
							};

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

		write!(stdout, "{}", s)?;
		return Ok(());
	}
}


impl Add for FormattedText {
	type Output = Self;

	fn add(self, other: Self) -> Self::Output {
		return FormattedText::new(format!("{}{}", self.text, other.text));
	}
}