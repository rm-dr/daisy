use crate::formattedtext::FormattedText;

#[derive(Debug)]
pub enum DaisyError {

	// Parser errors
	//MissingCloseParen,
	ExtraCloseParen,
	EmptyGroup,
	Syntax,
	BadNumber,

	// Evaluation errors
	BadMath,
	TooBig,
	ZeroDivision,
	IncompatibleUnit,
	IncompatibleUnits(String, String),
	Undefined(String),
	EvaluationError,
	BadArguments(String, usize, usize)
}

impl DaisyError {
	pub fn text(&self) -> FormattedText {
		match self {
			//DaisyError::MissingCloseParen => {
			//	String::from("Missing close parenthesis")
			//},
			DaisyError::ExtraCloseParen => {
				return FormattedText::new(
					"[e]Syntax Error:[n] Extra close parenthesis".to_string()
				);
			},
			DaisyError::EmptyGroup => {
				return FormattedText::new(
					"[e]Syntax Error:[n] Groups can't be empty".to_string()
				);
			},
			DaisyError::Syntax => {
				return FormattedText::new(
					"[e]Syntax Error[n]".to_string()
				);
			},
			DaisyError::BadNumber => {
				return FormattedText::new(
					"[e]Syntax Error:[n] Invalid number".to_string()
				);
			}

			DaisyError::BadMath => {
				return FormattedText::new(
					"[e]Evaluation Error:[n] Failed to evaluate expression".to_string()
				);
			},
			DaisyError::TooBig => {
				return FormattedText::new(
					"[e]Evaluation Error:[n] Number too big".to_string()
				);

			},
			DaisyError::ZeroDivision => {
				return FormattedText::new(
					"[e]Evaluation Error:[n] Division by zero".to_string()
				);
			},
			DaisyError::IncompatibleUnit => {
				return FormattedText::new(
					"[e]Evaluation Error:[n] Incompatible unit".to_string()
				);
			},
			DaisyError::IncompatibleUnits(a, b) => {
				return FormattedText::new(format!(
					"[e]Evaluation Error:[n] Incompatible units ([c]{a}[n] and [c]{b}[n])"
				));
			},
			DaisyError::Undefined(s) => {
				return FormattedText::new(format!(
					"[e]Evaluation Error:[n] [c]{s}[n] is not defined"
				));
			},
			DaisyError::EvaluationError => {
				return FormattedText::new(
					"[e]Evaluation Error:[n] Could not evaluate".to_string()
				);
			},
			DaisyError::BadArguments(s, want, got) => {
				return FormattedText::new(format!(
					"[e]Evaluation Error:[n] [c]{s}[n] takes {want} argument{}, got {got}",
					if *want == 1 {""} else {"s"},
				));
			}
		}
	}
}
