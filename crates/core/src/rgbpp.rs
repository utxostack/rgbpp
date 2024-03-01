use ckb_gen_types::{core::ScriptHashType, packed::Script, prelude::Unpack};

use crate::{
    bitcoin::BTCTx,
    schemas::rgbpp::{BTCTimeLock, RGBPPConfig, RGBPPLock},
};

pub fn check_utxo_seal(lock_args: &RGBPPLock, btc_tx: &BTCTx) -> bool {
    let out_index: u32 = lock_args.out_index().unpack();
    lock_args.btc_txid() == btc_tx.txid && out_index < btc_tx.outputs.len() as u32
}

pub fn check_btc_time_lock(lock_args: &BTCTimeLock, btc_tx: &BTCTx, min_lock: u32) -> bool {
    let after: u32 = lock_args.after().unpack();
    lock_args.btc_txid() == btc_tx.txid && after >= min_lock
}

pub fn is_btc_time_lock(config: &RGBPPConfig, lock: &Script) {
    lock.code_hash() == config.btc_time_lock_type_hash()
        && lock.hash_type() == ScriptHashType::Type.into()
}
