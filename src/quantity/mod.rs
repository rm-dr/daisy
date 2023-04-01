mod rationalq;

pub mod quantity;
pub use crate::quantity::quantity::Quantity;

const FLOAT_PRECISION: u32 = 2048;
const PRINT_LEN: usize = 4; // How many significant digits we will show in output