use std::io::{Write, stdout, stdin};

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::raw::RawTerminal;
use termion::{color, style};

mod parser;
mod promptbuffer;
use crate::promptbuffer::PromptBuffer;

use crate::parser::Token;
//use crate::parser::ParserError;
use crate::parser::LineLocation;


fn draw_line(
	stdout: &mut RawTerminal<std::io::Stdout>,
	s: &String,
	clear_len: usize
) -> Result<(), std::io::Error> {
	write!(
		stdout, "\r{}{}==>{}{} {}",
		style::Bold,
		color::Fg(color::Blue),
		color::Fg(color::Reset),
		style::Reset,
		s
	)?;

	// If this string is shorter, clear the remaining old one.
	if clear_len != 0 {
		write!(
			stdout, "{}{}",
			" ".repeat(clear_len as usize),
			termion::cursor::Left(clear_len as u16)
		)?;
	}

	stdout.flush()?;

	return Ok(());
}

fn main() -> Result<(), std::io::Error> {
	let mut stdout = stdout().into_raw_mode().unwrap();

	//let size = termion::terminal_size().unwrap();  
	//write!(stdout, "{:?}", size).unwrap();

	let mut pb: PromptBuffer = PromptBuffer::new(64);
	let mut last_len: usize = 0;

	'outer: loop {

		draw_line(
			&mut stdout,
			pb.get_contents(),
			if pb.get_contents().len() >= last_len
			{ 0 } else {last_len - pb.get_contents().len()}
		)?;
		last_len = pb.get_contents().len();
		

		let stdin = stdin();
		for c in stdin.keys() {
			if let Key::Char(q) = c.as_ref().unwrap() {
				match q {
					'\n' => {
						let s = pb.enter();
						if s == "" { write!(stdout, "\r\n")?; break; }

						RawTerminal::suspend_raw_mode(&stdout)?;
						write!(stdout, "\n")?;
						let g = parser::parse(&s);
						RawTerminal::activate_raw_mode(&stdout)?;

						match g {
							Ok(g) => {
								let n = g.eval();
								if let Token::Number(_, v) = n {
									write!(
										stdout, "\r\n  {}{}={} {v}{}\r\n\n",
										style::Bold,
										color::Fg(color::Green),
										style::Reset,
										color::Fg(color::Reset)
									)?;
								} else { panic!(); }
							},
							Err((l, e)) => {
								let LineLocation{pos, len} = l;
								write!(
									stdout, "{}{}{} {e:?}{}\r\n",
									color::Fg(color::Red),
									" ".repeat(pos + 4),
									"^".repeat(len),
									color::Fg(color::Reset),
								)?;
							}
						};

						break;
					},
					'/' => { pb.add_char('÷'); },
					'*' => { pb.add_char('×'); },
					_ => { pb.add_char(*q); }
				};
			} else {
				match c.unwrap() {
					Key::Backspace => { pb.backspace(); },
					Key::Delete => { pb.delete(); },
					Key::Left => {},
					Key::Right => {},
					Key::Up => { pb.hist_up(); },
					Key::Down => { pb.hist_down(); },
					
					Key::Ctrl('d') |
					Key::Ctrl('c') => { break 'outer; },
					_ => {}
				};
			};

			draw_line(
				&mut stdout,
				pb.get_contents(),
				if pb.get_contents().len() >= last_len
				{ 0 } else {last_len - pb.get_contents().len()}
			)?;
			last_len = pb.get_contents().len();
		}
	}

	write!(stdout, "\r\n")?;
	return Ok(());
}