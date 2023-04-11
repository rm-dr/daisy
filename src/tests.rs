// Many of these have been borrowed from insect.
use crate::parser;

fn eval_to_str(s: &str) -> Result<String, ()> {
	let g = match parser::parse(&String::from(s)) {
		Ok(x) => x,
		Err(_) => return Err(())
	};
	//let out_str = g.print();

	return match g.evaluate() {
		Ok(x) => Ok(x.to_string_outer()),
		Err(_) => Err(())
	};
}

fn good_expr(r: &str, s: &str) {
	let out = eval_to_str(s).unwrap();
	assert_eq!(r, out);
}

fn bad_expr(s: &str) {
	let out = eval_to_str(s);

	match out {
		Err(_) => { return },
		_ => {}
	};

	panic!()
}

#[test]
fn basic_numbers() {
	good_expr("1", "1");
	good_expr("1", "1.0");
	good_expr("1", "1.0000");
	good_expr("1", "+1.0");
	good_expr("1", "+1");
	good_expr("3.5", "3.5");
	good_expr("3.5", "3.50");
	good_expr("3.5", "+3.50");
	good_expr("0.2", "0.2");
	good_expr("0.2", "+0.2 ");
	good_expr("0.2", ".2");
	good_expr("0.2", "+.2");
	good_expr("-0.61", "-0.61");
	good_expr("-0.61", "-.61");
	good_expr("-0.61", "-   .61");
	good_expr("0.05", ".05");
	good_expr("-123.45", "-123.45");

	bad_expr("123..");
	bad_expr("0..");
	bad_expr(".0.");
	bad_expr(".");
	bad_expr(". 2");
	bad_expr("..2");
}

#[test]
fn big_numbers() {
	good_expr("1.2346e15", "1234567890000000");
	good_expr("1.2346e15", "1234567890000000.0");
	good_expr("1.2346e15", "+1234567890000000.0");
}

#[test]
fn signs() {
	good_expr( "5", "+++++5");
	good_expr( "5", "++++5");
	good_expr( "5", "+++5");
	good_expr( "5", "++5");
	good_expr( "5", "+5");
	good_expr("-5", "-5");
	good_expr( "5", "--5");
	good_expr("-5", "---5");
	good_expr( "5", "----5");
	good_expr("-5", "-----5");
}

#[test]
fn bad_expressions() {
	bad_expr("2^");
	bad_expr("^2");
	bad_expr("5*");
	bad_expr("5/");
	bad_expr("5%");
	bad_expr("%2");
	bad_expr("3 + ");
	bad_expr("3 + @");
	bad_expr("3 - ");
	bad_expr("()");
	bad_expr("3+2)");
}

#[test]
fn implicit_multiply() {
	good_expr("15", "5(3)");
	good_expr("15", "(5)3");
	good_expr("15", "(5)(3)");
	bad_expr("5 2");
}


#[test]
fn scientific() {
	good_expr("100", "1e2");
	good_expr("0.01", "1e-2");
	good_expr("1", "1e0");

	// In these expressions, `e` is euler's number
	// under implicit multiplication
	good_expr("5.4366", "1e(2)");
	good_expr("14.778", "e2e");

	bad_expr("2 2e2");
	bad_expr("1e1.2");
}


#[test]
fn operators() {

	good_expr("125", "5^(+3)");
	good_expr("125", "+5^3");
	good_expr("0.2148", "3 ^ (-1.4)");

	// Should parse as ((2^3)^4)^5
	good_expr("1.1529e18", "2^3^4^5");

	// Should parse as 1/(2pi)
	good_expr("0.15915", "1/2pi");
	// Should parse as (1/2)*pi
	good_expr("1.5708", "1/2*pi");



	good_expr("15", "5*3");
	good_expr("15", "5 * 3 ");
	good_expr("15", "( 5 ) * ( 3 )");
	good_expr("15", "( 5 ) ( 3 )");
	good_expr("15", "( ( 5 ) * ( 3 ) )");
	good_expr("15", "((5)*(3");
	good_expr("15", "( 5 * 3 )");
	good_expr("15", "5(+3)");
	good_expr("15", "+5*3");

	good_expr("-15", "5*(-3)");
	good_expr("-15", "5 * (-3)");
	good_expr("-15", "( 5 ) * ( -3 )");
	good_expr("-15", "( ( 5 ) * (-( 3 )) )");
	good_expr("-15", "( 5 * (-3) )");
	good_expr("-15", "+5*(-3)");

	good_expr("2", "6/3");
	good_expr("2", "5%3");
	good_expr("8", "5+3");
	good_expr("64", "4^3");
	good_expr("64", "4 ^ 3");
	good_expr("64", "4**3");
	good_expr("-81", "-3^4");
	good_expr("-81", "-(3^4)");
	good_expr("0.5", "2^-1");
	good_expr("0.25", "2^-2");

	good_expr("2", "rt 4");
	good_expr("2", "sqrt 4");
	good_expr("6", "2 rt 9");

	good_expr("7", "3!+1");
	good_expr("18", "3!3");
	bad_expr("3.1!");
	bad_expr("pi!");
}

#[test]
fn units() {
	//good_expr("4 m*s", "2 m * 2s");
	good_expr("1 s⁻¹", "1/s");
	good_expr("6 kg", "2 * 3kg");
	good_expr("1 m/s", "2 m / 2s");
	good_expr("10 m", "10 m");
	good_expr("10 m", "10 * m");
	good_expr("12 m", "(2 + 10) m");

	good_expr("2 m^2", "2m^2");
	good_expr("4 m^2", "(2m)^2");
	good_expr("2 m^2", "2m * m");
	good_expr("1 m^2", "m m");

	//good_expr("2 m", "rt (4m^2)");

	bad_expr("m + s");
	bad_expr("m ^ s");
	//bad_expr("m ^ pi");
}