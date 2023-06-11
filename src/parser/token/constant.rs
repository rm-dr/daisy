#[derive(Debug)]
#[derive(Clone)]
pub enum Constant {
	// Fake units
	MPG,
	MPH,

	// Mathematics
	Pi,
	Phi,
	E,
}

impl Constant {
	pub fn to_string(&self) -> String {
		match self {
			// Fake units
			Constant::MPG => { String::from("mpg") },
			Constant::MPH => { String::from("mph") },

			// Mathematics
			Constant::Pi => { String::from("π") },
			Constant::Phi => { String::from("φ") },
			Constant::E => { String::from("e") }
		}
	}
}