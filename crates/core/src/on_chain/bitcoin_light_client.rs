use ckb_bitcoin_spv_verifier::types::packed::TransactionProofReader;
use ckb_gen_types::{packed::*, prelude::*};
use ckb_std::{ckb_constants::Source, error::SysError, high_level::*};

/// Check light client cell
pub fn check_btc_tx_exists(
    btc_lc_type_hash: &Byte32,
    btc_txid: &Byte32,
    confirmations: u32,
    tx_proof: &[u8],
) -> Result<bool, SysError> {
    let tx_proof = TransactionProofReader::from_slice(tx_proof).unwrap();
    let index = QueryIter::new(load_cell_type_hash, Source::CellDep)
        .enumerate()
        .find_map(|(index, type_hash)| {
            if type_hash.is_some_and(|type_hash| type_hash == btc_lc_type_hash.as_slice()) {
                Some(index)
            } else {
                None
            }
        })
        .expect("can't find light client cell");
    let data = load_cell_data(index, Source::CellDep)?;
    let client = ckb_bitcoin_spv_verifier::types::packed::SpvClient::from_slice(&data)
        .expect("Bitcoin SPV Client");
    let txid: [u8; 32] = btc_txid.as_slice().try_into().unwrap();
    match client.verify_transaction(&txid, tx_proof, confirmations) {
        Ok(_) => Ok(true),
        Err(err) => {
            ckb_std::debug!("failed to do SPV verification err: {}", err as i8);
            Ok(false)
        }
    }
}
