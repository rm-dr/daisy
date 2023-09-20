use std::collections::VecDeque;
use std::io::Write;
use termion::raw::RawTerminal;
use daisycalc::FormattedText;
use daisycalc::parser::substitute_cursor;
use daisycalc::Context;

const PROMPT_STR: &str = "==> ";

#[derive(Debug)]
pub struct PromptBuffer {
	// History
	hist: VecDeque<String>,
	hist_maxlen: usize,

	// Counts from back of hist.
	// 0 means "not on history",
	// 1 means "on last item of history"
	hist_cursor: usize,

	buffer: String,
	buffer_changed: bool,
	cursor: usize,
	last_print_len: usize
}

impl PromptBuffer {
	pub fn new(maxlen: usize) -> PromptBuffer {
		return PromptBuffer {
			hist: VecDeque::with_capacity(maxlen/2),
			hist_maxlen: maxlen,
			hist_cursor: 0,
			buffer: String::with_capacity(64),
			buffer_changed: false,
			cursor: 0,
			last_print_len: 0,
		};
	}

	// Same as write_primpt, but pretends there is no cursor
	pub fn write_prompt_nocursor(&mut self, context: &Context, stdout: &mut RawTerminal<std::io::Stdout>) -> Result<(), std::io::Error> {
		let tmp = self.cursor;
		self.cursor = 0;
		let r = self.write_prompt(context, stdout);
		self.cursor = tmp;
		return r;
	}

	pub fn write_prompt(&mut self, context: &Context, stdout: &mut RawTerminal<std::io::Stdout>) -> Result<(), std::io::Error> {
		let l = self.buffer.chars().count();
		let i = if l == 0 {0} else {l - self.cursor};

		// Draw prettyprinted expression
		let (display_c, s) = substitute_cursor(context, &self.get_contents(), i);
	
		write!(
			stdout, "\r{}{PROMPT_STR}{}{}",
			FormattedText::format_map('p', context).unwrap(),
			FormattedText::format_map('n', context).unwrap(),
			s
		)?;

		// If this string is shorter, clear the remaining old one.
		if s.chars().count() < self.last_print_len {
			write!(
				stdout, "{}",
				" ".repeat(self.last_print_len - s.chars().count()),
			)?;
		}

		write!(
			stdout, "\r{}",
			termion::cursor::Right((display_c + PROMPT_STR.chars().count()) as u16)
		)?;

		stdout.flush()?;
		self.last_print_len = s.chars().count();

		return Ok(());
	}

	// Prompt methods
	pub fn get_contents(&self) -> &String {&self.buffer}

	pub fn enter(&mut self) -> String {
		// Don't trim input string so that linelocations are correct
		//let s = String::from(self.buffer.trim());
		let s = self.buffer.clone();
		self.buffer.clear();
		self.hist_cursor = 0;
		self.cursor = 0;
		self.buffer_changed = false;

		if s != "" { self.hist.push_back(s.clone()); }
		while self.hist.len() > self.hist_maxlen {
			self.hist.pop_front();
		}

		return s;
	}

	// Buffer manipulation
	pub fn add_char(&mut self, c: char) {
		self.buffer_changed = true;

		if self.cursor == 0 {
			self.buffer.push(c);
		} else {
			let l = self.buffer.chars().count();
			let i = l - self.cursor;
			self.buffer.insert(i, c);
		}
	}
	pub fn backspace(&mut self) {
		if self.buffer.len() == 0 { return }
		self.buffer_changed = true;
		let l = self.buffer.chars().count();

		if self.cursor == 0 {
			self.buffer.pop();
		} else if self.cursor != l {
			let i = l - self.cursor;
			self.buffer.remove(i-1);

			if self.cursor >= l {
				self.cursor = l-1;
			}
		}
	}

	pub fn delete(&mut self) {
		if self.cursor != 0 {
			self.cursor -= 1;
			self.backspace();
		}
	}


	// Cursor manipulation
	pub fn cursor_left(&mut self) {
		let l = self.buffer.chars().count();
		if self.cursor < l {
			self.cursor += 1;
		}
	}

	pub fn cursor_right(&mut self) {
		if self.cursor > 0 {
			self.cursor -= 1;
		}
	}

	// History manipulation
	pub fn hist_up(&mut self) {
		if self.buffer_changed && self.buffer.len() != 0 { return; }

		if self.hist_cursor < self.hist.len() {
			if self.buffer.len() != 0 || !self.buffer_changed {
				self.hist_cursor += 1;
			}

			self.buffer_changed = false;
			if self.hist_cursor == 0 {
				self.buffer.clear();
			} else {
				self.buffer = self.hist[self.hist.len() - self.hist_cursor].clone();
			}
		}
	}
	pub fn hist_down(&mut self) {
		if self.buffer_changed && self.buffer.len() != 0 { return; }

		if self.hist_cursor > 0 {
			self.hist_cursor -= 1;

			self.buffer_changed = false;
			if self.hist_cursor == 0 {
				self.buffer.clear();
			} else {
				self.buffer = self.hist[self.hist.len() - self.hist_cursor].clone();
			}
		} else {
			self.buffer.clear();
		}
	}

}