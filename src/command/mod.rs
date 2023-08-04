use crate::context::Context;
use crate::parser::Constant;
use crate::parser::substitute;
use crate::formattedtext::FormattedText;

pub fn is_command(
	s: &String
) -> bool {
	let args: Vec<&str> = s.split(" ").collect();
	let first = args[0];

	match first {
		"help" | "clear"
		| "ops" | "operators"
		| "fns" | "functions"
		| "vars"
		| "consts" | "constants"
		| "del" | "delete"
		=> true,
		_ => false
	}
}

#[inline(always)]
fn greeter() -> FormattedText {
	return FormattedText::new(
		format!(
			concat!(
				"[a]              ###### [n] @@@@@@\r\n",
				"[a]             #     ##[n]@@     @\r\n",
				"[a]             ##     #[n]@     @@\r\n",
				"[a]               [n]@@@@@@@@@@@@@[a]\r\n",
				"[n]             @@     @[a]#     ##\r\n",
				"[n]             @     @@[a]##     #\r\n",
				"[n]              @@@@@@ [a] ###### [n]\r\n",
				"               [t]Daisy[n]  [i]v{ver}[n]\r\n",
				"\n"
			),
			ver = env!("CARGO_PKG_VERSION")
		)
	);
}


#[inline(always)]
pub fn do_command(
	s: &String,
	context: &mut Context
) -> FormattedText {
	let args: Vec<&str> = s.split(" ").collect();
	let first = args[0];

	match first {
		"help" => {
			let mut t = greeter();

			t.push(
				concat!(
					"Daisy is a high-precision, general-purpose\r\n",
					"scientific calculator.\r\n",
					"\n",
					" - Use Up/Down arrows to navigate history.\r\n",
					" - Use Ctrl-C or Ctrl-D to quit.\r\n",
					" - Use [c]ans[n] to reference the last result.\r\n",
					" - Use [c]var = 1337[n] to define varibles.\r\n",
					"\n",
					"╞═══════════════ [t]Commands[n] ═══════════════╡\r\n",
					"      [c]help[n]   Show this help\r\n",
					"      [c]clear[n]  Clear the terminal\r\n",
					"      [c]quit[n]   Exit daisy\r\n",
					//"      [c]units[n]  List available units\r\n",
					"      [c]consts[n] List built-in constants\r\n",
					"      [c]ops[n]    List built-in operators\r\n",
					"      [c]fns[n]    List built-in functions\r\n",
					"      [c]vars[n]   List user-defined variables\r\n",
					"      [c]del[n]    Delete a variable\r\n",
					"\n\n",
				)
			);

			return t;
		},

		"clear" => {
			return FormattedText::new("[clear]".to_string());
		},

		"ops" | "operators" => {
			return FormattedText::new(
				concat!(
					"\r\n",
					"Operators, sorted by priority (high to low).\r\n",
					"High-piority operators are applied first.\r\n\n",
					"╞═════ [t]Operator[n] ═════╪═════ [t]Syntax[n] ═════╡\r\n",
					"  function             [c]sin, cos, etc[n]\r\n",
					"  factorial            [c]![n]\r\n",
					"  powers               [c]^, **[n]\r\n",
					"  implicit multiply    [c]3π, 3(2+1), etc[n]\r\n",
					"  square root          [c]sqrt, rt, √[n]\r\n",
					"  negate               [c]-3, -(1 + 2)[n]\r\n",
					"  modulo (short)       [c]%[n]\r\n",
					"  multiply, divide     [c]*, /, ×, ÷[n]\r\n",
					"  add, subtract        [c]+, -[n]\r\n",
					"  unit conversion      [c]to[n]\r\n",
					"  division (long)      [c]per[n]\r\n",
					"  modulo (long)        [c]mod[n]\r\n",
					"\n\n"
				).to_string()
			);
		},

		"fns" | "functions" => {
			return FormattedText::new(
				concat!(
					"\r\n╞═══════ [t]Function[n] ═══════╪══════ [t]Syntax[n] ══════╡\r\n",
					"  absolute value           [c]abs[n]\r\n",
					"  floor, ceiling, round    [c]floor, ceil, round[n]\r\n",
					"  log base e               [c]ln[n]\r\n",
					"  log base 10              [c]log[n]\r\n",
					"  sin, arcsin, cosecant    [c]sin, asin, csc[n]\r\n",
					"  cos, arccos, secant      [c]cos, acos, secant[n]\r\n",
					"  tan, arctan, cotan       [c]tan, atan, cot[n]\r\n",
					"  hyperbolic sin, etc      [c]sinh, asinh, csch[n]\r\n",
					"  hyperbolic cos, etc      [c]cosh, acosh, sech[n]\r\n",
					"  hyperbolic tan, etc      [c]tanh, atanh, coth[n]\r\n",
					"\n",
					"  Celsius to Kelvin        [c]fromC, fromCelsius[n]\r\n",
					"  Kelvin to Celsius        [c]toC,   toCelsius[n]\r\n",
					"  Fahrenheit to Kelvin     [c]fromF, fromFahrenheit[n]\r\n",
					"  Kelvin to Fahrenheit     [c]toF,   toFahrenheit[n]\r\n",
					"\n",
					"  convert to base unit     [c]tobase[n]\r\n",
					"  remove units             [c]nounit[n]\r\n",
					"\n\n"
				).to_string()
			);
		},

		"vars" => {
			let v = context.get_variables();

			if v.len() == 0 {
				return FormattedText::new(
					"You have not defined any variables\r\n\n".to_string()
				);
			}

			let mut t = FormattedText::new(
				"\r\n╞═══ [t]User-Defined Variables[n] ═══╡\r\n".to_string()
			);

			let mut longest = 0;
			for (key, _) in v {
				if key.len() > longest {
					longest = key.len();
				}
			}

			for (key, value) in v {
				let padding = " ".repeat(longest - key.len());

				t.push(&format!(
					"  {key}{padding} = [c]{v}[n]\r\n",
					v = value.to_string(),
				));
			}

			t.push("\r\n\n");
			return t;
		},

		"consts" | "constants" => {
			let a = Constant::all_consts();

			let mut t = FormattedText::new(
				"\r\n╞═══ [t]Built-in Constants[n] ═══╡\r\n".to_string()
			);


			for c in a {
				let Some(p) = c.pretty_name() else { continue };

				// If you subtract with overflow here,
				// your padding length is too short.
				let padding = " ".repeat(25 - p.chars().count());

				t.push(&format!(
					"  {p}{padding}: [c]{s}[n]",
					s = c.source_strings().join(", "),
				));

				t.push(&"\n");
			}

			t.push(&"\n\n");
			return t;
		},

		"del" | "delete" => {
			if args.len() != 2 {
				return FormattedText::new(
					format!(
						"[c]{first}[n] [t]takes exactly two arguments.[n]\r\n\n",
					)
				);
			}

			let v = args[1].to_string();
			let v = substitute(&v);
			let r = context.delete_variable(&v);

			return match r {
				Ok(()) => { FormattedText::new("".to_string()) },
				Err(()) => {
					FormattedText::new(
						format!(
							"[c]{v}[n] [t]isn't a variable.[n]\r\n\n",
						)
					)
				}
			};
		},

		_ => unreachable!("Bad command!")
	};
}
