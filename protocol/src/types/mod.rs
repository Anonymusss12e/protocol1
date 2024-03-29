//! Contains newtypes over the standard library types
//! that support finer-grained serialization settings.

pub use self::numerics::Integer;
pub use self::string::String;
pub use self::unimplemented::Unimplemented;
pub use self::vec::Vec;

mod array;
mod char;
/// Definitions for the `std::collections` module.
mod collections;
mod cstring;
mod marker;
mod numerics;
mod option;
mod range;
mod string;
mod tuple;
/// Definitions for smart pointers in the `std` module.
mod smart_ptr;
mod unimplemented;
#[cfg(feature = "uuid")]
mod uuid;
mod vec;

