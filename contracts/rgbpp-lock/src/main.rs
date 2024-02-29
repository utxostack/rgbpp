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
        packed::{Byte32, Transaction},
        prelude::{Entity, Unpack},
    },
    error::SysError,
    high_level::{
        load_cell_lock, load_cell_type_hash, load_input_out_point, load_script, load_witness,
        load_witness_args, QueryIter,
    },
};
use rgbpp_core::schemas::rgbpp::*;
#[cfg(not(test))]
ckb_std::entry!(program_entry);
#[cfg(not(test))]
default_alloc!();

pub fn program_entry() -> i8 {
    match main() {
        Ok(_) => 0,
        Err(err) => {
            panic!("failed because {:?}", err);
        }
    }
}

fn main() -> Result<(), SysError> {
    let lock_args = todo!();
    let config = todo!();
    let unlock_witness = fetch_unlock_from_witness()?;
    verify_input_rgbpp_cell(&unlock_witness);
    verify_unlock(&lock_args, &unlock_witness);
    verify_outputs(&config);
    Ok(())
}

/// Verify outputs cells is protected with RGB++ lock
fn verify_outputs(config: &RGBPPConfig) -> Result<(), SysError> {
    let rgbpp_lock = load_script()?;
    for (index, type_hash) in QueryIter::new(load_cell_type_hash, Source::Output).enumerate() {
        if type_hash.is_none() {
            continue;
        }

        let lock = load_cell_lock(index, Source::Output)?;
        // must be locked by RGB++ lock or BTC time lock
        if lock.code_hash() == rgbpp_lock.code_hash() && lock.hash_type() == rgbpp_lock.hash_type()
        {
            continue;
        }

        if lock.code_hash() == config.bitcoin_time_lock_type_hash()
            && lock.hash_type() == ScriptHashType::Type.into()
        {
            continue;
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

/// Verify input RGB++ cell is generated from previous RGB++ transaction
fn verify_input_rgbpp_cell(unlock_witness: &RGBPPUnlock) {
    // Fetch previous CKB transaction
    let previous_ckb_transaction = unlock_witness.previous_ckb_transaction();
    let raw_bitcoin_transaction = unlock_witness.bitcoin_transaction().raw_data();
    let bitcoin_transaction: BitcoinTransaction =
        parse_bitcoin_transaction(&raw_bitcoin_transaction);

    // Ensure inputs is from prev ckb transaction
    let prev_ckb_transaction_hash = previous_ckb_transaction.calc_witness_hash();
    let all_valid = QueryIter::new(load_input_out_point, Source::GroupInput)
        .all(|input| input.tx_hash() == prev_ckb_transaction_hash);
    if !all_valid {
        panic!("not all inputs is from the prev CKB transaction");
    }

    // Fetch prev bitcoin transaction from the prev CKB transaction.
    //
    // Now we face two branches:
    //
    // 1. Successed to fetch a prev bitcoin transaction.
    //  Since prev CKB transaction is already unlocked,
    //  so it is proved prev bitcoin transaction is exist,
    //  but we cannot prove it is right so we need to verify the commitment.
    // 2. Failed to find a prev bitcoin transaction.
    //  Which means this this a new created RGB++ cells.

    let prev_raw_bitcoin_transaction =
        fetch_bitcoin_transaction_from_ckb_transaction(&previous_ckb_transaction);
    let prev_bitcoin_transaction: BitcoinTransaction =
        parse_bitcoin_transaction(&prev_raw_bitcoin_transaction);

    // Check previous bitcoin transaction commitment
    check_bitcoin_transaction_commitment(&prev_bitcoin_transaction);
}

/// Verify unlock
fn verify_unlock(lock_args: &RGBPPLock, unlock_witness: &RGBPPUnlock) {
    // parse bitcoin transaction
    let raw_bitcoin_transaction = unlock_witness.bitcoin_transaction().raw_data();
    let bitcoin_transaction: BitcoinTransaction =
        parse_bitcoin_transaction(&raw_bitcoin_transaction);

    // check bitcoin transaction inputs unlock RGB++ cell
    let expected_out_point: (Byte32, u32) = (
        lock_args.bitcoin_transaction_hash(),
        lock_args.out_index().unpack(),
    );
    let is_found = bitcoin_transaction
        .inputs
        .iter()
        .any(|out_point| out_point == &expected_out_point);
    if !is_found {
        panic!("Bitcoin transaction doesn't unlock this cell");
    }

    // check bitcoin transaction exists in light client
    let bitcoin_transaction_hash = calc_bitcoin_hash(&raw_bitcoin_transaction);
    let is_exists = check_bitcoin_transaction_exists(bitcoin_transaction_hash);
    if !is_exists {
        panic!("Bitcoin transaction doesn't exists in the light client");
    }

    // verify commitment
    check_bitcoin_transaction_commitment(&bitcoin_transaction);
}

fn check_bitcoin_transaction_commitment(bitcoin_transaction: &BitcoinTransaction) {
    todo!()
}

/// Check light client cell
fn check_bitcoin_transaction_exists(bitcoin_transaction_hash: Byte32) -> bool {
    todo!()
}

struct BitcoinTransaction {
    pub inputs: Vec<(Byte32, u32)>,
}

fn parse_bitcoin_transaction(raw_bitcoin_transaction: &Bytes) -> BitcoinTransaction {
    todo!()
}

fn calc_bitcoin_hash(bitcoin_transaction: &Bytes) -> Byte32 {
    todo!()
}

fn fetch_bitcoin_transaction_from_ckb_transaction(previous_ckb_transaction: &Transaction) -> Bytes {
    todo!()
}
