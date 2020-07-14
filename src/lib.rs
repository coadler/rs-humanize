#![feature(duration_constants)]
#![feature(test)]
pub mod bytes;
pub mod time;

mod ordinal;
pub use ordinal::{ordinal, ordinal_ref};
