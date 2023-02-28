#![no_std]

mod error;
mod table;

pub use error::Error;
pub use table::CoSortTable;

#[cfg(any(test, feature = "std-error"))]
#[macro_use]
extern crate std;

#[cfg(test)]
mod tests;
