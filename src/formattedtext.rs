use std::io::Write;
use termion::raw::RawTerminal;
use termion::color;
use termion::style;
use termion::clear;
use termion::cursor;
use std::ops::Add;


#[derive(Debug)]
#[derive(Clone)]
pub struct FormattedText {
	text: String
}

impl ToString for FormattedText {
	fn to_string(&self) -> String { return self.text.clone(); }
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
						('n', ']') => { // Normal text
							s.push_str(&format!("{}{}", color::Fg(color::Reset), style::Reset));
						},
						('i', ']') => { // Normal italic text
							s.push_str(&format!("{}{}", color::Fg(color::Reset), style::Italic));
						},
						('t', ']') => { // Title text
							s.push_str(&format!("{}{}", color::Fg(color::Magenta), style::Bold));
						},
						('a', ']') => { // Colored text
							s.push_str(&format!("{}{}", color::Fg(color::Magenta), style::Reset));
						},
						('e', ']') => { // Error titles
							s.push_str(&format!("{}{}", color::Fg(color::Red), style::Bold));
						},
						('c', ']') => { // Console text
							s.push_str(&format!("{}{}", color::Fg(color::LightBlack), style::Italic));
						},

						('s', ']') => { // Repeat prompt (how => is styled)
							s.push_str(&format!("{}{}", color::Fg(color::Magenta), style::Bold));
						},
						('r', ']') => { // Result prompt (how = is styled)
							s.push_str(&format!("{}{}", color::Fg(color::Green), style::Bold));
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