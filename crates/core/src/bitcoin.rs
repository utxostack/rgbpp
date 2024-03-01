use alloc::vec::Vec;
use ckb_gen_types::{packed::Byte32, prelude::Pack};
use molecule::bytes::Bytes;
use sha2::{Digest, Sha256};

pub fn sha2(data: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().into()
}

pub fn calc_txid(data: &Bytes) -> Byte32 {
    sha2(&sha2(data)).pack()
}

pub struct BTCTx {
    pub txid: Byte32,
    pub inputs: Vec<(Byte32, u32)>,
    pub outputs: Vec<Bytes>,
}

pub fn parse_btc_tx(raw_btc_tx: &Bytes) -> BTCTx {
    todo!()
}
