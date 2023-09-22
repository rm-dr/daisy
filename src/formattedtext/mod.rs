mod formattedtext;
pub use formattedtext::FormattedText;


// Select write implementation by target system
cfg_if::cfg_if! {
	if #[cfg(target_family = "unix")] {
		mod unix_backend;
	} else if #[cfg(target_arch = "wasm32")] {
		mod wasm_backend;
	}
}