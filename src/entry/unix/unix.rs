use std::io::Write;
use std::io::stdout;
use std::io::stdin;
use std::env;

use termion::{
	event::Key,
	input::TermRead,
	raw::IntoRawMode,
	raw::RawTerminal,
	color,
	style,
};

use super::promptbuffer::PromptBuffer;
//use crate::tokens::EvalError;
use crate::parser;
use crate::command;
use crate::evaluate::evaluate;
use crate::evaluate::EvalError;




#[inline(always)]
fn do_expression(
	stdout: &mut RawTerminal<std::io::Stdout>,
	s: &String,
	history: &Vec<parser::Token>
) -> Result<parser::Token, ()> {
	#[cfg(debug_assertions)]
	RawTerminal::suspend_raw_mode(&stdout).unwrap();
	let g = parser::parse(&s);
	#[cfg(debug_assertions)]
	RawTerminal::activate_raw_mode(&stdout).unwrap();

	// Check for parse errors
	if let Err((l, e)) = g {
		write!(
			stdout, "{}{}{} {}{}\r\n",
			color::Fg(color::Red),
			" ".repeat(l.pos + 4),
			"^".repeat(l.len),
			e.to_string(),
			color::Fg(color::Reset),
		).unwrap();
		return Err(());
	}

	let Ok(g) = g else {panic!()};


	// Display parsed string
	write!(
		stdout, " {}{}=>{}{} {}\r\n",
		style::Bold, color::Fg(color::Magenta),
		style::Reset, color::Fg(color::Reset),
		g.to_string()
	).unwrap();


	// Evaluate expression
	#[cfg(debug_assertions)]
	RawTerminal::suspend_raw_mode(&stdout).unwrap();
	let g = evaluate(&g, history);
	#[cfg(debug_assertions)]
	RawTerminal::activate_raw_mode(&stdout).unwrap();

	// Show output
	if let Ok(q) = g {
		write!(
			stdout, "\n  {}{}={} {}{}\r\n\n",
			style::Bold,
			color::Fg(color::Green),
			style::Reset,
			q.to_string_outer(),
			color::Fg(color::Reset)
		).unwrap();
		return Ok(q);

	} else {
		match g {
			Ok(_) => panic!(),

			Err(EvalError::TooBig) => {
				write!(
					stdout, "\n  {}{}Mathematical Error: {}Number too big{}\r\n\n",
					style::Bold,
					color::Fg(color::Red),
					style::Reset,
					color::Fg(color::Reset),
				).unwrap();
			},

			Err(EvalError::ZeroDivision) => {
				write!(
					stdout, "\n  {}{}Mathematical Error: {}Division by zero{}\r\n\n",
					style::Bold,
					color::Fg(color::Red),
					style::Reset,
					color::Fg(color::Reset),
				).unwrap();
			},

			Err(EvalError::BadMath) => {
				write!(
					stdout, "\n  {}{}Mathematical Error: {}Failed to evaluate expression{}\r\n\n",
					style::Bold,
					color::Fg(color::Red),
					style::Reset,
					color::Fg(color::Reset),
				).unwrap();
			},

			Err(EvalError::IncompatibleUnit) => {
				write!(
					stdout, "\n  {}{}Evaluation Error: {}Incompatible units{}\r\n\n",
					style::Bold,
					color::Fg(color::Red),
					style::Reset,
					color::Fg(color::Reset),
				).unwrap();
			},

			Err(EvalError::NoHistory) => {
				write!(
					stdout, "\n  {}{}Evaluation Error: {}There is no previous answer to reference{}\r\n\n",
					style::Bold,
					color::Fg(color::Red),
					style::Reset,
					color::Fg(color::Reset),
				).unwrap();
			}
		}
	}

	return Err(());
}


#[inline(always)]
pub fn main() -> Result<(), std::io::Error> {
	let mut stdout = stdout().into_raw_mode().unwrap();

	let args: Vec<String> = env::args().collect();

	// Handle command-line arguments
	if args.iter().any(|s| s == "--help") {
		command::do_command(&mut stdout, &String::from("help"))?;
		return Ok(());
	} else if args.iter().any(|s| s == "--version") {
		write!(stdout, "Daisy v{}\r\n", env!("CARGO_PKG_VERSION"))?;
		return Ok(());
	}


	//let size = termion::terminal_size().unwrap();
	//write!(stdout, "{:?}", size).unwrap();

	let mut pb: PromptBuffer = PromptBuffer::new(64);
	let mut history: Vec<parser::Token> = Vec::new();


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

						if in_str.trim() == "quit" {
							break 'outer;
						} else if command::is_command(&in_str) {
							command::do_command(&mut stdout, &in_str)?;
						} else {
							let r = do_expression(&mut stdout, &in_str, &history);
							if let Ok(t) = r { history.push(t); }
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

			pb.write_prompt(&mut stdout)?;
		}
	}

	write!(stdout, "\r\n")?;
	return Ok(());
}