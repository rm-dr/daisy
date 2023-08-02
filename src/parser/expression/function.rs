#[derive(Debug)]
#[derive(Copy, Clone)]
pub enum Function {
	// When adding a function, don't forget to update help command text.
	// It isn't automatically generated.

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

	NoUnit,
	ToBase,
	FromCelsius,
	ToCelsius,
	FromFahrenheit,
	ToFahrenheit
}


impl ToString for Function {
	fn to_string(&self) -> String {
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
			Function::NoUnit => { String::from("nounit") },
			Function::ToBase => { String::from("tobase") },
			Function::FromCelsius => { String::from("fromcelsius") },
			Function::ToCelsius => {String::from("tocelsius") },
			Function::FromFahrenheit => { String::from("fromfahrenheit") },
			Function::ToFahrenheit => { String::from("tofahrenheit") },
		}
	}

}

impl Function {
	#[inline(always)]
	pub fn from_string(s: &str) -> Option<Function> {
		match s {
			"abs"     => {Some(Function::Abs)},
			"floor"   => {Some(Function::Floor)},
			"ceil"    => {Some(Function::Ceil)},
			"round"   => {Some(Function::Round)},
			"ln"      => {Some(Function::NaturalLog)},
			"log"     => {Some(Function::TenLog)},
			"sin"     => {Some(Function::Sin)},
			"cos"     => {Some(Function::Cos)},
			"tan"     => {Some(Function::Tan)},
			"asin"    => {Some(Function::Asin)},
			"acos"    => {Some(Function::Acos)},
			"atan"    => {Some(Function::Atan)},
			"csc"     => {Some(Function::Csc)},
			"secant"  => {Some(Function::Sec)},
			"cot"     => {Some(Function::Cot)},
			"sinh"    => {Some(Function::Sinh)},
			"cosh"    => {Some(Function::Cosh)},
			"tanh"    => {Some(Function::Tanh)},
			"asinh"   => {Some(Function::Asinh)},
			"acosh"   => {Some(Function::Acosh)},
			"atanh"   => {Some(Function::Atanh)},
			"csch"    => {Some(Function::Csch)},
			"sech"    => {Some(Function::Sech)},
			"coth"    => {Some(Function::Coth)},

			"nounit" => {Some(Function::NoUnit)},
			"tobase" => {Some(Function::ToBase)},

			"toC"            => {Some(Function::ToCelsius)},
			"tocelsius"      => {Some(Function::ToCelsius)},
			"toCelsius"      => {Some(Function::ToCelsius)},

			"fromC"          => {Some(Function::FromCelsius)},
			"fromcelsius"    => {Some(Function::FromCelsius)},
			"fromCelsius"    => {Some(Function::FromCelsius)},

			"toFahrenheit"   => {Some(Function::ToFahrenheit)},
			"tofahrenheit"   => {Some(Function::ToFahrenheit)},
			"toF"            => {Some(Function::ToFahrenheit)},

			"fromF"          => {Some(Function::FromFahrenheit)},
			"fromfahrenheit" => {Some(Function::FromFahrenheit)},
			"fromFahrenheit" => {Some(Function::FromFahrenheit)},
			_ => None
		}
	}
}