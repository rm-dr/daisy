use super::FormattedText;
use std::io::Write;
use crate::context::Context;

use termion::raw::RawTerminal;
use termion::color;
use termion::style;
use termion::clear;
use termion::cursor;

fn format_map_ansi(s: &str) -> Option<String> {
	Some(match s {
		"n" => { // Normal text
			format!("{}{}", color::Fg(color::Reset), color::Bg(color::Reset))
		},
		"i" => { // Normal italic text
			format!("{}{}", color::Fg(color::Reset), color::Bg(color::Reset))
		},
		"t" => { // Title text (should be cyan)
			format!("{}{}", color::Fg(color::AnsiValue(6)), color::Bg(color::Reset))
		},
		"a" => { // Colored text (should be pink)
			format!("{}{}", color::Fg(color::AnsiValue(5)), color::Bg(color::Reset))
		},
		"e" => { // Error titles (should be red)
			format!("{}{}", color::Fg(color::AnsiValue(1)), color::Bg(color::Reset))
		},
		"c" => { // Console text (inverted black on white)
			format!("{}{}", color::Fg(color::AnsiValue(0)), color::Bg(color::AnsiValue(7)))
		},
		"p" => { // Input prompt (how ==> is styled) (should be blue)
			format!("{}{}", color::Fg(color::AnsiValue(4)), color::Bg(color::Reset))
		},
		"s" => { // Repeat prompt (how => is styled) (should be pink)
			format!("{}{}", color::Fg(color::AnsiValue(5)), color::Bg(color::Reset))
		},
		"r" => { // Result prompt (how = is styled) (should be green)
			format!("{}{}", color::Fg(color::AnsiValue(2)), color::Bg(color::Reset))
		},

		_ => { return None }
	})
}



fn format_map_none(s: &str) -> Option<String> {
	Some(match s {
		"n"|"i"|"t"|"a"|
		"e"|"c"|"s"|"r"|
		"p"
		=> { "".to_string() },
		_ => { return None }
	})
}

// style::reset also resets color.
// Make sure color comes AFTER style reset.
fn format_map_full(s: &str) -> Option<String> {
	Some(match s {
		"n" => { // Normal text
			format!("{}{}", style::Reset, color::Fg(color::Reset))
		},
		"i" => { // Normal italic text
			format!("{}{}", color::Fg(color::Reset), style::Italic)
		},
		"t" => { // Title text
			format!("{}{}", color::Fg(color::Magenta), style::Bold)
		},
		"a" => { // Colored text
			format!("{}{}", style::Reset, color::Fg(color::Magenta))
		},
		"e" => { // Error titles
			format!("{}{}", color::Fg(color::Red), style::Bold)
		},
		"c" => { // Console text
			format!("{}{}", color::Fg(color::LightBlack), style::Italic)
		},
		"p" => { // Input prompt (how ==> is styled)
			format!("{}{}", color::Fg(color::Blue), style::Bold)
		},
		"s" => { // Repeat prompt (how => is styled)
			format!("{}{}", color::Fg(color::Magenta), style::Bold)
		},
		"r" => { // Result prompt (how = is styled)
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

	pub fn format_map(s: &str, context: &Context) -> Option<String> {
		match context.config.term_color_type {
			0 => format_map_none(s),
			1 => format_map_ansi(s),
			2 => format_map_full(s),
			_ => unreachable!("Invalid term_color_type")
		}
	}

	pub fn write(&self, context: &Context, stdout: &mut RawTerminal<std::io::Stdout>) -> Result<(), std::io::Error> {

		let mut word = String::new();
		let mut reading = false; // are we reading a word?
		let mut chars = self.text.chars();
		let mut out = String::new();

		while let Some(c) = chars.next() {

			match c {
				'[' => {
					if reading {
						// Discard old word, start reading again.
						out.push_str(&word);
						word.clear();
					} 
					
					// Start reading a new word
					reading = true;
					word.push(c);
				},

				']' => {
					if !reading {
						out.push(c);
					} else {
						word.push(c);


						let f = Self::format_map(&word[1..word.len()-1], context);

						if f.is_some() {
							out.push_str(&f.unwrap());
						} else if word == "[clear]" {
							out.push_str(&format!(
								"{}{}",
								clear::All,
								cursor::Goto(1, 1)
							));
						} else if word.starts_with("[cursorright") {
							let n: u16 = word[12..word.len()-1].parse().unwrap();
							out.push_str(&format!(
								"{}",
								cursor::Right(n),
							));
						} else {
							out.push_str(&word);
						}

						reading = false;
						word.clear();
					}
				},

				'\n' => {
					if reading { word.push_str("\r\n"); }
					else { out.push_str("\r\n"); }
				},

				_ => {
					if reading { word.push(c); }
					else { out.push(c); }
				}
			}
		}

		write!(stdout, "\r{}", out)?;
		stdout.flush()?;
		return Ok(());
	}
}