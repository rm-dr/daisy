use std::io::{Write, stdout, stdin};

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::raw::RawTerminal;
use termion::{color, style};

pub mod tokens;
pub mod parser;
pub mod evaluate;
pub mod quantity;

mod promptbuffer;


use crate::promptbuffer::PromptBuffer;

//use crate::tokens::Token;
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

fn main() -> Result<(), std::io::Error> {
	let mut stdout = stdout().into_raw_mode().unwrap();

	draw_greeter(&mut stdout)?;

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
								let out_str = g.print();
								let g = evaluate::evaluate(g);
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
											q.print(),
											color::Fg(color::Reset)
										)?;
									},

									Err(_) => {
										write!(
											stdout, "\n  {}{}Mathematical Error: {}Failed to evaluate expression.{}\r\n\n",
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

#[cfg(test)]
mod tests {
	// Many of these have been borrowed from insect.
	use crate::parser;
	use crate::evaluate;

	fn eval_to_str(s: &str) -> Result<String, ()> {
		let g = match parser::parse(&String::from(s)) {
			Ok(x) => x,
			Err(_) => return Err(())
		};
		//let out_str = g.print();

		return match evaluate::evaluate(g) {
			Ok(x) => Ok(x.print()),
			Err(_) => Err(())
		};
	}

	fn good_expr(r: &str, s: &str) {
		let out = eval_to_str(s).unwrap();
		assert_eq!(r, out);
	}

	fn bad_expr(s: &str) {
		let out = eval_to_str(s);

		match out {
			Err(_) => { return },
			_ => {}
		};

		panic!()
	}

	#[test]
	fn basic_numbers() {
		good_expr("1", "1");
		good_expr("1", "1.0");
		good_expr("1", "1.0000");
		good_expr("1", "+1.0");
		good_expr("1", "+1");
		good_expr("3.5", "3.5");
		good_expr("3.5", "3.50");
		good_expr("3.5", "+3.50");
		good_expr("0.2", "0.2");
		good_expr("0.2", "+0.2 ");
		good_expr("0.2", ".2");
		good_expr("0.2", "+.2");
		good_expr("-0.61", "-0.61");
		good_expr("-0.61", "-.61");
		good_expr("-0.61", "-   .61");
		good_expr("0.05", ".05");
		good_expr("-123.45", "-123.45");

		bad_expr("123..");
		bad_expr("0..");
		bad_expr(".0.");
		bad_expr(".");
		bad_expr(". 2");
		bad_expr("..2");
	}

	#[test]
	fn big_numbers() {
		good_expr("1.2346e15", "1234567890000000");
		good_expr("1.2346e15", "1234567890000000.0");
		good_expr("1.2346e15", "+1234567890000000.0");
	}

	#[test]
	fn signs() {
		good_expr( "5", "+++++5");
		good_expr( "5", "++++5");
		good_expr( "5", "+++5");
		good_expr( "5", "++5");
		good_expr( "5", "+5");
		good_expr("-5", "-5");
		good_expr( "5", "--5");
		good_expr("-5", "---5");
		good_expr( "5", "----5");
		good_expr("-5", "-----5");
	}

	#[test]
	fn bad_expressions() {
		bad_expr("2^");
		bad_expr("^2");
		bad_expr("5*");
		bad_expr("5/");
		bad_expr("5%");
		bad_expr("%2");
		bad_expr("3 + ");
		bad_expr("3 + @");
		bad_expr("3 - ");
		bad_expr("()");
		bad_expr("3+2)");
	}

	#[test]
	fn implicit_multiply() {
		good_expr("15", "5(3)");
		good_expr("15", "(5)3");
		good_expr("15", "(5)(3)");
		bad_expr("5 2");
	}


	#[test]
	fn scientific() {
		good_expr("100", "1e2");
		good_expr("0.01", "1e-2");
		good_expr("1", "1e0");

		// In these expressions, `e` is euler's number
		// under implicit multiplication
		good_expr("5.4366", "1e(2)");
		good_expr("14.778", "e2e");

		bad_expr("2 2e2");
		bad_expr("1e1.2");
	}


	#[test]
	fn operators() {

		good_expr("125", "5^(+3)");
		good_expr("125", "+5^3");
		good_expr("0.2148", "3 ^ (-1.4)");

		// Should parse as ((2^3)^4)^5
		good_expr("1.1529e18", "2^3^4^5");

		// Should parse as 1/(2pi)
		good_expr("0.15915", "1/2pi");
		// Should parse as (1/2)*pi
		good_expr("1.5708", "1/2*pi");



		good_expr("15", "5*3");
		good_expr("15", "5 * 3 ");
		good_expr("15", "( 5 ) * ( 3 )");
		good_expr("15", "( 5 ) ( 3 )");
		good_expr("15", "( ( 5 ) * ( 3 ) )");
		good_expr("15", "((5)*(3");
		good_expr("15", "( 5 * 3 )");
		good_expr("15", "5(+3)");
		good_expr("15", "+5*3");

		good_expr("-15", "5*(-3)");
		good_expr("-15", "5 * (-3)");
		good_expr("-15", "( 5 ) * ( -3 )");
		good_expr("-15", "( ( 5 ) * (-( 3 )) )");
		good_expr("-15", "( 5 * (-3) )");
		good_expr("-15", "+5*(-3)");

		good_expr("2", "6/3");
		good_expr("2", "5%3");
		good_expr("8", "5+3");
		good_expr("64", "4^3");
		good_expr("64", "4 ^ 3");
		good_expr("64", "4**3");
		good_expr("-81", "-3^4");
		good_expr("-81", "-(3^4)");
		good_expr("0.5", "2^-1");
		good_expr("0.25", "2^-2");

		good_expr("2", "rt 4");
		good_expr("2", "sqrt 4");
		good_expr("6", "2 rt 9");

		good_expr("7", "3!+1");
		good_expr("18", "3!3");
		bad_expr("3.1!");
		bad_expr("pi!");
	}
}