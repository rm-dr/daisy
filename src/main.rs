use std::io::{Write, stdout, stdin};

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::raw::RawTerminal;
use termion::{color, style};

pub mod tokens;
mod parser;
mod promptbuffer;
mod evaluate;

use crate::tokens::Token;
use crate::promptbuffer::PromptBuffer;

//use crate::parser::ParserError;
//use crate::parser::LineLocation;


/*
  ######  @@@@@@
 #     ##@@     @
 ##     #@     @@
   @@@@@@@@@@@@@
 @@     @#     ##
 @     @@##     #
  @@@@@@  ######

   Daisy 0.0.1
*/
#[inline(always)]
fn draw_greeter(stdout: &mut RawTerminal<std::io::Stdout>) -> Result<(), std::io::Error> {
	write!(
		stdout,
	"\n \
	{a} ###### {b} @@@@@@\r\n \
	{a}#     ##{b}@@     @\r\n \
	{a}##     #{b}@     @@\r\n \
	{a}  {b}@@@@@@@@@@@@@{a}\r\n \
	{b}@@     @{a}#     ##\r\n \
	{b}@     @@{a}##     #\r\n \
	{b} @@@@@@ {a} ###### {r}\r\n \
	\n  {t}Daisy{r}  {v}v0.0.0{r}\r\n\n",
		a = color::Fg(color::Magenta),
		b = color::Fg(color::White),
		t = format!("{}{}", color::Fg(color::White), style::Bold),
		v = format!("{}{}", color::Fg(color::White), style::Italic),

		r = format!("{}{}", color::Fg(color::Reset), style::Reset),
	)?;

	return Ok(());
}


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

/*

#[cfg(debug_assertions)]
RawTerminal::suspend_raw_mode(&stdout)?;
#[cfg(debug_assertions)]
write!(stdout, "\n")?;

#[cfg(debug_assertions)]
RawTerminal::activate_raw_mode(&stdout)?;
*/

fn main() -> Result<(), std::io::Error> {
	let mut stdout = stdout().into_raw_mode().unwrap();

	draw_greeter(&mut stdout)?;

	//let size = termion::terminal_size().unwrap();  
	//write!(stdout, "{:?}", size).unwrap();

	let mut pb: PromptBuffer = PromptBuffer::new(64);
	let mut last_len: usize = 0;

	'outer: loop {

		let s = parser::substitute(&pb.get_contents());
		draw_line(
			&mut stdout, &s,
			if s.chars().count() >= last_len
			{ 0 } else {last_len - s.chars().count()}
		)?;
		last_len = s.chars().count();

		let stdin = stdin();
		for c in stdin.keys() {
			if let Key::Char(q) = c.as_ref().unwrap() {
				match q {
					'\n' => {
						let s = pb.enter();
						write!(stdout, "\r\n")?;
						if s == "" { break; }

						#[cfg(debug_assertions)]
						RawTerminal::suspend_raw_mode(&stdout)?;
						let g = parser::parse(&s);
						#[cfg(debug_assertions)]
						RawTerminal::activate_raw_mode(&stdout)?;

						match g {
							Ok(g) => {
								#[cfg(debug_assertions)]
								RawTerminal::suspend_raw_mode(&stdout)?;
								let g = evaluate::evaluate(g).unwrap();
								let n = g.eval();
								#[cfg(debug_assertions)]
								RawTerminal::activate_raw_mode(&stdout)?;


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

							// Show parse error
							Err((l, e)) => {
								write!(
									stdout, "{}{}{} {}{}\r\n",
									color::Fg(color::Red),
									" ".repeat(l.pos + 4),
									"^".repeat(l.len),
									e.to_message(),
									color::Fg(color::Reset),
								)?;
							}
						};

						break;
					},
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

			let s = parser::substitute(&pb.get_contents());
			draw_line(
				&mut stdout, &s,
				if s.chars().count() >= last_len
				{ 0 } else {last_len - s.chars().count()}
			)?;
			last_len = s.chars().count();
		}
	}

	write!(stdout, "\r\n")?;
	return Ok(());
}