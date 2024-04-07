use super::types::*;

use ckb_gen_types::{bytes::Bytes, prelude::*};
use molecule::bytes::{BufMut, BytesMut};
pub use sha2::Digest;

struct Encoder {
    buf: BytesMut,
}

impl Encoder {
    pub fn new() -> Self {
        Self {
            buf: BytesMut::default(),
        }
    }

    pub fn put_var_int(&mut self, n: usize) {
        if n > 0xFFFFFFFF {
            self.buf.put_u8(0xFF);
            self.buf.put_u64_le(n as u64);
        } else if n > 0xFFFF {
            self.buf.put_u8(0xFE);
            self.buf.put_u32_le(n as u32);
        } else if n > 0xFD {
            self.buf.put_u8(0xFD);
            self.buf.put_u16_le(n as u16);
        } else {
            self.buf.put_u8(n as u8);
        }
    }

    pub fn put_txin(&mut self, txin: &TxIn) {
        let TxIn {
            previous_output: (txid, vout),
            script,
            sequence,
        } = txin;
        self.buf.put_slice(txid.as_slice());
        self.buf.put_u32_le(*vout);
        self.put_var_int(script.len());
        self.buf.put_slice(script);
        self.buf.put_u32_le(*sequence);
    }

    pub fn put_txout(&mut self, txout: &TxOut) {
        let TxOut { value, script } = txout;
        self.buf.put_i64_le(*value);
        self.put_var_int(script.len());
        self.buf.put_slice(script);
    }
}

pub fn encode_btc_tx(btc_tx: BTCTx) -> Bytes {
    let BTCTx {
        txid: _,
        version,
        inputs,
        outputs,
        lock_time,
    } = btc_tx;
    let mut encoder = Encoder::new();
    encoder.buf.put_u32_le(version);
    encoder.put_var_int(inputs.len());

    for input in inputs.iter() {
        encoder.put_txin(input);
    }

    encoder.put_var_int(outputs.len());

    for output in outputs.iter() {
        encoder.put_txout(output);
    }
    encoder.buf.put_u32_le(lock_time);
    encoder.buf.freeze()
}
