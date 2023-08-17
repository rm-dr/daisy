use crate::parser::{Expression, Function, Constant};
use crate::quantity::freeunit_from_string;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Context {
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

	pub fn is_varible(&self, s: &str) -> bool {
		return {
			self.valid_varible(s) &&
			(self.variables.contains_key(s) || self.shadow.contains_key(s))
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