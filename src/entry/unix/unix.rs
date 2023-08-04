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
use crate::errors::DaisyError;
use crate::formattedtext::FormattedText;
use crate::parser;
use crate::command;
use crate::evaluate;
use crate::context::Context;
use crate::parser::LineLocation;
use crate::parser::substitute;


#[inline(always)]
fn do_expression(
	stdout: &mut RawTerminal<std::io::Stdout>,
	s: &String,
	context: &mut Context
) -> Result<parser::Expression, (LineLocation, DaisyError)> {

	// Parse string
	#[cfg(debug_assertions)]
	RawTerminal::suspend_raw_mode(&stdout).unwrap();
	let g = parser::parse(&s, context)?;
	#[cfg(debug_assertions)]
	RawTerminal::activate_raw_mode(&stdout).unwrap();

	// Evaluate expression
	#[cfg(debug_assertions)]
	RawTerminal::suspend_raw_mode(&stdout).unwrap();
	let g_evaluated = evaluate::evaluate(&g, context, false)?;
	#[cfg(debug_assertions)]
	RawTerminal::activate_raw_mode(&stdout).unwrap();

	// Display parsed string
	write!(
		stdout, " {}{}=>{}{} {}\r\n",
		style::Bold, color::Fg(color::Magenta),
		style::Reset, color::Fg(color::Reset),
		g.to_string()
	).unwrap();

	// Display result
		write!(
			stdout, "\n  {}{}={} {}{}\r\n\n",
			style::Bold,
			color::Fg(color::Green),
			style::Reset,
		g_evaluated.to_string_outer(),
			color::Fg(color::Reset)
		).unwrap();

	return Ok(g_evaluated);


/*
			Err((l, e)) => {
				// Display user input
				let s = substitute(&s);
				write!(
					stdout, "\n{}{}==>{}{} {}\r\n",
					style::Bold, color::Fg(color::Red),
					style::Reset, color::Fg(color::Reset),
					s
				).unwrap();

				write!(
					stdout, "{}{}{}{}{}{}\r\n",
					color::Fg(color::Red),
					style::Bold,
					" ".repeat(l.pos + 4),
					"^".repeat(l.len),
					color::Fg(color::Reset),
					style::Reset,
				).unwrap();

				write!(
					stdout, "  {}{}Error: {}{}{}\r\n\n",
					style::Bold,
					color::Fg(color::Red),
					style::Reset,
					e.to_string(),
					color::Fg(color::Reset),
				).unwrap();
			}
		}
	}
	*/
}


#[inline(always)]
fn do_assignment(
	stdout: &mut RawTerminal<std::io::Stdout>,
	s: &String,
	context: &mut Context
) -> Result<(), (LineLocation, DaisyError)> {

	let parts = s.split("=").collect::<Vec<&str>>();
	if parts.len() != 2 {
		return Err((
			LineLocation::new_zero(),
			DaisyError::Syntax
		));
	}

	//let offset = parts[0].chars().count() + 1;
	let left = parts[0].trim().to_string();
	let right = parts[1].trim().to_string();
	let right = substitute(&right);
	let left = substitute(&left);


	if !context.valid_varible(&left) {
		return Err((
			LineLocation::new_zero(),
			DaisyError::Syntax
		));
	}
	#[cfg(debug_assertions)]
	RawTerminal::suspend_raw_mode(&stdout).unwrap();
	let g = parser::parse(&right, context)?;
	#[cfg(debug_assertions)]
	RawTerminal::activate_raw_mode(&stdout).unwrap();

	// Display parsed string
	write!(
		stdout, " {}{}=>{}{} {left} = {}\r\n\n",
		style::Bold, color::Fg(color::Magenta),
		style::Reset, color::Fg(color::Reset),
		g.to_string()
	).unwrap();

	// Evaluate expression
	#[cfg(debug_assertions)]
	RawTerminal::suspend_raw_mode(&stdout).unwrap();
	let g_evaluated = evaluate::evaluate(&g, context, false)?;
	#[cfg(debug_assertions)]
	RawTerminal::activate_raw_mode(&stdout).unwrap();

	context.push_var(left.to_string(), g_evaluated).unwrap();
	return Ok(());
}


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


	//let size = termion::terminal_size().unwrap();
	//write!(stdout, "{:?}", size).unwrap();



	'outer: loop {

		pb.write_prompt(&mut stdout)?;

		let stdin = stdin();
		for c in stdin.keys() {
			if let Key::Char(q) = c.as_ref().unwrap() {
				match q {
					'\n' => {
						// Print again without cursor, in case we pressed enter
						// while inside a substitution
						pb.write_prompt_nocursor(&mut stdout)?;
						let in_str = pb.enter();
						write!(stdout, "\r\n")?;
						if in_str == "" { break; }

						if in_str.trim() == "quit" {
							break 'outer;
						} else if command::is_command(&in_str) {
							let t = command::do_command(&in_str, &mut context);
							t.write(&mut stdout)?;
						} else if in_str.contains("=") {
							let r = do_assignment(&mut stdout, &in_str, &mut context);
							if let Err((l, e)) = r {

								let t = FormattedText::new(
									format!(
										concat!(
											"{}[e]{}[n]\n",
											"  {}\n"
										),
										" ".repeat(l.pos + 4),
										"^".repeat(l.len),
										e.text().to_string(),
									)
								);

								t.write(&mut stdout).unwrap();
							}
						} else {
							let r = do_expression(&mut stdout, &in_str, &mut context);
							if let Ok(t) = r {
								context.push_hist(t);
							} else {
								let Err((l, e)) = r else { unreachable!() };

								let t = FormattedText::new(
									format!(
										concat!(
											"{}[e]{}[n]\n",
											"  {}\n"
										),
										" ".repeat(l.pos + 4),
										"^".repeat(l.len),
										e.text().to_string(),
									)
								);

								t.write(&mut stdout).unwrap();
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

			pb.write_prompt(&mut stdout)?;
		}
	}

	write!(stdout, "\r\n")?;
	return Ok(());
}