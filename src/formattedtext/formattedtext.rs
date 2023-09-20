use std::ops::Add;


#[derive(Debug)]
#[derive(Clone)]
pub struct FormattedText {
	pub(super) text: String
}

impl ToString for FormattedText {
	fn to_string(&self) -> String { return self.text.clone(); }
}

impl FormattedText {
	pub fn new(s: String) -> FormattedText {
		return FormattedText {
			text: s
		}
	}

	pub fn push(&mut self, s: &str) {
		self.text.push_str(s);
	}
}


impl Add for FormattedText {
	type Output = Self;

	fn add(self, other: Self) -> Self::Output {
		return FormattedText::new(format!("{}{}", self.text, other.text));
	}
}