use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::ops::{
	Mul, Div,
	MulAssign, DivAssign
};

use super::Scalar;
use super::Quantity;


#[derive(Hash)]
#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(Eq, PartialEq)]
pub enum UnitBase {
	// Base Units
	Second,
	Meter,
	Gram, // Technically kilogram, but that messes with prefix architecture.
	Ampere,
	Kelvin,
	Mole,
	Candela,

	// Length units
	Inch,
	Foot,
	Mile,

	// Time units
	Minute,
	Hour,
	Day,
	//Week,
	//Month,
}

#[derive(Hash)]
#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(Eq, PartialEq)]
pub enum Prefix {
	None,

	Quetta,
	Ronna,
	Yotta,
	Zetta,
	Exa,
	Peta,
	Tera,
	Giga,
	Mega,
	Kilo,
	Hecto,
	Deka,

	Deci,
	Centi,
	Milli,
	Micro,
	Nano,
	Pico,
	Femto,
	Atto,
	Zepto,
	Yocto,
	Ronto,
	Quecto
}

impl Prefix {
	pub fn to_ratio(&self) -> Quantity {
		let q = Quantity::new_rational_from_string(match self {
			Prefix::Quetta => "1e30",
			Prefix::Ronna => "1e27",
			Prefix::Yotta => "1e24",
			Prefix::Zetta => "1e21",
			Prefix::Exa => "1e18",
			Prefix::Peta => "1e15",
			Prefix::Tera => "1e12",
			Prefix::Giga => "1e9",
			Prefix::Mega => "1e6",
			Prefix::Kilo => "1e3",
			Prefix::Hecto => "1e2",
			Prefix::Deka => "1e1",

			Prefix::Deci => "1e-1",
			Prefix::Centi => "1e-2",
			Prefix::Milli => "1e-3",
			Prefix::Micro => "1e-6",
			Prefix::Nano => "1e-9",
			Prefix::Pico => "1e-12",
			Prefix::Femto => "1e-15",
			Prefix::Atto => "1e-18",
			Prefix::Zepto => "1e-21",
			Prefix::Yocto => "1e-24",
			Prefix::Ronto => "1e-27",
			Prefix::Quecto => "1e-30",

			Prefix::None => { "1" }
		}).unwrap();

		return q;

	}
}

impl ToString for Prefix {
	fn to_string(&self) -> String {
		String::from(match self {
			Prefix::Quetta => "Q",
			Prefix::Ronna => "R",
			Prefix::Yotta => "Y",
			Prefix::Zetta => "Z",
			Prefix::Exa => "E",
			Prefix::Peta => "P",
			Prefix::Tera => "T",
			Prefix::Giga => "G",
			Prefix::Mega => "M",
			Prefix::Kilo => "k",
			Prefix::Hecto => "h",
			Prefix::Deka => "da",

			Prefix::Deci => "d",
			Prefix::Centi => "c",
			Prefix::Milli => "m",
			Prefix::Micro => "u",
			Prefix::Nano => "n",
			Prefix::Pico => "p",
			Prefix::Femto => "f",
			Prefix::Atto => "a",
			Prefix::Zepto => "z",
			Prefix::Yocto => "y",
			Prefix::Ronto => "r",
			Prefix::Quecto => "q",

			Prefix::None => ""
		})
	}
}


#[derive(Debug)]
#[derive(Copy, Clone)]
pub struct FreeUnit {
	base: UnitBase,
	prefix: Prefix
}

impl Hash for FreeUnit {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.base.hash(state);
	}
}

impl Eq for FreeUnit {}
impl PartialEq for FreeUnit {
	fn eq(&self, other: &Self) -> bool {
		self.base.eq(&other.base)
	}
}


macro_rules! quick_base_factor {
	(float, $u:expr, $s:expr, $( ($x:expr, $p:expr) ),* ) => {
		Some(Quantity {
			scalar: Scalar::new_float_from_string($s).unwrap(),
			unit: Unit::from_array(&[
				$(
					(FreeUnit::from_base($x), Scalar::new_rational($p).unwrap()),
				)*
				(FreeUnit::from_base($u), Scalar::new_rational(-1f64).unwrap())
			])
		})
	};

	(rational, $u:expr, $s:expr, $( ($x:expr, $p:expr) ),* ) => {
		Some(Quantity {
			scalar: Scalar::new_float_from_string($s).unwrap(),
			unit: Unit::from_array(&[
				$(
					(FreeUnit::from_base($x), Scalar::new_rational($p).unwrap()),
				)*
				(FreeUnit::from_base($u), Scalar::new_rational(-1f64).unwrap())
			])
		})
	};
}


impl FreeUnit {
	pub fn from_base(base: UnitBase) -> FreeUnit {
		return FreeUnit { base, prefix: Prefix::None }
	}

	pub fn from_base_prefix(base: UnitBase, prefix: Prefix) -> FreeUnit { FreeUnit {base, prefix} }
	pub fn set_prefix(&mut self, prefix: Prefix) { self.prefix = prefix; }
	pub fn get_prefix(&self) -> Prefix { self.prefix }

	pub fn same_with_prefix(&self, other: &FreeUnit) -> bool {
		self.base.eq(&other.base) && self.prefix.eq(&other.prefix)
	}


	pub fn to_base_factor(&self) -> Quantity {
		let q = match self.base {
			// Returns the unit we need to multiply by to get a base
			// unit, or `None` if this is already a base unit.
			//
			// Example:
			// 1 foot  = 0.3048 m,
			// so 1 ft * (0.3084 m / ft) will give meters.
			//
			// The units here MUST be in terms of base units.
			// If they aren't, things will break.

			UnitBase::Foot => quick_base_factor!(float,
				UnitBase::Foot,
				"0.3048",
				(UnitBase::Meter, 1f64)
			),

			UnitBase::Inch => quick_base_factor!(float,
				UnitBase::Inch,
				"0.0254",
				(UnitBase::Meter, 1f64)
			),

			UnitBase::Mile => quick_base_factor!(rational,
				UnitBase::Mile,
				"1609",
				(UnitBase::Meter, 1f64)
			),

			UnitBase::Minute => quick_base_factor!(rational,
				UnitBase::Minute,
				"60",
				(UnitBase::Second, 1f64)
			),

			UnitBase::Hour => quick_base_factor!(rational,
				UnitBase::Hour,
				"3600",
				(UnitBase::Second, 1f64)
			),

			UnitBase::Day => quick_base_factor!(rational,
				UnitBase::Day,
				"86400",
				(UnitBase::Second, 1f64)
			),

			// Only base units should be missing a conversion factor.
			_ => None
		};

		let mut q = q.unwrap_or(Quantity::new_rational_from_string("1").unwrap());

		let mut p = self.prefix.to_ratio();
		p.insert_unit(FreeUnit::from_base(self.base), Scalar::new_rational(1f64).unwrap());
		p.insert_unit(FreeUnit::from_base_prefix(self.base, self.prefix), Scalar::new_rational(-1f64).unwrap());
		q *= p;

		return q;
	}
}


impl ToString for FreeUnit {
	fn to_string(&self) -> String {
		let s = match self.base {
			UnitBase::Second => "s",
			UnitBase::Meter => "m",
			UnitBase::Gram => "g",
			UnitBase::Ampere => "a",
			UnitBase::Kelvin => "k",
			UnitBase::Mole => "mol",
			UnitBase::Candela => "c",

			UnitBase::Foot => "ft",
			UnitBase::Inch => "in",
			UnitBase::Mile => "mile",

			UnitBase::Hour => "hour",
			UnitBase::Minute => "min",
			UnitBase::Day => "day",
		};

		let p = self.prefix.to_string();

		format!("{p}{s}")
	}
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Unit {
	// Unit, power.
	pub val: HashMap<FreeUnit, Scalar>
}

impl ToString for Unit {
	fn to_string(&self) -> String {
		if self.unitless() { return String::new(); };

		let mut top_empty = true;
		let mut bottom_empty = true;

		for (_, p) in self.get_val() {
			if p.is_positive() {
				top_empty = false;
			} else {
				bottom_empty = false;
			}
		};

		let mut t = String::new();
		let mut b = String::new();

		for (u, p) in self.get_val() {
			let c = u.to_string();

			if *p == Scalar::new_rational(1f64).unwrap() {
				t.push_str(&format!("{c}·"));
			} else if *p == Scalar::new_rational(-1f64).unwrap() {
				if top_empty {
					b.push_str(&format!("{c}⁻¹·"));
				} else {
					b.push_str(&format!("{c}·"));
				}
			} else if p.is_positive() {
				t.push_str(&format!("{c}^{}·", p.to_string()));
			} else {
				if top_empty {
					b.push_str(&format!("{c}^{}·", p.to_string()));
				} else {
					b.push_str(&format!("{c}^{}·", (-p.clone()).to_string()));
				}
			}
		};

		if top_empty {
			format!("{}", &b[..b.len()-2]) // Slice cuts off the last `·` (2 bytes)
		} else if bottom_empty {
			format!("{}", &t[..t.len()-2])
		} else {
			format!("{}/{}", &t[..t.len()-2], &b[..b.len()-2])
		}
	}
}


impl Unit {
	pub fn new() -> Unit {
		return Unit {
			val: HashMap::new()
		}
	}

	pub fn get_val(&self) -> &HashMap<FreeUnit, Scalar> { &self.val }
	pub fn get_val_mut(&mut self) -> &mut HashMap<FreeUnit, Scalar> { &mut self.val }
	pub fn unitless(&self) -> bool { self.get_val().len() == 0 }

	pub fn from_array(a: &[(FreeUnit, Scalar)]) -> Unit {
		let mut n = Unit::new();
		for (u, p) in a.iter() {
			n.insert(*u, p.clone());
		}
		return n;
	}

	pub fn prefixes_match(&self, other: &Unit) -> bool {
		let v = self.get_val();
		for (u, _) in other.get_val() {
			let k = v.get_key_value(u);

			if k.is_some() {
				let k = k.unwrap().0;
				if !u.same_with_prefix(k) { return false; }
			}
		}
		return true;
	}

	pub fn match_prefix_factor(&self, other: &Unit) -> Quantity {
		let mut f = Quantity::new_rational(1f64).unwrap();

		let v = self.get_val();
		for (ou, op) in other.get_val() {
			let k = v.get_key_value(ou);

			if k.is_some() {
				let (su, _) = k.unwrap();

				// Conversion factor ou -> basic
				let mut p = ou.prefix.to_ratio();
				p.insert_unit(FreeUnit::from_base(ou.base), Scalar::new_rational(1f64).unwrap());
				p.insert_unit(FreeUnit::from_base_prefix(ou.base, ou.prefix), Scalar::new_rational(-1f64).unwrap());

				// Conversion factor su -> basic
				let mut q = su.prefix.to_ratio();
				q.insert_unit(FreeUnit::from_base(su.base), Scalar::new_rational(1f64).unwrap());
				q.insert_unit(FreeUnit::from_base_prefix(su.base, su.prefix), Scalar::new_rational(-1f64).unwrap());

				f = f * (p / q).pow(Quantity::from_scalar(op.clone()));
			}
		}

		return f;
	}


	pub fn insert(&mut self, u: FreeUnit, p: Scalar) {
		let v = self.get_val_mut();
		match v.get_mut(&u) {
			Some(i) => {
				let n = i.clone() + p;

				if n.is_zero() {
					v.remove(&u);
				} else { *i = n; }
			},
			None => { v.insert(u, p); }
		};
	}

	pub fn pow(&self, pwr: Scalar) -> Unit {
		let mut u = self.clone();
		for (_, p) in u.get_val_mut() {
			*p *= pwr.clone();
		};
		return u;
	}


	pub fn from_string(s: &str) -> Option<Quantity> {
		// Base Units
		let b = match s {
			"m" => Some(UnitBase::Meter),
			"s" => Some(UnitBase::Second),
			"sec" => Some(UnitBase::Second),
			"g" => Some(UnitBase::Gram),
			"a" => Some(UnitBase::Ampere),
			"k" => Some(UnitBase::Kelvin),
			"mol" => Some(UnitBase::Mole),
			"c" => Some(UnitBase::Candela),
			"ft" => Some(UnitBase::Foot),
			"mile" => Some(UnitBase::Mile),
			"hour" => Some(UnitBase::Hour),
			"min" => Some(UnitBase::Minute),
			"day" => Some(UnitBase::Day),
			_ => { None }
		};

		if b.is_some() {
			let mut u = Unit::new();
			let b = FreeUnit::from_base(b.unwrap());

			u.insert(b, Scalar::new_rational(1f64).unwrap());

			let mut q = Quantity::new_rational(1f64).unwrap();
			q.set_unit(u);

			return Some(q);
		};

		if b.is_none() {
			if s == "kg" {
				let mut u = Unit::new();
				let b = FreeUnit::from_base_prefix(UnitBase::Gram, Prefix::Kilo);

				u.insert(b, Scalar::new_rational(1f64).unwrap());

				let mut q = Quantity::new_rational(1f64).unwrap();
				q.set_unit(u);

				return Some(q);
			}

			if s == "km" {
				let mut u = Unit::new();
				let b = FreeUnit::from_base_prefix(UnitBase::Meter, Prefix::Kilo);

				u.insert(b, Scalar::new_rational(1f64).unwrap());

				let mut q = Quantity::new_rational(1f64).unwrap();
				q.set_unit(u);

				return Some(q);
			}
		}

		return None;
	}


	pub fn to_base_factor(&self) -> Quantity {
		let mut q = Quantity::new_rational(1f64).unwrap();

		for (u, p) in self.get_val().iter() {
			let b = u.to_base_factor();
			q *= b.pow(Quantity::from_scalar(p.clone()));
		}

		return q;
	}
}


impl PartialEq for Unit {
	fn eq(&self, other: &Self) -> bool {
		let v = self.get_val();
		for (u, p) in other.get_val() {
			match v.get(u) {
				Some(i) => { if i != p { return false; } },
				None => { return false; }
			};
		}
		return true;
	}
}

impl Mul for Unit {
	type Output = Self;

	fn mul(self, other: Self) -> Self::Output {
		let mut o = self.clone();
		for (u, p) in other.get_val() { o.insert(*u, p.clone()); }
		return o;
	}
}

impl MulAssign for Unit where {
	fn mul_assign(&mut self, other: Self) {
		for (u, p) in other.get_val() { self.insert(*u, p.clone()); }
	}
}

impl Div for Unit {
	type Output = Self;

	fn div(self, other: Self) -> Self::Output {
		let mut o = self.clone();
		for (u, p) in other.get_val() { o.insert(*u, -p.clone()); }
		return o;
	}
}

impl DivAssign for Unit where {
	fn div_assign(&mut self, other: Self) {
		for (u, p) in other.get_val() { self.insert(*u, -p.clone()); }
	}
}
