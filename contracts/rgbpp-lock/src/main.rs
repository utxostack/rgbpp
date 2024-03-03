#![no_std]
#![cfg_attr(not(test), no_main)]

#[cfg(test)]
extern crate alloc;

use alloc::vec::Vec;
#[cfg(not(test))]
use ckb_std::default_alloc;
use ckb_std::{
    ckb_constants::Source,
    ckb_types::{
        bytes::Bytes,
        core::ScriptHashType,
        packed::{Byte32, CellDep, Transaction},
        prelude::{Entity, Unpack},
    },
    error::SysError,
    high_level::{
        load_cell_data, load_cell_lock, load_cell_type, load_cell_type_hash, load_input_out_point,
        load_script, load_transaction, load_witness, load_witness_args,
        look_for_dep_with_data_hash, look_for_dep_with_hash2, QueryIter,
    },
    syscalls::load_cell_code,
};
use rgbpp_core::{
    bitcoin::{self, parse_btc_tx, BTCTx},
    rgbpp::{check_btc_time_lock, check_utxo_seal, is_btc_time_lock},
    schemas::rgbpp::*,
    utils::is_script_code_equal,
};
#[cfg(not(test))]
ckb_std::entry!(program_entry);
#[cfg(not(test))]
default_alloc!();

const MIN_BTC_TIME_LOCK_AFTER: u32 = 6;

pub fn program_entry() -> i8 {
    match main() {
        Ok(_) => 0,
        Err(err) => {
            panic!("failed because {:?}", err);
        }
    }
}

fn main() -> Result<(), SysError> {
    let lock_args = {
        let rgbpp_lock = load_script()?;
        RGBPPLock::from_slice(&rgbpp_lock.args().raw_data()).expect("parse RGBPP lock")
    };
    let config = load_rgbpp_config()?;
    let unlock_witness = fetch_unlock_from_witness()?;

    // parse bitcoin transaction
    let raw_btc_tx = unlock_witness.btc_tx().raw_data();
    let btc_tx: BTCTx = parse_btc_tx(&raw_btc_tx);

    verify_unlock(&config, &lock_args, &btc_tx)?;
    verify_outputs(&config, &btc_tx)?;
    Ok(())
}

/// Config cell is deployed together with the current contract
///
/// ``` yaml
/// contract_deployment_transaction:
///   - output(index=0, data=rgbpp_code)
///   - output(index=1, data=rgbpp_config)
/// ```
fn load_rgbpp_config() -> Result<RGBPPConfig, SysError> {
    // get current script
    let script = load_script()?;
    let script_hash_type: ScriptHashType = script.hash_type().try_into().unwrap();
    // look up script dep cell
    let cell_dep_index = look_for_dep_with_hash2(script.code_hash().as_slice(), script_hash_type)?;
    let tx = load_transaction()?;
    let raw_tx = tx.raw();
    let script_cell_dep = raw_tx
        .cell_deps()
        .get(cell_dep_index)
        .expect("find script cell dep");
    let script_out_point_index: u32 = script_cell_dep.out_point().index().unpack();
    assert_eq!(script_out_point_index, 0, "script must be deployed on 0");
    // look up config dep cell
    let config_cell_dep_index = raw_tx
        .cell_deps()
        .into_iter()
        .enumerate()
        .find(|(_index, cell_dep)| {
            let index: u32 = cell_dep.out_point().index().unpack();
            index == 1 && cell_dep.out_point().tx_hash() == script_cell_dep.out_point().tx_hash()
        })
        .expect("find config cell dep")
        .0;
    let data = load_cell_data(config_cell_dep_index, Source::CellDep)?;
    let config = RGBPPConfig::from_slice(&data).expect("parse config");
    Ok(config)
}

/// Verify outputs cells is protected with RGB++ lock
fn verify_outputs(config: &RGBPPConfig, btc_tx: &BTCTx) -> Result<(), SysError> {
    let rgbpp_lock = load_script()?;
    for (index, type_hash) in QueryIter::new(load_cell_type_hash, Source::Output).enumerate() {
        // ignore non-type cells
        if type_hash.is_none() {
            continue;
        }

        let lock = load_cell_lock(index, Source::Output)?;
        // check RGB++ lock
        if is_script_code_equal(&lock, &rgbpp_lock) {
            // check new seal txid + index is valid
            let lock_args =
                RGBPPLock::from_slice(lock.args().as_slice()).expect("Invalid RGBPP lock args");
            if check_utxo_seal(&lock_args, btc_tx) {
                continue;
            }
        }

        // check bitcoin time lock
        if is_btc_time_lock(config, &lock) {
            // check new seal txid + index is valid
            let lock_args = BTCTimeLock::from_slice(lock.args().as_slice())
                .expect("Invalid BTCTimeLock lock args");
            if check_btc_time_lock(&lock_args, btc_tx, MIN_BTC_TIME_LOCK_AFTER) {
                continue;
            }
        }

        panic!("All outputs with type should be locked by RGBPP lock or Bitcoin time lock");
    }
    Ok(())
}

fn fetch_unlock_from_witness() -> Result<RGBPPUnlock, SysError> {
    let witness_args = load_witness_args(0, Source::GroupInput)?;
    match witness_args.lock().to_opt() {
        Some(args) => {
            let unlock = RGBPPUnlock::from_slice(args.as_slice()).unwrap();
            Ok(unlock)
        }
        None => Err(SysError::ItemMissing),
    }
}

/// Verify unlock
fn verify_unlock(
    config: &RGBPPConfig,
    lock_args: &RGBPPLock,
    btc_tx: &BTCTx,
) -> Result<(), SysError> {
    // check bitcoin transaction inputs unlock RGB++ cell
    let expected_out_point: (Byte32, u32) = (lock_args.btc_txid(), lock_args.out_index().unpack());
    let is_found = btc_tx
        .inputs
        .iter()
        .any(|txin| &txin.previous_output == &expected_out_point);
    if !is_found {
        panic!("Bitcoin transaction doesn't unlock this cell");
    }

    // check bitcoin transaction exists in light client
    let is_exists = check_btc_tx_exists(config, &btc_tx.txid)?;
    if !is_exists {
        panic!("Bitcoin transaction doesn't exists in the light client");
    }

    // verify commitment
    check_btc_tx_commitment(&btc_tx);
    Ok(())
}

fn check_btc_tx_commitment(btc_tx: &BTCTx) {
    todo!()
}

/// Check light client cell
/// TODO this is a mock implementation!!!
fn check_btc_tx_exists(config: &RGBPPConfig, btc_txid: &Byte32) -> Result<bool, SysError> {
    let btc_lc_type_hash = config.btc_lc_type_hash();
    let index = QueryIter::new(load_cell_type_hash, Source::CellDep)
        .enumerate()
        .find_map(|(index, type_hash)| {
            if type_hash.is_some_and(|type_hash| &type_hash == btc_lc_type_hash.as_slice()) {
                Some(index)
            } else {
                None
            }
        })
        .expect("can't find light client cell");
    let data = load_cell_data(index, Source::CellDep)?;
    Ok(&data == btc_txid.as_slice())
}
