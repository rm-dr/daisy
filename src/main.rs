use std::io;
use std::io::Write;
//use std::io::Read;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};



use termcolor::{
	Color,
	ColorChoice,
	ColorSpec,
	StandardStream,
	WriteColor
};

mod parser;
use crate::parser::Token;
//use crate::parser::ParserError;
use crate::parser::LineLocation;

const PROMPT_PREFIX: &str = "==> ";

/// Show a prompt and save trimmed input to `input`.
///
/// # Arguments:
/// 
/// * `stdout`: Where we should write the prompt
/// * `input`: Where we should save user input
///
/// # Example usage:
/// ```
/// let mut input = String::new();
/// prompt(&mut stdout, &mut input)?;
/// ```
fn prompt(
	stdout: &mut StandardStream,
	input: &mut String
) -> Result<(), std::io::Error> {

	// Print colored prompt prefix
	stdout.set_color(ColorSpec::new().set_fg(Some(Color::Blue)))?;
	write!(*stdout, "{PROMPT_PREFIX}")?;
	stdout.reset()?;  // reset colors
	stdout.flush()?;  // flush, we didn't print a full line yet.

	// Ask for input
	io::stdin().read_line(input)?;

	// If this input doesn't end with a newline,
	// the user terminated this prompt with ctrl-d.
	// Add a newline to keep spacing consistent,
	// and clear the input.
	if match input.chars().last() {
			Some(val) => val != '\n',
			None => true
	} {
		write!(*stdout, "\n")?;
		input.clear();
	} else {
		(*input) = input.trim().to_string();
	}

	Ok(())
}


fn main() -> Result<(), std::io::Error> {

	let mut stdout = StandardStream::stdout(ColorChoice::Always);

	let term = Arc::new(AtomicBool::new(false));
	signal_hook::flag::register(signal_hook::consts::SIGINT, Arc::clone(&term))?;
	while !term.load(Ordering::Relaxed) {
		let mut input = String::with_capacity(64);
		prompt(&mut stdout, &mut input).expect("Could not show prompt");
		let input = input;

		// Ignore empty input
		if input == "" {
			stdout.flush()?;
			continue;
		}

		// Parse input.
		// Fail if we encounter invalid characters.
		let g: Token = match parser::parse(&input) {
			Ok(g) => g,
			Err((l, e)) => {
				let LineLocation{pos, len} = l;
	
				let s = " ";
				let m = "^";
				println!("{}{} {:?}", s.repeat(pos + 4), m.repeat(len), e);
				stdout.flush()?;
				continue;
			}
		};

		stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
		write!(stdout, "\n  => ")?;
		stdout.reset()?;
		write!(stdout, "Got {input}\n\n\n")?;
	
		writeln!(stdout, "Tokenized: {g:#?}")?;
	}

	writeln!(stdout, "Exiting.")?;
	Ok(())
}