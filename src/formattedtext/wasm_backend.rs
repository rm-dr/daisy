use super::FormattedText;

fn format_map(s: &str) -> Option<String> {
	Some(match s {
		"n" => {"\x1B[0m"},
		"i" => {"\x1B[3m"},
		"t" => {"\x1B[1;35m"},
		"a" => {"\x1B[0;35m"},
		"e" => {"\x1B[1;31m"},
		"c" => {"\x1B[3;90m"},
		"p" => {"\x1B[1;34m"},
		"s" => {"\x1B[1;35m"},
		"r" => {"\x1B[1;32m"},
		_ => { return None }
	}.to_string())
}


impl FormattedText {
	pub fn write(&self) -> String {

		let mut word = String::new();
		let mut reading = false; // are we reading a word?
		let mut chars = self.text.chars();
		let mut out = String::new();

		while let Some(c) = chars.next() {

			match c {
				'[' => {
					if reading {
						// Discard old word, start reading again.
						out.push_str(&word);
						word.clear();
					} 
					
					// Start reading a new word
					reading = true;
					word.push(c);
				},

				']' => {
					if !reading {
						out.push(c);
					} else {
						word.push(c);

						let f = format_map(&word[1..word.len()-1]);

						if f.is_some() {
							out.push_str(&f.unwrap());
						} else if word == "[clear]" {
							out.push_str(&format!("\x1B[2J\x1B[H"));
						} else if word.starts_with("[cursorright") {
							let n: u16 = word[12..word.len()-1].parse().unwrap();
							out.push_str(&format!("\x1B[{n}C"));
						} else {
							out.push_str(&word);
						}

						reading = false;
						word.clear();
					}
				},

				'\n' => {
					if reading { word.push_str("\r\n"); }
					else { out.push_str("\r\n"); }
				},

				_ => {
					if reading { word.push(c); }
					else { out.push(c); }
				}
			}
		}

		return out;
	}
}