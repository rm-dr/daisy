use std::cmp;
use std::ops::Add;
use std::ops::AddAssign;
use std::cmp::Ordering;

/// Specifies the location of a token in an input string.
#[derive(Debug)]
#[derive(Copy, Clone)]
pub struct LineLocation {
	pub pos: usize,
	pub len: usize
}

impl LineLocation {
	pub fn zero(&self) -> bool {
		return self.pos == 0 && self.len == 0
	}

	pub fn new_zero() -> LineLocation {
		return LineLocation { pos: 0, len: 0}
	}

	pub fn cut_string<'a>(&self, s: &'a String) -> &'a str {
		&s[self.pos..self.pos+self.len]
	}
}

impl PartialEq for LineLocation {
	fn eq(&self, other: &Self) -> bool {
		self.pos == other.pos &&
		self.len == other.len
	}
}

impl PartialOrd for LineLocation {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		return self.pos.partial_cmp(&other.pos);
	}
}

impl Add for LineLocation {
	type Output = Self;

	fn add(self, other: Self) -> Self::Output {
		if self.zero() { return other; }
		if other.zero() { return self; }

		let start = cmp::min(self.pos, other.pos);
		let end = cmp::max(self.pos+self.len-1, other.pos+other.len-1);
		return LineLocation{
			pos: start,
			len: end - start+1
		};
	}
}

impl AddAssign for LineLocation where {
	fn add_assign(&mut self, other: Self) {
		if self.zero() {*self = other}
		if other.zero() { return }

		let start = cmp::min(self.pos, other.pos);
		let end = cmp::max(self.pos+self.len-1, other.pos+other.len-1);
		self.pos = start;
		self.len = end - start + 1;
	}
}
