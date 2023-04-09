/*
Quantity:
	Represents a value with a unit attached to it.
Units have yet to be implemented.

f64q: a quantity based on plain f64s
floatq: a quantity using rug bigfloat
rationalq: a quantity using rug rationals

All of the above are ONLY used for values.
There is only one kind of unit type.


The cfg_if blocks here are a temporary hack to allow for
cross-compilation to other systems. RUG does not work on all systems.
*/

mod scalar;
pub(in crate::quantity) use crate::quantity::scalar::Scalar;


mod unit;
pub use crate::quantity::unit::Unit;
pub use crate::quantity::unit::BaseUnit;

mod quantity;
pub use crate::quantity::quantity::Quantity;
