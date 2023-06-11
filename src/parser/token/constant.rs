#[derive(Debug)]
#[derive(Clone)]
pub enum Constant {
	Pi,
	Phi,
	E
}

impl Constant {
	pub fn to_string(&self) -> String {
		match self {
			Constant::Pi => { String::from("π") },
			Constant::Phi => { String::from("φ") },
			Constant::E => { String::from("e") },
		}
	}
}