use std::hash::Hash;

mod freeunit;
mod prefix;
mod unit;

pub use prefix::Prefix;
pub use unit::Unit;
pub use freeunit::FreeUnit;

use crate::quantity::Quantity;
use crate::quantity::Scalar;

use prefix::str_to_prefix;
include!(concat!(env!("OUT_DIR"), "/units.rs"));