#![no_std]
#![cfg_attr(not(test), no_main)]

#[cfg(test)]
extern crate alloc;

#[cfg(not(test))]
use ckb_std::default_alloc;
use ckb_std::{
    ckb_constants::Source,
    ckb_types::prelude::*,
    error::SysError,
    high_level::{
        load_cell, load_cell_data_hash, load_cell_lock_hash, load_script, load_script_hash,
        load_transaction, load_witness_args, QueryIter,
    },
};
use rgbpp_core::{
    ensure_eq,
    error::Error,
    on_chain::{bitcoin_light_client::check_btc_tx_exists, utils::load_config},
    schemas::rgbpp::*,
};
#[cfg(not(test))]
ckb_std::entry!(program_entry);
#[cfg(not(test))]
default_alloc!();

pub fn program_entry() -> i8 {
    match main() {
        Ok(_) => 0,
        Err(err) => {
            let err_code = err as i8;
            ckb_std::debug!("failed because {}", err_code);
            err_code
        }
    }
}

fn main() -> Result<(), Error> {
    let lock_args = load_lock_args()?;
    let ckb_tx = load_transaction()?;
    check_output_cells(&lock_args)?;
    let config = load_config::<BTCTimeLockConfig>(&ckb_tx)?;
    let unlock_witness = fetch_unlock_from_witness()?;
    let btc_tx_proof = unlock_witness.btc_tx_proof().raw_data();
    check_btc_tx_exists(
        &config.btc_lc_type_hash(),
        &lock_args.btc_txid(),
        lock_args.after().unpack(),
        &btc_tx_proof,
    )?;
    Ok(())
}

fn load_lock_args() -> Result<BTCTimeLock, SysError> {
    let script = load_script()?;
    let lock = BTCTimeLock::from_slice(&script.args().raw_data()).expect("parse BTCTimeLock");
    Ok(lock)
}

fn fetch_unlock_from_witness() -> Result<BTCTimeUnlock, Error> {
    let witness_args = load_witness_args(0, Source::GroupInput)?;
    match witness_args.lock().to_opt() {
        Some(args) => {
            let unlock =
                BTCTimeUnlock::from_slice(&args.raw_data()).map_err(|_| Error::BadBTCTimeUnlock)?;
            Ok(unlock)
        }
        None => Err(Error::ItemMissing),
    }
}

fn check_output_cells(lock_args: &BTCTimeLock) -> Result<(), Error> {
    let script_hash = load_script_hash()?;
    // iter btc time lock inputs
    let time_lock_iter = QueryIter::new(load_cell_lock_hash, Source::Input)
        .enumerate()
        .filter_map(|(index, lock_hash)| {
            if script_hash == lock_hash {
                Some(index)
            } else {
                None
            }
        });
    // check corresponded outputs
    let expected_output_lock = lock_args.lock_script();
    for index in time_lock_iter {
        let input_cell = load_cell(index, Source::Input)?;
        let output_cell = load_cell(index, Source::Output)?;
        ensure_eq!(
            expected_output_lock,
            output_cell.lock(),
            Error::OutputCellMismatch
        );

        ensure_eq!(
            input_cell.capacity(),
            output_cell.capacity(),
            Error::OutputCellMismatch
        );

        ensure_eq!(
            input_cell.type_(),
            output_cell.type_(),
            Error::OutputCellMismatch
        );

        let input_cell_data = load_cell_data_hash(index, Source::Input)?;
        let output_cell_data = load_cell_data_hash(index, Source::Output)?;
        ensure_eq!(input_cell_data, output_cell_data, Error::OutputCellMismatch);
    }
    Ok(())
}
