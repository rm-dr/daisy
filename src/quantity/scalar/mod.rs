//const FLOAT_PRECISION: u32 = 1024;
const SHOW_SIG: usize = 5; // How many significant digits we will show in output
const MAX_LEN: usize = 5; // If a scientific exponent is >= this value, do not use scientific notation.

pub(in self) mod rationalbase;


// Pick a float implementation.
// floatbase is high-precision, f64base is for testing.

//pub(in self) mod floatbase;
//pub use floatbase::FloatBase;

pub(in self) mod f64base;
pub use f64base::F64Base as FloatBase;



mod scalar;
pub use self::scalar::Scalar;
pub use self::scalar::ScalarBase;



// Convert a string to scientific notation,
// with parameters SHOW_SIG and MAX_LEN.
//
// input (s): a decimal of any length, like 123123.123123
// s may start with an optional `-` sign.
pub(in self)  fn dec_to_sci(mut s: String) -> String {
	// Remove negative sign from string
	let neg = s.starts_with("-");
	if neg { s = String::from(&s[1..]); }
	
	// Power of ten
	let mut p: i32 = {
		if let Some(x) = s.find(".") {
			x as i32
		} else {
			s.len() as i32
		}
	};
	p -= 1;

	// We no longer need a decimal point in our string.
	// also, trim off leading zeros and adjust power.
	let mut s: &str = &s.replace(".", "");
	s = &s[0..];
	s = s.trim_end_matches('0');
	while s.starts_with('0') {
		s = &s[1..];
		p -= 1;
	}


	// Pick significant digits and round
	let mut s = String::from(s);
	if s.len() > SHOW_SIG {
		let round;
		if s.len() != SHOW_SIG + 1 {
			round = s[SHOW_SIG..SHOW_SIG+1].parse().unwrap();
		} else { round = 0; }

		s = String::from(&s[0..SHOW_SIG]);

		if round >= 5 {
			let new = s[s.len()-1..s.len()].parse::<u8>().unwrap() + 1u8;
			if new != 10 {
				s = format!("{}{new}", &s[0..s.len()-1]);
			}
		}
	}

	s = format!("{s}{}", "0".repeat(SHOW_SIG - s.len()));
	// at this point, s is guaranteed to have exactly SHOW_SIG digits.

	let neg = if neg {"-"} else {""};

	if (p.abs() as usize) < MAX_LEN {
		if p >= 0 {
			let q = p as usize;

			let first = &s[0..q+1];
			let mut rest = &s[q+1..];
			rest = rest.trim_end_matches('0');
			if rest == "" {
				return format!("{neg}{first}");
			} else {
				return format!("{neg}{first}.{rest}");
			}
		} else {
			let q = p.abs() as usize;
			let t = format!("0.{}{s}", "0".repeat(q-1));
			return format!("{neg}{}", t.trim_end_matches('0'));
		}

	// Print full scientific notation
	} else {
		let first = &s[0..1];
		let mut rest = &s[1..];
		rest = rest.trim_end_matches('0');
		if rest == "" {
			return format!("{neg}{first}e{p}");
		} else {
			return format!("{neg}{first}.{rest}e{p}");
		}
	}
}