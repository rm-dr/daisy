use std::env;
use std::path::Path;

mod units;
mod constants;


fn main() -> Result<(), ()>{

	let out_dir = env::var_os("OUT_DIR").unwrap();
	println!("cargo:rerun-if-changed=buildscript/build.rs");
	println!("cargo:rerun-if-changed=buildscript/constants.rs");
	println!("cargo:rerun-if-changed=buildscript/units.rs");
	println!("cargo:rerun-if-changed=buildscript/units.toml");
	println!("cargo:rerun-if-changed=buildscript/constants.toml");

	units::write(&Path::new(&out_dir).join("units.rs"));
	constants::write(&Path::new(&out_dir).join("constants.rs"));
	//constants::write(&Path::new("constants.rs"));

	return Ok(());
}