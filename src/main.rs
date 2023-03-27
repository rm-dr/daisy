use std::io::{Write, stdout, stdin};

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::raw::RawTerminal;
use termion::{color, style};

pub mod tokens;
pub mod parser;
mod promptbuffer;
pub mod evaluate;

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






#[cfg(test)]
mod tests {
	// Many of these have been borrowed from insect.
	use crate::parser;
	use crate::evaluate;
	use crate::tokens;

	fn good_expr(r: f64, s: &str) {
		let s = String::from(s);
		let g = parser::parse(&s).unwrap();
		let g = evaluate::evaluate(g).unwrap();
		let n = g.eval();
		let tokens::Token::Number(_, v) = n else {panic!()};
		assert_eq!(v, r);
	}

	fn bad_expr(s: &str) {
		let s = String::from(s);
		match parser::parse(&s) {
			Err(_) => { return },
			_ => {}
		};

		panic!()
	}

	#[test]
	fn basic_numbers() {
		good_expr(1f64, "1");
		good_expr(1f64, "1.0");
		good_expr(1f64, "1.0000");
		//good_expr(1f64, "+1.0");
		//good_expr(1f64, "+1");
		good_expr(3.5f64, "3.5");
		good_expr(3.5f64, "3.50");
		//good_expr(3.5f64, "+3.50");
		good_expr(0.2f64, "0.2");
		//good_expr(0.2f64, "+0.2 ");
		good_expr(0.2f64, ".2");
		//good_expr(0.2f64, "+.2");
		good_expr(-0.61f64, "-0.61");
		good_expr(-0.61f64, "-.61");
		good_expr(-0.61f64, "-   .61");
		good_expr(0.05f64, ".05");
		good_expr(-123.45f64, "-123.45");

		bad_expr("123..");
		bad_expr("0..");
		bad_expr(".0.");
		bad_expr(".");
		bad_expr(". 2");
		bad_expr("..2");
	}

	#[test]
	fn big_numbers() {
		good_expr(1234567890000000f64, "1234567890000000");
		good_expr(1234567890000000f64, "1234567890000000.0");
		//good_expr(1234567890000000f64, "+1234567890000000.0");
	}


	#[test]
	fn negatives() {
		good_expr(-5f64, "-5");
		good_expr(5f64,  "--5");
		good_expr(-5f64, "---5");
		good_expr(5f64,  "----5");
		good_expr(-5f64, "-----5");
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
	fn operators() {
		good_expr(125f64, "5^3");
		good_expr(125f64, "5 ^ 3");
		good_expr(125f64, "( 5 ) ^ ( 3 )");
		good_expr(125f64, "( ( 5 ) ^ ( 3 ) )");
		good_expr(125f64, "( ( 5 ^ ( 3");
		good_expr(125f64, "( 5 ^ 3 )");
		//good_expr(125f64, "5^(+3)");
		//good_expr(125f64, "+5^3");
	
		good_expr(64f64, "4^3");
		good_expr(64f64, "4 ^ 3");
		//good_expr(64f64, "4**3");
		//good_expr(64f64, "4 ** 3");

		good_expr(-81f64, "-3^4");
		good_expr(-81f64, "-3 ^ 4");
		//good_expr(-81f64, "-3**4");
		//good_expr(-81f64, "-3 ** 4");
		good_expr(-81f64, "-(3^4)");

		//good_expr(f64, "3 ^ (-1.4)");
		//good_expr(f64, "3 ** (-1.4)");
	
		//good_expr(f64, "2^3^4^5");
		good_expr(0.5f64, "2^-1");
		good_expr(0.25f64, "2^-2");


		good_expr(15f64, "5*3");
		good_expr(15f64, "5 * 3 ");
		good_expr(15f64, "( 5 ) * ( 3 )");
		good_expr(15f64, "( 5 ) ( 3 )");
		good_expr(15f64, "( ( 5 ) * ( 3 ) )");
		good_expr(15f64, "( 5 * 3 )");
		good_expr(15f64, "5(3)");
		good_expr(15f64, "(5)3");
		//good_expr(15f64, "5(+3)");
		//good_expr(15f64, "+5*3");
	
		good_expr(-15f64, "5*(-3)");
		good_expr(-15f64, "5 * (-3)");
		good_expr(-15f64, "( 5 ) * ( -3 )");
		good_expr(-15f64, "( ( 5 ) * (-( 3 )) )");
		good_expr(-15f64, "( 5 * (-3) )");
		//good_expr(-15f64, "+5*(-3)");

		good_expr(2f64, "6/3");
		good_expr(2f64, "6 / 3");
		good_expr(2f64, "( 6 ) / ( 3 )");
		good_expr(2f64, "( ( 6 ) / ( 3 ) )");
		good_expr(2f64, "( 6 / 3 )");

		good_expr(2f64, "5%3");
		good_expr(2f64, "5 % 3");
		good_expr(2f64, "( 5 ) % ( 3 )");
		good_expr(2f64, "( ( 5 ) % ( 3 ) )");
		good_expr(2f64, "( 5 % 3 )");

		good_expr(8f64, "5+3");
		good_expr(8f64, "5 + 3");
		good_expr(8f64, "( 5 ) + ( 3 )");
		good_expr(8f64, "( ( 5 ) + ( 3 ) )");
		good_expr(8f64, "( 5 + 3 )");

		good_expr(2f64, "5-3");
		good_expr(2f64, "5 - 3");
		good_expr(2f64, "( 5 ) - ( 3 )");
		good_expr(2f64, "( ( 5 ) - ( 3 ) )");
		good_expr(2f64, "( 5 - 3 )");
	}
}