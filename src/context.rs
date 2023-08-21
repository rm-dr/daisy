use crate::parser::{Expression, Function, Constant};
use crate::quantity::freeunit_from_string;
use std::collections::HashMap;

#[derive(Debug)]
#[derive(Clone)]
pub struct Config {

	// How to color terminal text.
	// 0: No colors
	// 1: ANSI-compatible, 8 colors
	// 2: Full 256 color and special styles
	pub term_color_type: u8,

	// Should we accept input and print in unicode?
	//pub enable_unicode: bool,

	// Should we replace certain strings (like "pi")
	// with prettier unicode alternatives?
	//
	// Automatically disabled if enable_unicode is off.
	pub enable_substituion: bool,

	// Should we print simple powers
	// as unicode superscript chars?
	//
	// Automatically disables if enable_unicode is off.
	pub enable_super_powers: bool,

	// Should we write "one-over" fractions
	// as -1 powers?
	//
	// Automatically disabled if enable_super_powers is off.
	pub enable_one_over_power: bool,
}

impl Config {
	pub fn new() -> Config {
		Config{
			term_color_type: 2,
			enable_substituion: true,
			//enable_unicode: true,
			enable_super_powers: true,
			enable_one_over_power: true
		}
	}

	pub fn check(&mut self) {
		//if !self.enable_unicode {
		//	self.enable_substituion = false;
		//	self.enable_super_powers = false;
		//}

		if !self.enable_super_powers {
			self.enable_one_over_power = false
		}
	}
}




#[derive(Debug)]
#[derive(Clone)]
pub struct Context {
	pub config: Config,

	history: Vec<Expression>,
	variables: HashMap<String, Expression>,
	functions: HashMap<String, (Vec<String>, Expression)>,

	// Shadow variables, for function evaluation.
	shadow: HashMap<String, Option<Expression>>
}

// General functions
impl Context {
	pub fn new() -> Context {
		Context{
			config: Config::new(),
			history: Vec::new(),
			variables: HashMap::new(),
			functions: HashMap::new(),
			shadow: HashMap::new(),
		}
	}

	pub fn push_hist(&mut self, t: Expression) { self.history.push(t); }


	pub fn delete(&mut self, s: &String) -> Result<(), ()> {
		if !(self.is_varible(s) || self.is_function(s)) { return Err(()) };
		if self.is_varible(s) { self.variables.remove(s); }
		if self.is_function(s) { self.functions.remove(s); }
		return Ok(());
	}
}



// Variable manipulation
impl Context {
	pub fn push_variable(&mut self, s: String, t: Expression) -> Result<(), ()> {
		if self.valid_varible(&s) {
			self.functions.remove(&s);
			self.variables.insert(s, t);
			return Ok(());
		} else { return Err(()); }
	}

	// Returns None if this is a "floating" variable
	pub fn get_variable(&self, s: &String) -> Option<Expression> {
		if self.shadow.contains_key(s) {
			return self.shadow.get(s).unwrap().clone();
		}

		let v: Option<&Expression>;
		if s == "ans" {
			v = self.history.last();
		} else {
			v = self.variables.get(s);
		}

		if v.is_some() {
			return Some(v.unwrap().clone());
		} else { panic!() }
	}

	// Can we define a new variable with this name?
	pub fn valid_varible(&self, s: &str) -> bool {
		if {
			Function::from_string(s).is_some() ||
			Constant::from_string(s).is_some() ||
			freeunit_from_string(s).is_some()
		} { return false }

		for c in s.to_lowercase().chars() {
			if !"abcdefghijklmnopqrtstuvwxyz_".contains(c) {
				return false;
			}
		}

		return match s {
			"ans" => false,
			_ => true
		}
	}

	// Can we get a value fro mthis variable name?
	pub fn is_varible(&self, s: &str) -> bool {
		return {
			(
				s == "ans" &&
				self.history.len() != 0
			) ||
			(
				self.valid_varible(s) &&
				(self.variables.contains_key(s) || self.shadow.contains_key(s))
			)
		};
	}

	pub fn get_variables(&self) -> &HashMap<String, Expression> {
		return &self.variables
	}

	pub fn add_shadow(&mut self, s: String, v: Option<Expression>) {
		if !self.valid_varible(&s) { panic!() }
		self.shadow.insert(s, v);
	}

	pub fn clear_shadow(&mut self) {
		self.shadow = HashMap::new();
	}

}


// Function manipulation
impl Context {
	pub fn valid_function(&self, s: &str) -> bool {
		return self.valid_varible(s);
	}

	pub fn push_function(&mut self, s: String, a: Vec<String>, t: Expression) -> Result<(), ()> {
		if self.valid_function(&s) {
			self.variables.remove(&s);
			self.functions.insert(s, (a, t));
			return Ok(());
		} else { return Err(()); }
	}

	pub fn get_function(&self, s: &String) -> Option<(Vec<String>, Expression)> {
		return Some(self.functions.get(s).unwrap().clone());
	}

	pub fn is_function(&self, s: &str) -> bool {
		return self.valid_function(s) && self.functions.contains_key(s);
	}
	pub fn get_functions(&self) -> &HashMap<String, (Vec<String>, Expression)> {
		return &self.functions
	}
}