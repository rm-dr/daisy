const FLOAT_PRECISION: u32 = 1024;
const PRINT_LEN: usize = 5; // How many significant digits we will show in output

pub(in self) mod rationalbase;
pub(in self) mod floatbase;
//mod f64base;

mod scalar;
pub use self::scalar::Scalar;
pub use self::scalar::ScalarBase;