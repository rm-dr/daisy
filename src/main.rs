use std::io;
use std::io::Write;
//use std::io::Read;
use std::sync::Arc;
use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, Ordering};

use termcolor::{
	Color,
	ColorChoice,
	ColorSpec,
	StandardStream,
	WriteColor
};

pub mod tokenize;

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


fn treefold(
	mut exp: tokenize::Token, // Must be a group
	check: fn(&tokenize::Token) -> bool,
	op_type: u8,
	new_token: fn(VecDeque<tokenize::Token>) -> tokenize::Token,
) -> Result<tokenize::Token, ()> {

	// Groups to process
	let mut t_vec: VecDeque<&mut tokenize::Token> = VecDeque::with_capacity(32);
	t_vec.push_back(&mut exp);

	let mut out: Option<tokenize::Token> = None;

	while t_vec.len() > 0 {

		// The group we're currently working with
		let g: &mut tokenize::Token = t_vec.pop_front().unwrap();
		let g_inner: &mut VecDeque<tokenize::Token> = match g {
			tokenize::Token::Group(ref mut x) => x,
			_ => panic!()
		};

		let mut new: VecDeque<tokenize::Token> = VecDeque::with_capacity(8);

		// Build new group array
		while g_inner.len() > 0 {
			let t: tokenize::Token = match g_inner.pop_front() {
				Some(o) => o,
				None => break
			};

			if check(&t) {
				match op_type {
					0 => {},
					1 => {},
					2 => {
						let last: tokenize::Token = new.pop_back().unwrap();
						let next: tokenize::Token = g_inner.pop_front().unwrap().clone();

						let mut new_token_args: VecDeque<tokenize::Token> = VecDeque::with_capacity(2);
						new_token_args.push_back(last);
						new_token_args.push_back(next);
						new.push_back(new_token(new_token_args));
					},
					_ => panic!()
				};
			} else {
				new.push_back(t.clone());
			}
		}

		*g_inner = new;
	}
	return Ok(exp);
}


fn is_mult(t: &tokenize::Token) -> bool {
	match t {
		tokenize::Token::Operator(s) => {s == "*"},
		_ => false
	}
}

fn new_mult(v: VecDeque<tokenize::Token>) -> tokenize::Token {
	tokenize::Token::Mult(v)
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

		// Tokenize input.
		// Fail if we encounter invalid characters.
		let exp = match tokenize::tokenize(&input) {
			Ok(v) => v,
			Err(_) => {
				continue;
			}
		};


		stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
		write!(stdout, "\n  => ")?;
		stdout.reset()?;
		write!(stdout, "Got {input}\n\n\n")?;

		//writeln!(stdout, "Tokenized: {exp:#?}")?;


		let q = treefold(
			exp,
			is_mult,
			2,
			new_mult
		);
		writeln!(stdout, "{q:#?}")?;


		/*
		// Groups to process
		let mut t_vec: VecDeque<tokenize::Token> = VecDeque::with_capacity(32);
		t_vec.push_back(exp);
		
		while t_vec.len() > 0 {
			let g: tokenize::Token = t_vec.pop_front().unwrap();
			let mut g_inner: Vec<tokenize::Token> = match g {
				tokenize::Token::Group(x) => x,
				_ => panic!()
			};

			let mut new: Vec<tokenize::Token> = Vec::with_capacity(8);

			// Parse binary operators
			for o in ["*", "/", "+", "-"] {
				let mut i = g_inner.iter();
				loop {
					let t = match i.next() {
						Some(o) => o,
						None => break
					};
		
					match t {
						tokenize::Token::Operator(s) => {
							if s == o {
								let last = new.pop().unwrap();
								let next = i.next().unwrap();
	
								new.push(tokenize::Token::Op(
									String::from(s),
									Box::new(last.clone()),
									Box::new(next.clone())
								))
							} else {
								new.push(t.clone());
							}
						},
						_ => {
							new.push(t.clone());
						}
					}
				}
				g_inner = new.clone();
				new = Vec::with_capacity(8);
			}
			writeln!(stdout, "{:?}", g_inner)?;
		}
		*/
	}

	writeln!(stdout, "Exiting.")?;
	Ok(())
}