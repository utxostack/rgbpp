use core::fmt::Write;

use super::types::*;
use super::utils::*;
use alloc::vec::Vec;
use ckb_gen_types::{bytes::Bytes, packed::Byte32, prelude::*};

struct Parser<'r> {
    data: &'r Bytes,
    offset: usize,
}

impl<'r> Parser<'r> {
    pub fn new(data: &'r Bytes) -> Self {
        Self { data, offset: 0 }
    }

    fn read_slice(&mut self, n: usize) -> &[u8] {
        let s = self.offset;
        let e = s + n;
        self.offset = e;
        &self.data[s..e]
    }

    pub fn is_exhausted(&self) -> bool {
        self.offset == self.data.len()
    }

    pub fn read_u8(&mut self) -> u8 {
        self.read_slice(1)[0]
    }

    pub fn read_u16(&mut self) -> u16 {
        u16::from_le_bytes(self.read_slice(2).try_into().unwrap())
    }

    pub fn read_u32(&mut self) -> u32 {
        u32::from_le_bytes(self.read_slice(4).try_into().unwrap())
    }

    pub fn read_u64(&mut self) -> u64 {
        u64::from_le_bytes(self.read_slice(8).try_into().unwrap())
    }

    pub fn read_i64(&mut self) -> i64 {
        i64::from_le_bytes(self.read_slice(8).try_into().unwrap())
    }

    pub fn read_var_int(&mut self) -> usize {
        let n = self.read_u8();
        match n {
            0xFD => {
                let v = self.read_u16();
                assert!(v >= 0xFD);
                v.into()
            }
            0xFE => {
                let v = self.read_u32();
                assert!(v > 0xFFFF);
                v.try_into().expect("overflow")
            }
            0xFF => {
                let v = self.read_u64();
                assert!(v > 0xFFFFFFFF);
                v.try_into().expect("overflow")
            }
            _ => n.into(),
        }
    }

    pub fn read_byte32(&mut self) -> Byte32 {
        let byte32: [u8; 32] = self.read_slice(32).try_into().unwrap();
        byte32.pack()
    }

    pub fn read_bytes(&mut self, n: usize) -> Bytes {
        self.read_slice(n).to_vec().into()
    }

    pub fn read_txin(&mut self) -> TxIn {
        let txid = self.read_byte32();
        let vout = self.read_u32();
        let script_len = self.read_var_int();
        let script = self.read_bytes(script_len);
        let sequence = self.read_u32();

        TxIn {
            previous_output: (txid, vout),
            script,
            sequence,
        }
    }

    pub fn read_txout(&mut self) -> TxOut {
        let value = self.read_i64();
        let script_len = self.read_var_int();
        let script = self.read_bytes(script_len);
        TxOut { value, script }
    }
}

pub fn parse_btc_tx(data: &Bytes) -> BTCTx {
    let txid = sha2(&sha2(data)).pack();

    let mut p = Parser::new(data);

    let version = p.read_u32();
    let inputs_len = p.read_var_int();
    let mut inputs = Vec::with_capacity(inputs_len);

    for _ in 0..inputs_len {
        inputs.push(p.read_txin());
    }

    let outputs_len = p.read_var_int();
    let mut outputs = Vec::with_capacity(outputs_len);
    for _ in 0..outputs_len {
        outputs.push(p.read_txout());
    }
    let lock_time = p.read_u32();

    assert!(p.is_exhausted(), "can't parse remain data");

    BTCTx {
        txid,
        version,
        inputs,
        outputs,
        lock_time,
    }
}
