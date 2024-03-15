use super::types::*;
use super::*;

use ckb_gen_types::{packed::Byte32, prelude::*};
pub use sha2::{Digest, Sha256};

pub fn sha2(data: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().into()
}

pub fn extract_commitment(btc_tx: &BTCTx) -> Option<Byte32> {
    // find first op_return, other op_return must be ignored
    let op_return_out = btc_tx
        .outputs
        .iter()
        .find(|output| output.script[0] == OP_RETURN)?;
    // check push 32 bytes
    if op_return_out.script[1] != OP_PUSHBYTES_32 || op_return_out.script.len() != 34 {
        return None;
    }
    // parse commitment
    let commitment: [u8; 32] = op_return_out.script[2..].try_into().unwrap();
    Some(commitment.pack())
}
