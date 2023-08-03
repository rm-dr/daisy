use crate::parser::Expression;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Context {
	history: Vec<Expression>,
	variables: HashMap<String, Expression>
}

impl Context {
	pub fn new() -> Context {
		Context{ history: Vec::new(), variables: HashMap::new() }
	}

	pub fn push_hist(&mut self, t: Expression) { self.history.push(t); }
	pub fn push_var(&mut self, s: String, t: Expression) -> Result<(), ()> {
		if self.valid_varible(&s) {
			self.variables.insert(s, t);
			return Ok(());
		} else { return Err(()); }
	}

	pub fn get_variable(&self, s: &String) -> Option<Expression> {
		let v: Option<&Expression>;
		if s == "ans" {
			v = self.history.last();
		} else {
			v = self.variables.get(s);
		}
		if v.is_some() { Some(v.unwrap().clone()) } else { None }
	}

	pub fn delete_variable(&mut self, s: &String) -> Result<(), ()> {
		if !self.is_varible(s) { return Err(()) };
		self.variables.remove(s);
		return Ok(());
	}

	pub fn valid_varible(&self, s: &str) -> bool {

		if {
			Function::from_string(s).is_some() ||
			Constant::from_string(s).is_some()
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
		return self.valid_varible(s) && self.variables.contains_key(s);
	}

	pub fn get_variables(&self) -> &HashMap<String, Expression> {
		return &self.variables
	}

}
