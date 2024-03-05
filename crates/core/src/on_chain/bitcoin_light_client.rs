use ckb_gen_types::{packed::*, prelude::*};
use ckb_std::{ckb_constants::Source, error::SysError, high_level::*};

/// Check light client cell
/// TODO this is a mock implementation!!!
pub fn check_btc_tx_exists(
    btc_lc_type_hash: &Byte32,
    btc_txid: &Byte32,
    _confirmations: usize,
) -> Result<bool, SysError> {
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
    Ok(data == btc_txid.as_slice())
}
