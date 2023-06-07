use std::io::Write;
use std::io::stdout;
use std::io::stdin;


// We only need this for debug
#[cfg(debug_assertions)]
use termion::raw::RawTerminal;

use termion::{
	event::Key,
	input::TermRead,
	raw::IntoRawMode,
	color,
	style,
};

use crate::tokens::EvalError;

use super::promptbuffer::PromptBuffer;
use crate::parser;

/*
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
	\n  {t}Daisy{r}  {v}v{ver}{r}\r\n\n",
		r = format!("{}{}", color::Fg(color::Reset), style::Reset),

		// Icon colors
		a = color::Fg(color::Magenta),
		b = color::Fg(color::White),

		// Title format
		t = format!("{}{}", color::Fg(color::White), style::Bold),

		// Version
		v = format!("{}{}", color::Fg(color::White), style::Italic),
		ver = env!("CARGO_PKG_VERSION"),
	)?;

	return Ok(());
}
*/


#[inline(always)]
pub fn main() -> Result<(), std::io::Error> {
	let mut stdout = stdout().into_raw_mode().unwrap();

	//draw_greeter(&mut stdout)?;

	//let size = termion::terminal_size().unwrap();
	//write!(stdout, "{:?}", size).unwrap();

	let mut pb: PromptBuffer = PromptBuffer::new(64);

	'outer: loop {

		pb.write_prompt(&mut stdout)?;

		let stdin = stdin();
		for c in stdin.keys() {
			if let Key::Char(q) = c.as_ref().unwrap() {
				match q {
					'\n' => {
						let in_str = pb.enter();
						write!(stdout, "\r\n")?;
						if in_str == "" { break; }

						#[cfg(debug_assertions)]
						RawTerminal::suspend_raw_mode(&stdout)?;
						let g = parser::parse(&in_str);
						#[cfg(debug_assertions)]
						RawTerminal::activate_raw_mode(&stdout)?;

						match g {
							Ok(g) => {
								#[cfg(debug_assertions)]
								RawTerminal::suspend_raw_mode(&stdout)?;
								let out_str = g.to_string();
								let g = g.evaluate();
								#[cfg(debug_assertions)]
								RawTerminal::activate_raw_mode(&stdout)?;

								write!(
									stdout, " {}{}=>{}{} {}\r\n",
									style::Bold, color::Fg(color::Magenta),
									style::Reset, color::Fg(color::Reset),
									out_str
								)?;

								match g {
									Ok(q) => {
										write!(
											stdout, "\n  {}{}={} {}{}\r\n\n",
											style::Bold,
											color::Fg(color::Green),
											style::Reset,
											q.to_string_outer(),
											color::Fg(color::Reset)
										)?;
									},

									Err(EvalError::BadMath) => {
										write!(
											stdout, "\n  {}{}Mathematical Error: {}Failed to evaluate expression{}\r\n\n",
											style::Bold,
											color::Fg(color::Red),
											style::Reset,
											color::Fg(color::Reset),
										)?;
									},

									Err(EvalError::IncompatibleUnit) => {
										write!(
											stdout, "\n  {}{}Evaluation Error: {}Incompatible units{}\r\n\n",
											style::Bold,
											color::Fg(color::Red),
											style::Reset,
											color::Fg(color::Reset),
										)?;
									}
								}
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
					Key::Left => { pb.cursor_left(); },
					Key::Right => { pb.cursor_right(); },
					Key::Up => { pb.hist_up(); },
					Key::Down => { pb.hist_down(); },

					Key::Ctrl('d') |
					Key::Ctrl('c') => { break 'outer; },
					_ => {}
				};
			};

			pb.write_prompt(&mut stdout)?;
		}
	}

	write!(stdout, "\r\n")?;
	return Ok(());
}