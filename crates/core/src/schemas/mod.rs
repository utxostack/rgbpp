#![allow(clippy::needless_lifetimes)]
#![allow(clippy::write_literal)]
#![allow(clippy::if_same_then_else)]
#![allow(clippy::needless_borrow)]
#![allow(clippy::derivable_impls)]
#![allow(clippy::useless_conversion)]
#![allow(clippy::redundant_slicing)]

pub use ckb_gen_types;
pub use ckb_gen_types::packed as blockchain;
mod conversion;
pub mod rgbpp;
