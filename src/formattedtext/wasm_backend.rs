use super::FormattedText;

impl FormattedText {
	pub fn newline() -> Result<(), ()> {
		print!("\n");
		return Ok(());
	}

	pub fn write(&self) -> String {
		return self.text.clone();
	}
}