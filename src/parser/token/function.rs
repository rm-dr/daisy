#[derive(Debug)]
#[derive(Copy, Clone)]
pub enum Function {
	Abs,
	Floor,
	Ceil,
	Round,

	NaturalLog,
	TenLog,

	Sin,
	Cos,
	Tan,
	Asin,
	Acos,
	Atan,
	Csc,
	Sec,
	Cot,

	Sinh,
	Cosh,
	Tanh,
	Asinh,
	Acosh,
	Atanh,
	Csch,
	Sech,
	Coth,
}

impl Function {
	pub fn to_string(&self) -> String {
		match self {
			Function::Abs => { String::from("abs") },
			Function::Floor => { String::from("floor") },
			Function::Ceil => { String::from("ceil") },
			Function::Round => { String::from("round") },
			Function::NaturalLog => { String::from("ln") },
			Function::TenLog => { String::from("log") },
			Function::Sin => { String::from("sin") },
			Function::Cos => { String::from("cos") },
			Function::Tan => { String::from("tan") },
			Function::Asin => { String::from("asin") },
			Function::Acos => { String::from("acos") },
			Function::Atan => { String::from("atan") },
			Function::Csc => { String::from("csc") },
			Function::Sec => { String::from("sec") },
			Function::Cot => { String::from("cot") },
			Function::Sinh => { String::from("sinh") },
			Function::Cosh => { String::from("cosh") },
			Function::Tanh => { String::from("tanh") },
			Function::Asinh => { String::from("asinh") },
			Function::Acosh => { String::from("acosh") },
			Function::Atanh => { String::from("atanh") },
			Function::Csch => { String::from("csch") },
			Function::Sech => { String::from("sech") },
			Function::Coth => { String::from("coth") },
		}
	}
}