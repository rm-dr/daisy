use std::collections::VecDeque;


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
	//cursor: usize // Counts from back of buffer
}

impl PromptBuffer {
	pub fn new(maxlen: usize) -> PromptBuffer {
		return PromptBuffer {
			hist: VecDeque::with_capacity(maxlen/2),
			hist_maxlen: maxlen,
			hist_cursor: 0,
			buffer: String::with_capacity(64),
			buffer_changed: false
			//cursor: 0,
		};
	}


	// Prompt methods
	pub fn get_contents(&self) -> &String {&self.buffer}

	pub fn enter(&mut self) -> String{
		let s = String::from(self.buffer.trim());
		self.buffer.clear();
		self.hist_cursor = 0;
		self.buffer_changed = false;

		if s != "" { self.hist.push_back(s.clone()); }
		while self.hist.len() > self.hist_maxlen {
			self.hist.pop_front();
		}

		return s;
	}

	// Buffer manipulation
	pub fn add_char(&mut self, c: char) {
		self.buffer.push(c);
		self.buffer_changed = true;
	}
	pub fn backspace(&mut self) {
		if self.buffer.len() != 0 {
			self.buffer_changed = true;
			self.buffer.pop();
		}
	}
	pub fn delete(&mut self) {
		self.backspace();
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