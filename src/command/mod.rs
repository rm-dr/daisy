use std::io::Write;

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
	match &s.trim()[..] {
		"help" | "clear"
		| "ops" | "operators"
		| "fns" | "functions"
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
	s: &String
) -> Result<(), std::io::Error> {

	match &s[..] {
		"help" => {
			draw_greeter(stdout)?;

			write!(stdout,
				concat!(
					"Daisy is a high-precision, general-purpose\r\n",
					"scientific calculator.\r\n",
					"\n",
					" - Use Up/Down arrows to navigate history.\r\n",
					" - Use Ctrl-C or Ctrl-D to quit.\r\n",
					"\n",
					"╞═══════════════ {t}Commands{r} ═══════════════╡\r\n",
					"      {c}help{r}   Show this help\r\n",
					"      {c}clear{r}  Clear the terminal\r\n",
					"      {c}quit{r}   Exit daisy\r\n",
					//"      {c}units{r}  List available units\r\n",
					//"      {c}const{r}  List available constants\r\n",
					"      {c}ops{r}    List built-in operators\r\n",
					"      {c}fns{r}    List built-in functions\r\n",
					"\n",
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
					"  modulo (long)        {c}mod{r}\r\n",
					"\n"
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
					"\r\n",
					"  convert to base unit     {c}tobase(quantity){r}\r\n",
					"  remove units             {c}nounit(quantity){r}\r\n",
					"\n"
				),

				r = format!("{}{}", color::Fg(color::Reset), style::Reset),
				c = format!("{}{}", color::Fg(color::LightBlack), style::Italic),
				t = format!("{}{}", color::Fg(color::Magenta), style::Bold)
			)?;
		},
		_ => unreachable!("Bad command!")
	};

	return Ok(());
}
