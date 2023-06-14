use crate::parser::Token;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Context {
	history: Vec<Token>,
	variables: HashMap<String, Token>
}

impl Context {
	pub fn new() -> Context {
		Context{ history: Vec::new(), variables: HashMap::new() }
	}

	pub fn push_hist(&mut self, t: Token) { self.history.push(t); }
	pub fn push_var(&mut self, s: String, t: Token) { self.variables.insert(s, t); }
	pub fn del_var(&mut self, s: &String) { self.variables.remove(s); }

	pub fn get_variable(&self, s: &String) -> Option<Token> {

		let v: Option<&Token>;

		if s == "ans" {
			v = self.history.last();
		} else {
			v = self.variables.get(s);
		}

		if v.is_some() { Some(v.unwrap().clone()) } else { None }
	}
}
