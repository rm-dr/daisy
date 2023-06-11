use crate::parser::Token;
use crate::parser::Constant;
use crate::quantity::Quantity;

use super::EvalError;

pub fn eval_constant(c: &Constant) -> Result<Token, EvalError> {
	Ok(match c {
		// Mathematical constants
		// 100 digits of each.
		Constant::Pi => { Token::Quantity(Quantity::new_float_from_string(
			"3.141592653589793238462643383279502884197169399375105820974944592307816406286208998628034825342117067"
		).unwrap())},

		Constant::E => { Token::Quantity(Quantity::new_float_from_string(
			"2.718281828459045235360287471352662497757247093699959574966967627724076630353547594571382178525166427"
		).unwrap()) },

		Constant::Phi => { Token::Quantity(Quantity::new_float_from_string(
			"1.618033988749894848204586834365638117720309179805762862135448622705260462818902449707207204189391137"
		).unwrap()) },
	})
}