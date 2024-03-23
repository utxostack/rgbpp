use ckb_gen_types::core::*;
use ckb_gen_types::packed::*;
use ckb_gen_types::prelude::*;
use ckb_std::ckb_constants::Source;
use ckb_std::high_level::*;

use crate::error::Error;

#[derive(Eq, PartialEq)]
pub enum DepType {
    Code = 0,
    DepGroup = 1,
}

fn byte_to_dep_type(v: u8) -> Option<DepType> {
    match v {
        0 => Some(DepType::Code),
        1 => Some(DepType::DepGroup),
        _ => None,
    }
}

fn byte_to_script_hash_type(v: u8) -> Option<ScriptHashType> {
    match v {
        0 => Some(ScriptHashType::Data),
        1 => Some(ScriptHashType::Type),
        2 => Some(ScriptHashType::Data1),
        4 => Some(ScriptHashType::Data2),
        _ => None,
    }
}

// get current script cell dep
fn get_script_cell_dep(raw_tx: &RawTransaction, script: &Script) -> Result<CellDep, Error> {
    let script_hash_type: ScriptHashType =
        byte_to_script_hash_type(script.hash_type().into()).expect("parse script hash type");
    // look up script dep cell
    let dep_index = look_for_dep_with_hash2(script.code_hash().as_slice(), script_hash_type)
        .map_err(|_| Error::ConfigNotFound)?;
    // script dep cell and config cell must located in front of all dep groups
    let all_dep_is_code = raw_tx
        .cell_deps()
        .into_iter()
        .take(dep_index + 1)
        .all(|cell_dep| {
            byte_to_dep_type(cell_dep.dep_type().into())
                .is_some_and(|dep_type| dep_type == DepType::Code)
        });
    if !all_dep_is_code {
        return Err(Error::ConfigNotFound);
    }

    let script_cell_dep = raw_tx
        .cell_deps()
        .get(dep_index)
        .ok_or(Error::ConfigNotFound)?;
    Ok(script_cell_dep)
}

fn get_config_cell_dep_index(
    raw_tx: &RawTransaction,
    script_cell_dep: &CellDep,
) -> Result<usize, Error> {
    for (index, cell_dep) in raw_tx.cell_deps().into_iter().enumerate() {
        // script dep cell and config cell must located in front of all dep groups
        if !byte_to_dep_type(cell_dep.dep_type().into())
            .is_some_and(|dep_type| dep_type == DepType::Code)
        {
            return Err(Error::ConfigNotFound);
        }
        let cell_index: u32 = cell_dep.out_point().index().unpack();
        if cell_index == 1
            && cell_dep.out_point().tx_hash() == script_cell_dep.out_point().tx_hash()
        {
            return Ok(index);
        }
    }
    Err(Error::ConfigNotFound)
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
    let raw_tx = tx.raw();
    let script_cell_dep = get_script_cell_dep(&raw_tx, &script)?;
    let script_out_point_index: u32 = script_cell_dep.out_point().index().unpack();
    if script_out_point_index != 0 {
        // script cell with config must at 0 index
        return Err(Error::ConfigMalformed);
    }
    // look up config dep cell
    let config_cell_dep_index = get_config_cell_dep_index(&raw_tx, &script_cell_dep)?;
    let data = load_cell_data(config_cell_dep_index, Source::CellDep)
        .map_err(|_| Error::ConfigNotFound)?;
    let config = Config::from_slice(&data).map_err(|_| Error::ConfigMalformed)?;
    Ok(config)
}
