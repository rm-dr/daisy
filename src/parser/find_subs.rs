use std::collections::VecDeque;

use crate::parser::Operator;
use crate::parser::Token;
use crate::parser::LineLocation;


pub fn p_find_subs(
	mut g: VecDeque<Token>,
) -> Vec<(LineLocation, String)> {

	let mut r: Vec<(LineLocation, String)> = Vec::with_capacity(8);

	while g.len() > 0 {
		// Read in reverse. Very important!
		let t = g.pop_back().unwrap();

		match &t {

			Token::PreOperator(l, o) => {
				match o {
					Operator::Multiply => { r.push((l.clone(), String::from("×"))); },
					Operator::Divide => { r.push((l.clone(), String::from("÷"))); },
					_ => {}
				}
			},

			Token::PreWord(l, s) => {
				match &s[..] {

					// Greek letters
					"alpha"   => { r.push((l.clone(), String::from("α"))); },
					"beta"    => { r.push((l.clone(), String::from("β"))); },
					"gamma"   => { r.push((l.clone(), String::from("γ"))); },
					"delta"   => { r.push((l.clone(), String::from("δ"))); },
					"epsilon" => { r.push((l.clone(), String::from("ε"))); },
					"zeta"    => { r.push((l.clone(), String::from("ζ"))); },
					"eta"     => { r.push((l.clone(), String::from("η"))); },
					"theta"   => { r.push((l.clone(), String::from("θ"))); },
					"iota"    => { r.push((l.clone(), String::from("ι"))); },
					//"kappa"   => { r.push((l.clone(), String::from("κ"))); },
					"lambda"  => { r.push((l.clone(), String::from("λ"))); },
					"mu"      => { r.push((l.clone(), String::from("μ"))); },
					"nu"      => { r.push((l.clone(), String::from("ν"))); },
					"xi"      => { r.push((l.clone(), String::from("ξ"))); },
					//"omicron" => { r.push((l.clone(), String::from("ο"))); },
					"pi"      => { r.push((l.clone(), String::from("π"))); },
					"rho"     => { r.push((l.clone(), String::from("ρ"))); },
					"sigma"   => { r.push((l.clone(), String::from("σ"))); },
					"tau"     => { r.push((l.clone(), String::from("τ"))); },
					//"upsilon" => { r.push((l.clone(), String::from("υ"))); },
					"phi"     => { r.push((l.clone(), String::from("φ"))); },
					"chi"     => { r.push((l.clone(), String::from("χ"))); },
					"psi"     => { r.push((l.clone(), String::from("ψ"))); },
					"omega"   => { r.push((l.clone(), String::from("ω"))); }

					_ => {}
				}

			}
			_ => {}
		}
	}

	return r;
}