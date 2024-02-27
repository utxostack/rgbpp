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
        packed::Byte32,
        prelude::{Entity, Unpack},
    },
    error::SysError,
    high_level::{load_witness, load_witness_args},
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
    let unlock_witness = fetch_unlock_from_witness()?;
    verify_this_rgbpp_cell(&unlock_witness);
    verify_unlock(&lock_args, &unlock_witness);
    Ok(())
}

fn fetch_unlock_from_witness() -> Result<RGBPPUnlock, SysError> {
    let witness_args = load_witness_args(0, Source::GroupInput)?;
    match witness_args.lock().to_opt() {
        Some(args) => {
            let unlock = RGBPPUnlock::from_slice(&args).unwrap();
            Ok(unlock)
        }
        None => Err(SysError::ItemMissing),
    }
}

/// Verify this RGB++ cell
fn verify_this_rgbpp_cell(unlock_witness: &RGBPPUnlock) {
    // 1. fetch previous_CKB_tx
    let previous_ckb_transaction = unlock_witness.previous_ckb_transaction();
    // 2. fetch previous bitcoin tx
    //     1. tx_hash is valid
    //     2. inputs == lock utxo
    //     3. btc_tx is a successor of previous_bitcoin_tx
    //     4. commitment is valid
    todo!()
}

/// Verify unlock
fn verify_unlock(lock_args: &RGBPPLock, unlock_witness: &RGBPPUnlock) {
    // parse bitcoin transaction
    let raw_bitcoin_transaction = unlock_witness.bitcoin_transaction();
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
        .any(|out_point| out_point == expected_out_point);
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

fn check_bitcoin_transaction_commitment(bitcoin_transaction: &BitcoinTransaction) -> _ {
    todo!()
}

/// Check light client cell
fn check_bitcoin_transaction_exists(bitcoin_transaction_hash: Byte32) -> _ {
    todo!()
}

struct BitcoinTransaction {
    pub inputs: Vec<(Byte32, u32)>,
}

fn parse_bitcoin_transaction(
    raw_bitcoin_transaction: &ckb_std::ckb_types::packed::Bytes,
) -> BitcoinTransaction {
    todo!()
}

fn calc_bitcoin_hash(bitcoin_transaction: &ckb_std::ckb_types::packed::Bytes) -> Byte32 {
    todo!()
}
