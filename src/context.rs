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
	pub fn push_var(&mut self, s: String, t: Expression) { self.variables.insert(s, t); }
	pub fn del_var(&mut self, s: &String) { self.variables.remove(s); }

	pub fn get_variable(&self, s: &String) -> Option<Expression> {

		let v: Option<&Expression>;

		if s == "ans" {
			v = self.history.last();
		} else {
			v = self.variables.get(s);
		}

		if v.is_some() { Some(v.unwrap().clone()) } else { None }
	}
}
