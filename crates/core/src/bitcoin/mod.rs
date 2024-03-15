#[cfg(feature = "bitcoin-encoder")]
mod encoder;
mod parser;
mod types;
mod utils;

#[cfg(feature = "bitcoin-encoder")]
pub use encoder::*;
pub use parser::*;
pub use sha2::{Digest, Sha256};
pub use types::*;
pub use utils::*;

pub(crate) const OP_RETURN: u8 = 0x6A;
pub(crate) const OP_PUSHBYTES_32: u8 = 0x20;

pub const MIN_BTC_TIME_LOCK_AFTER: u32 = 6;
