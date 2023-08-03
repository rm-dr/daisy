use std::io::Write;
use crate::context::Context;
use crate::parser::Constant;
use crate::parser::substitute;

use termion::{
	raw::RawTerminal,
	color,
	style,
	clear,
	cursor
};

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
fn draw_greeter(stdout: &mut RawTerminal<std::io::Stdout>) -> Result<(), std::io::Error> {
	write!(
		stdout,
		concat!(
			"{a}              ###### {b} @@@@@@\r\n",
			"{a}             #     ##{b}@@     @\r\n",
			"{a}             ##     #{b}@     @@\r\n",
			"{a}               {b}@@@@@@@@@@@@@{a}\r\n",
			"{b}             @@     @{a}#     ##\r\n",
			"{b}             @     @@{a}##     #\r\n",
			"{b}              @@@@@@ {a} ###### {r}\r\n",
			"               {t}Daisy{r}  {v}v{ver}{r}\r\n",
			"\n"
		),
		r = format!("{}{}", color::Fg(color::Reset), style::Reset),
		a = color::Fg(color::Magenta),
		b = color::Fg(color::White),
		t = format!("{}{}", color::Fg(color::White), style::Bold),
		v = format!("{}{}", color::Fg(color::White), style::Italic),
		ver = env!("CARGO_PKG_VERSION"),
	)?;

	return Ok(());
}


#[inline(always)]
pub fn do_command(
	stdout: &mut RawTerminal<std::io::Stdout>,
	s: &String,
	context: &mut Context
) -> Result<(), std::io::Error> {
	let args: Vec<&str> = s.split(" ").collect();
	let first = args[0];

	match first {
		"help" => {
			draw_greeter(stdout)?;

			write!(stdout,
				concat!(
					"Daisy is a high-precision, general-purpose\r\n",
					"scientific calculator.\r\n",
					"\n",
					" - Use Up/Down arrows to navigate history.\r\n",
					" - Use Ctrl-C or Ctrl-D to quit.\r\n",
					" - Use {c}ans{r} to reference the last result.\r\n",
					" - Use {c}var = 1337{r} to define varibles.\r\n",
					"\n",
					"╞═══════════════ {t}Commands{r} ═══════════════╡\r\n",
					"      {c}help{r}   Show this help\r\n",
					"      {c}clear{r}  Clear the terminal\r\n",
					"      {c}quit{r}   Exit daisy\r\n",
					//"      {c}units{r}  List available units\r\n",
					"      {c}consts{r} List built-in constants\r\n",
					"      {c}ops{r}    List built-in operators\r\n",
					"      {c}fns{r}    List built-in functions\r\n",
					"      {c}vars{r}   List user-defined variables\r\n",
					"      {c}del{r}    Delete a variable\r\n",
					"\n\n",
				),

				r = format!("{}{}", color::Fg(color::Reset), style::Reset),
				c = format!("{}{}", color::Fg(color::LightBlack), style::Italic),
				t = format!("{}{}", color::Fg(color::Magenta), style::Bold)
			)?;
		},

		"clear" => {
			write!(
				stdout,
				"{}{}",
				clear::All,
				cursor::Goto(1, 1)
			)?;
		},

		"ops" | "operators" => {
			write!(stdout,
				concat!(
					"\r\n",
					"Operators, sorted by priority (high to low).\r\n",
					"High-piority operators are applied first.\r\n\n",
					"╞═════ {t}Operator{r} ═════╪═════ {t}Syntax{r} ═════╡\r\n",
					"  function             {c}sin, cos, etc{r}\r\n",
					"  factorial            {c}!{r}\r\n",
					"  powers               {c}^, **{r}\r\n",
					"  implicit multiply    {c}3π, 3(2+1), etc{r}\r\n",
					"  square root          {c}sqrt, rt, √{r}\r\n",
					"  negate               {c}-3, -(1 + 2){r}\r\n",
					"  modulo (short)       {c}%{r}\r\n",
					"  multiply, divide     {c}*, /, ×, ÷{r}\r\n",
					"  add, subtract        {c}+, -{r}\r\n",
					"  unit conversion      {c}to{r}\r\n",
					"  division (long)      {c}per{r}\r\n",
					"  modulo (long)        {c}mod{r}\r\n",
					"\n\n"
				),

				r = format!("{}{}", color::Fg(color::Reset), style::Reset),
				c = format!("{}{}", color::Fg(color::LightBlack), style::Italic),
				t = format!("{}{}", color::Fg(color::Magenta), style::Bold)
			)?;
		},

		"fns" | "functions" => {
			write!(stdout,
				concat!(
					"\r\n╞═══════ {t}Function{r} ═══════╪══════ {t}Syntax{r} ══════╡\r\n",
					"  absolute value           {c}abs{r}\r\n",
					"  floor, ceiling, round    {c}floor, ceil, round{r}\r\n",
					"  log base e               {c}ln{r}\r\n",
					"  log base 10              {c}log{r}\r\n",
					"  sin, arcsin, cosecant    {c}sin, asin, csc{r}\r\n",
					"  cos, arccos, secant      {c}cos, acos, secant{r}\r\n",
					"  tan, arctan, cotan       {c}tan, atan, cot{r}\r\n",
					"  hyperbolic sin, etc      {c}sinh, asinh, csch{r}\r\n",
					"  hyperbolic cos, etc      {c}cosh, acosh, sech{r}\r\n",
					"  hyperbolic tan, etc      {c}tanh, atanh, coth{r}\r\n",
					"\n",
					"  Celsius to Kelvin        {c}fromC, fromCelsius{r}\r\n",
					"  Kelvin to Celsius        {c}toC,   toCelsius{r}\r\n",
					"  Fahrenheit to Kelvin     {c}fromF, fromFahrenheit{r}\r\n",
					"  Kelvin to Fahrenheit     {c}toF,   toFahrenheit{r}\r\n",
					"\n",
					"  convert to base unit     {c}tobase{r}\r\n",
					"  remove units             {c}nounit{r}\r\n",
					"\n\n"
				),

				r = format!("{}{}", color::Fg(color::Reset), style::Reset),
				c = format!("{}{}", color::Fg(color::LightBlack), style::Italic),
				t = format!("{}{}", color::Fg(color::Magenta), style::Bold)
			)?;
		},

		"vars" => {
			let v = context.get_variables();

			if v.len() == 0 {
				write!(stdout,
					"You have not defined any variables.\r\n\n",
				)?;
				return Ok(());
			}

			write!(stdout,
				"\r\n╞═══ {t}User-Defined Variables{r} ═══╡\r\n",
				r = format!("{}{}", color::Fg(color::Reset), style::Reset),
				t = format!("{}{}", color::Fg(color::Magenta), style::Bold)
			)?;



			let mut longest = 0;
			for (key, _) in v {
				if key.len() > longest {
					longest = key.len();
				}
			}

			for (key, value) in v {
				let padding = " ".repeat(longest - key.len());

				write!(stdout,
					concat!(
						"  {k}{p} = {c}{v}{r}\r\n",
					),
					k = key, v = value.to_string(),
					p = padding,
					r = format!("{}{}", color::Fg(color::Reset), style::Reset),
					c = format!("{}{}", color::Fg(color::LightBlack), style::Italic),
				)?;
			}

			write!(stdout,
				"\r\n\n",
			)?;
		},

		"consts" | "constants" => {
			let a = Constant::all_consts();

			write!(stdout,
				"\r\n╞═══ {t}Built-in Constants{r} ═══╡\r\n",
				r = format!("{}{}", color::Fg(color::Reset), style::Reset),
				t = format!("{}{}", color::Fg(color::Magenta), style::Bold)
			)?;

			for c in a {
				let Some(p) = c.pretty_name() else { continue };

				// If you subtract with overflow here,
				// your padding length is too short.
				let padding = " ".repeat(25 - p.chars().count());

				write!(stdout,
					"  {n}{p}: {c}{s}{r}",
					p = padding,
					n = p,
					s = c.source_strings().join(", "),

					r = format!("{}{}", color::Fg(color::Reset), style::Reset),
					c = format!("{}{}", color::Fg(color::LightBlack), style::Italic),
				)?;

				write!(stdout, "\r\n")?;
			}

			write!(stdout,
				"\r\n\n",
			)?;
		},

		"del" | "delete" => {
			if args.len() != 2 {
				write!(stdout,
					"{c}{cmd}{r} {t}takes exactly two arguments.{r}\r\n\n",
					cmd = first,
					r = format!("{}{}", color::Fg(color::Reset), style::Reset),
					t = format!("{}{}", color::Fg(color::Red), style::Bold),
					c = format!("{}{}", color::Fg(color::LightBlack), style::Italic)
				)?;
				return Ok(());
			}

			let v = args[1].to_string();
			let v = substitute(&v);
			let r = context.delete_variable(&v);

			match r {
				Ok(()) => {
					/*write!(stdout,
						"Deleted variable {c}{v}{r}\r\n\n",
						v = v,
						r = format!("{}{}", color::Fg(color::Reset), style::Reset),
						c = format!("{}{}", color::Fg(color::LightBlack), style::Italic)
					)?;*/
				},

				Err(()) => {
					write!(stdout,
						"{c}{v}{r} {t}isn't a variable.{r}\r\n\n",
						v = v,
						r = format!("{}{}", color::Fg(color::Reset), style::Reset),
						t = format!("{}{}", color::Fg(color::Red), style::Bold),
						c = format!("{}{}", color::Fg(color::LightBlack), style::Italic)
					)?;
				}
			}

			return Ok(());
		},

		_ => unreachable!("Bad command!")
	};

	return Ok(());
}
