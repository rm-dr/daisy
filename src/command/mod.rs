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
		| "flags"
		=> true,
		_ => false
	}
}

#[inline(always)]
fn greeter() -> FormattedText {
	return FormattedText::new(
		format!(
			concat!(
				"[a]              ###### [n] @@@@@@\n",
				"[a]             #     ##[n]@@     @\n",
				"[a]             ##     #[n]@     @@\n",
				"[a]               [n]@@@@@@@@@@@@@[a]\n",
				"[n]             @@     @[a]#     ##\n",
				"[n]             @     @@[a]##     #\n",
				"[n]              @@@@@@ [a] ###### [n]\n",
				"               [t]Daisy[n]  [i]v{ver}[n]\n",
				"\n"
			),
			ver = env!("CARGO_PKG_VERSION")
		)
	);
}


#[inline(always)]
pub fn do_command(
	context: &mut Context,
	s: &String,
) -> FormattedText {
	let args: Vec<&str> = s.split(" ").collect();
	let first = args[0];

	match first {
		"help" => {
			let mut t = greeter();

			t.push(
				concat!(
					"Daisy is a pretty, general-purpose\n",
					"TUI scientific calculator.\n",
					"\n",
					" - Use Up/Down arrows to navigate history.\n",
					" - Use Ctrl-C or Ctrl-D to quit.\n",
					" - Use [c]ans[n] to reference the last result.\n",
					" - Use [c]var = 1337[n] to define varibles.\n",
					"\n",
					"╞═══════════════ [t]Commands[n] ═══════════════╡\n",
					"      [c]help[n]   Show this help\n",
					"      [c]flags[n]  Show command-line options\n",
					"      [c]clear[n]  Clear the terminal\n",
					"      [c]quit[n]   Exit daisy\n",
					//"      [c]units[n]  List available units\n",
					"      [c]consts[n] List built-in constants\n",
					"      [c]ops[n]    List built-in operators\n",
					"      [c]fns[n]    List built-in functions\n",
					"      [c]vars[n]   List user-defined variables\n",
					"      [c]del[n]    Delete a variable\n",
					"\n\n",
				)
			);

			return t;
		},

		"flags" => {
			return FormattedText::new(
				concat!(
					"\n",
					"A list of command-line arguments is below\n",
					"\n",
					"╞════ [t]Flag[n] ════╪════════════════ [t]Function[n] ════════════════╡\n",
					"  [c]--help[n]        Show help\n",
					"  [c]--version[n]     Show version\n",
					"  [c]--info[n]        Show system information\n",
					"  [c]--256color[n]    Use full color support (default)\n",
					"  [c]--8color[n]      Use reduced colors (ANSI, no styling)\n",
					"  [c]--nocolor[n]     Do not use colors and styling\n",
					"  [c]--nosub[n]       Disable inline substitution\n",
					"  [c]--nosuper[n]     Disable superscript powers\n",
					"  [c]--nooneover[n]   Disable \"one-over\" fractions as -1 power\n",
					"\n\n"
				).to_string()
			);
		},

		"clear" => {
			return FormattedText::new("[clear]".to_string());
		},

		"ops" | "operators" => {
			return FormattedText::new(
				concat!(
					"\n",
					"Operators, sorted by priority (high to low).\n",
					"High-piority operators are applied first.\n\n",
					"╞═════ [t]Operator[n] ═════╪═════ [t]Syntax[n] ═════╡\n",
					"  function             [c]sin, cos, etc[n]\n",
					"  factorial            [c]![n]\n",
					"  powers               [c]^, **[n]\n",
					"  implicit multiply    [c]3π, 3(2+1), etc[n]\n",
					"  square root          [c]sqrt, rt, √[n]\n",
					"  negate               [c]-3, -(1 + 2)[n]\n",
					"  modulo (short)       [c]%[n]\n",
					"  multiply, divide     [c]*, /, ×, ÷[n]\n",
					"  add, subtract        [c]+, -[n]\n",
					"  unit conversion      [c]to[n]\n",
					"  division (long)      [c]per[n]\n",
					"  modulo (long)        [c]mod[n]\n",
					"\n\n"
				).to_string()
			);
		},

		"fns" | "functions" => {
			return FormattedText::new(
				concat!(
					"\n╞═══════ [t]Function[n] ═══════╪══════ [t]Syntax[n] ══════╡\n",
					"  absolute value           [c]abs[n]\n",
					"  floor, ceiling, round    [c]floor, ceil, round[n]\n",
					"  log base e               [c]ln[n]\n",
					"  log base 10              [c]log[n]\n",
					"  sin, arcsin, cosecant    [c]sin, asin, csc[n]\n",
					"  cos, arccos, secant      [c]cos, acos, secant[n]\n",
					"  tan, arctan, cotan       [c]tan, atan, cot[n]\n",
					"  hyperbolic sin, etc      [c]sinh, asinh, csch[n]\n",
					"  hyperbolic cos, etc      [c]cosh, acosh, sech[n]\n",
					"  hyperbolic tan, etc      [c]tanh, atanh, coth[n]\n",
					"\n",
					"  Celsius to Kelvin        [c]fromC, fromCelsius[n]\n",
					"  Kelvin to Celsius        [c]toC,   toCelsius[n]\n",
					"  Fahrenheit to Kelvin     [c]fromF, fromFahrenheit[n]\n",
					"  Kelvin to Fahrenheit     [c]toF,   toFahrenheit[n]\n",
					"\n",
					"  Celsius to Fahrenheit    [c]CtoF[n]\n",
					"  Fahrenheit to Celsius    [c]FtoC[n]\n",
					"\n",
					"  convert to base unit     [c]tobase[n]\n",
					"  remove units             [c]nounit[n]\n",
					"\n\n"
				).to_string()
			);
		},

		"vars" => {
			let v = context.get_variables();
			let f = context.get_functions();

			if v.len() + f.len() == 0 {
				return FormattedText::new(
					"You have not defined any variables\n\n".to_string()
				);
			}

			let mut t = FormattedText::new("".to_string());

			let mut longest = 0;
			for (key, _) in v {
				if key.len() > longest {
					longest = key.len();
				}
			}
			for (key, (args, _exp)) in f {
				let s = format!("{key}({})", args.join(", "));
				if s.len() > longest {
					longest = s.len();
				}
			}


			if v.len() != 0 {
				t.push("\n╞═══ [t]User-Defined Variables[n] ═══╡\n");

				for (key, value) in v {
					let padding = " ".repeat(longest - key.len());

					t.push(&format!(
						"  {key}{padding} = [c]{v}[n]\n",
						v = value.display(context),
					));
				}
			}

			if f.len() != 0 {
				t.push("\n╞═══ [t]User-Defined Functions[n] ═══╡\n");

				for (key, (args, exp)) in f {
					let s = format!("{key}({})", args.join(", "));
					let padding = " ".repeat(longest - s.len());

					t.push(&format!(
						"  {s}{padding} = [c]{v}[n]\n",
						v = exp.display(context),
					));
				}
			}

			t.push("\n\n");
			return t;
		},

		"consts" | "constants" => {
			let a = Constant::all_consts();

			let mut t = FormattedText::new(
				"\n╞═══ [t]Built-in Constants[n] ═══╡\n".to_string()
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
						"[c]{first}[n] [e]takes exactly one argument.[n]\n\n",
					)
				);
			}

			let v = args[1].to_string();
			let v = substitute(context, &v);
			let r = context.delete(&v);

			return match r {
				Ok(()) => { FormattedText::new("".to_string()) },
				Err(()) => {
					FormattedText::new(
						format!(
							"[c]{v}[n] [e]isn't a variable.[n]\n\n",
						)
					)
				}
			};
		},

		_ => unreachable!("Bad command!")
	};
}
