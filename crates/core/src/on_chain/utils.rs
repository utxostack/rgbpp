use ckb_gen_types::core::*;
use ckb_gen_types::packed::*;
use ckb_gen_types::prelude::*;
use ckb_std::ckb_constants::Source;
use ckb_std::high_level::*;

use crate::error::Error;

fn byte_to_script_hash_type(v: u8) -> Option<ScriptHashType> {
    match v {
        0 => Some(ScriptHashType::Data),
        1 => Some(ScriptHashType::Type),
        2 => Some(ScriptHashType::Data1),
        4 => Some(ScriptHashType::Data2),
        _ => None,
    }
}

/// Config cell is deployed together with the current contract
///
/// ``` yaml
/// contract_deployment_transaction:
///   - output(index=0, data=rgbpp_code)
///   - output(index=1, data=rgbpp_config)
/// ```
pub fn load_config<Config: Entity>(tx: &Transaction) -> Result<Config, Error> {
    // get current script
    let script = load_script().map_err(|_| Error::ConfigNotFound)?;
    let script_hash_type: ScriptHashType =
        byte_to_script_hash_type(script.hash_type().into()).expect("parse script hash type");
    // look up script dep cell
    let cell_dep_index = look_for_dep_with_hash2(script.code_hash().as_slice(), script_hash_type)
        .map_err(|_| Error::ConfigNotFound)?;
    let raw_tx = tx.raw();
    let script_cell_dep = raw_tx
        .cell_deps()
        .get(cell_dep_index)
        .ok_or(Error::ConfigNotFound)?;
    let script_out_point_index: u32 = script_cell_dep.out_point().index().unpack();
    if script_out_point_index != 0 {
        // script cell with config must at 0 index
        return Err(Error::ConfigMalformed);
    }
    // look up config dep cell
    let config_cell_dep_index = raw_tx
        .cell_deps()
        .into_iter()
        .enumerate()
        .find(|(_index, cell_dep)| {
            let index: u32 = cell_dep.out_point().index().unpack();
            index == 1 && cell_dep.out_point().tx_hash() == script_cell_dep.out_point().tx_hash()
        })
        .ok_or(Error::ConfigNotFound)?
        .0;
    let data = load_cell_data(config_cell_dep_index, Source::CellDep)
        .map_err(|_| Error::ConfigNotFound)?;
    let config = Config::from_slice(&data).map_err(|_| Error::ConfigMalformed)?;
    Ok(config)
}
