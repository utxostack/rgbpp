use crate::error::Error;
use ckb_bitcoin_spv_verifier::types::packed::TransactionProofReader;
use ckb_gen_types::{packed::*, prelude::*};
use ckb_std::{ckb_constants::Source, high_level::*};

/// Check light client cell
#[cfg(not(feature = "mock-bitcoin-light-client"))]
pub fn check_btc_tx_exists(
    btc_lc_type_hash: &Byte32,
    btc_txid: &Byte32,
    confirmations: u32,
    tx_proof: &[u8],
) -> Result<(), Error> {
    let tx_proof =
        TransactionProofReader::from_slice(tx_proof).map_err(|_| Error::SpvProofMalformed)?;
    let index = QueryIter::new(load_cell_type_hash, Source::CellDep)
        .enumerate()
        .find_map(|(index, type_hash)| {
            if type_hash.is_some_and(|type_hash| type_hash == btc_lc_type_hash.as_slice()) {
                Some(index)
            } else {
                None
            }
        })
        .ok_or(Error::SpvClientNotFound)?;
    let data = load_cell_data(index, Source::CellDep).map_err(|_| Error::SpvClientNotFound)?;
    let client = ckb_bitcoin_spv_verifier::types::packed::SpvClient::from_slice(&data)
        .map_err(|_| Error::SpvClientMalformed)?;
    let txid: [u8; 32] = btc_txid.as_slice().try_into().unwrap();
    match client.verify_transaction(&txid, tx_proof, confirmations) {
        Ok(_) => Ok(()),
        Err(err) => {
            ckb_std::debug!("failed to do SPV verification err: {}", err as i8);
            Err(Error::SpvProofIncorrect)
        }
    }
}

#[cfg(feature = "mock-bitcoin-light-client")]
pub fn check_btc_tx_exists(
    _btc_lc_type_hash: &Byte32,
    _btc_txid: &Byte32,
    _confirmations: u32,
    _tx_proof: &[u8],
) -> Result<(), Error> {
    ckb_std::debug!(
        "Using Mock Bitcoin light client, please ensure do not use this binary in production"
    );
    Ok(())
}
