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