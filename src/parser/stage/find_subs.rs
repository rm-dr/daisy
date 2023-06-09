use std::collections::VecDeque;

use super::super::{
	Token,
	LineLocation
};


pub fn find_subs(
	mut g: VecDeque<Token>,
) -> (
	Vec<(LineLocation, String)>,
	VecDeque<Token>
) {

	// Array of replacements
	let mut r: Vec<(LineLocation, String)> = Vec::with_capacity(8);

	// New token array, with updated locations
	let mut n: VecDeque<Token> = VecDeque::with_capacity(g.len());

	let mut offset: usize = 0;

	while g.len() > 0 {
		// Read in reverse. Very important!
		let mut t = g.pop_back().unwrap();

		let target: Option<&str> = match &mut t {
			Token::Operator(_, s) => {
				let target = match &s[..] {
					"*" => {Some("×")},
					"/" => {Some("÷")},
					"sqrt"    => {Some("√")},
					"rt"      => {Some("√")},
					_ => {None}
				};

				// Update token contents too.
				// This makes sure that errors also contain the updated text.
				if target.is_some() { *s = String::from(target.unwrap()); }
				target
			},

			Token::Word(_, s) => {
				let target = match &s[..] {
					// Greek letters
					"alpha"   => {Some("α")},
					"beta"    => {Some("β")},
					"gamma"   => {Some("γ")},
					"delta"   => {Some("δ")},
					"epsilon" => {Some("ε")},
					"zeta"    => {Some("ζ")},
					"eta"     => {Some("η")},
					"theta"   => {Some("θ")},
					//"iota"    => {Some("ι")},
					//"kappa"   => {Some("κ")},
					"lambda"  => {Some("λ")},
					"mu"      => {Some("μ")},
					//"nu"      => {Some("ν")},
					"xi"      => {Some("ξ")},
					//"omicron" => {Some("ο")},
					"pi"      => {Some("π")},
					"rho"     => {Some("ρ")},
					"sigma"   => {Some("σ")},
					"tau"     => {Some("τ")},
					//"upsilon" => {Some("υ")},
					"phi"     => {Some("φ")},
					"chi"     => {Some("χ")},
					//"psi"     => {Some("ψ")},  Conflict with pound / square inch
					"omega"   => {Some("ω")},
					_ => {None}
				};

				if target.is_some() { *s = String::from(target.unwrap()); }
				target
			},

			_ => {None}
		};

		if target.is_none() {
			// Even if nothing changed, we need to update token location
			let l = t.get_mut_line_location();
			*l = LineLocation{pos: l.pos - offset, len: l.len};
		} else {
			let target = target.unwrap();
			let l = t.get_mut_line_location();
			r.push((l.clone(), String::from(target)));
			*l = LineLocation{ pos: l.pos - offset, len: target.chars().count()};
			offset += l.len - target.chars().count();
		}
		n.push_front(t);
	}

	return (r, n);
}