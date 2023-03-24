use std::io::{Write, stdout, stdin};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::raw::RawTerminal;
use termion::{color, style};

mod parser;
use crate::parser::Token;
//use crate::parser::ParserError;
use crate::parser::LineLocation;
use crate::parser::Eval;


fn draw_line(stdout: &mut RawTerminal<std::io::Stdout>, s: &String) -> Result<(), std::io::Error> {
	write!(
		stdout, "\r{}{}==>{}{} {s} {}",
		style::Bold,
		color::Fg(color::Blue),
		color::Fg(color::Reset),
		style::Reset,

		// Our string can change by at most one character each iteration.
		// Clearing is done by inserting an extra space, then moving the cursor left.
		termion::cursor::Left(1)
	)?;
	stdout.flush()?;

	return Ok(());
}

fn main() -> Result<(), std::io::Error> {
	let mut stdout = stdout().into_raw_mode().unwrap();

	//let size = termion::terminal_size().unwrap();  
	//write!(stdout, "{:?}", size).unwrap();

	let mut s: String = String::with_capacity(64);

	'outer: loop {

		s.clear();
		draw_line(&mut stdout, &s)?;

		let stdin = stdin();
		for c in stdin.keys() {
			if let Key::Char(q) = c.as_ref().unwrap() {
				match q {
					'\n' => {
						let s = s.trim().to_string();
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
					'/' => { s.push('÷'); },
					'*' => { s.push('×'); },
					_ => { s.push(*q); }
				};
			} else {
				match c.unwrap() {
					Key::Backspace => { s.pop(); },
					Key::Delete => { s.pop(); },
					Key::Left => {},
					Key::Right => {},
					Key::Up => {},
					Key::Down => {},
					
					Key::Ctrl('d') |
					Key::Ctrl('c') => { break 'outer; },
					_ => {}
				};
			};
			draw_line(&mut stdout, &s)?;
		}
	}

	write!(stdout, "\r\n")?;
	return Ok(());
}