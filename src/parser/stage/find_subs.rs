use std::collections::VecDeque;

use super::super::{
	Token,
	LineLocation
};


fn sub_string(s: &str) -> Option<&'static str> {
	let r = match s {

		/* Only found in operator tokens */

		"*"    => "×",
		"/"    => "÷",
		"sqrt" => "√",
		"rt"   => "√",



		/* Only found in word tokens */

		// Greek letters
		"alpha"   => "α",
		"beta"    => "β",
		"gamma"   => "γ",
		"delta"   => "δ",
		"epsilon" => "ε",
		"zeta"    => "ζ",
		"eta"     => "η",
		"theta"   => "θ",
		//"iota"    => {Some("ι")}, // looks just like i
		//"kappa"   => {Some("κ")}, // looks just like k
		"lambda"  => "λ",
		"mu"      => "μ",
		//"nu"      => {Some("ν")}, // looks just like v
		"xi"      => "ξ",
		//"omicron" => {Some("ο")}, // looks exactly like o
		"pi"      => "π",
		"rho"     => "ρ",
		"sigma"   => "σ",
		"tau"     => "τ",
		//"upsilon" => {Some("υ")}, // looks just like u
		"phi"     => "φ",
		"chi"     => "χ",
		//"psi"     => {Some("ψ")},  Conflict with pound / square inch
		"omega"   => "ω",

		// Constants
		"epsilon_zero" => "ε₀",
		"eps_zero"     => "ε₀",
		"g_zero"       => "g₀",
		"mu_zero"      => "μ₀",
		"h_bar"        => "ℏ",

		// Misc
		"deg" => "°",

		_ => { return None; }
	};
	return Some(r);
}




pub fn find_subs(
	mut g: VecDeque<Token>,
) -> (
	VecDeque<(LineLocation, String)>,
	VecDeque<Token>
) {

	// Array of replacements
	let mut r: VecDeque<(LineLocation, String)> = VecDeque::with_capacity(8);

	// New token array, with updated locations
	let mut n: VecDeque<Token> = VecDeque::with_capacity(g.len());

	let mut offset: usize = 0;

	while g.len() > 0 {
		let mut t = g.pop_front().unwrap();


		let target: Option<&str> = match &mut t {
			Token::Operator(_, s) => {
				let target = sub_string(s);

				// Update token contents too.
				// This makes errors and printouts use the updated string.
				if target.is_some() { *s = String::from(target.unwrap()); }
				target
			},

			Token::Word(_, s) => {
				let target = sub_string(s);
				if target.is_some() { *s = String::from(target.unwrap()); }
				target
			},

			_ => {None}
		};

		if target.is_none() {
			// Even if nothing changed, we need to update token location
			let l = t.get_mut_linelocation();
			*l = LineLocation{pos: l.pos - offset, len: l.len};
		} else {
			let target = target.unwrap();
			let l = t.get_mut_linelocation();
			r.push_back((*l, String::from(target)));

			let old_len = l.len;
			let new_len = target.chars().count();
			*l = LineLocation{ pos: l.pos - offset, len: new_len};
			offset += old_len - new_len;
		}
		n.push_back(t);
	}

	return (r, n);
}