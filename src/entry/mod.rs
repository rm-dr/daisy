
cfg_if::cfg_if! {
	if #[cfg(target_family = "unix")] {
		mod unix;
		pub use unix::main as main_e;
	} else {
		pub fn main_e () -> Result<(), std::io::Error> {
			unimplemented!("Not yet implemented.");
		}
	}
}