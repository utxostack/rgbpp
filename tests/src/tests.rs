use ckb_testtool::ckb_types::{
    bytes::Bytes,
    core::{ScriptHashType, TransactionBuilder},
    packed::*,
    prelude::*,
};
use ckb_testtool::context::Context;
use rgbpp_core::schemas::rgbpp::{Uint16, *};

use crate::{verify_and_dump_failed_tx, Loader};

const MAX_CYCLES: u64 = 10_000_000;

pub struct BTCLCScripts {
    pub btc_lc: OutPoint,
    pub btc_lc_type_hash: Byte32,
}

pub struct RGBPPScripts {
    pub script_out_point: OutPoint,
    pub config_out_point: OutPoint,
    pub type_hash: Byte32,
}

impl RGBPPScripts {
    pub fn build_lock(&self, btc_txid: Byte32, out_index: u32) -> Script {
        let args = RGBPPLock::new_builder()
            .btc_txid(btc_txid)
            .out_index(out_index.pack())
            .build();
        Script::new_builder()
            .code_hash(self.type_hash.clone())
            .hash_type(ScriptHashType::Type.into())
            .args(args.as_bytes().pack())
            .build()
    }
}

pub struct BTCTimeLockScripts {
    pub script_out_point: OutPoint,
    pub config_out_point: OutPoint,
    pub type_hash: Byte32,
}

pub struct DeployedScripts {
    pub rgbpp: RGBPPScripts,
    pub btc_time_lock: BTCTimeLockScripts,
}

pub struct SudtScripts {
    pub sudt_scripts: OutPoint,
    pub sudt_type_hash: Byte32,
}

pub struct Sudt {
    pub owner_script: Script,
}

impl Sudt {
    pub fn build_type(&self, sudt_scripts: &SudtScripts) -> Script {
        todo!()
    }
}

/// deploy scripts with config
/// return script out_point and config out_point
fn deploy_script_with_config(
    loader: &Loader,
    context: &mut Context,
    script_bin: Bytes,
    config: Bytes,
) -> (OutPoint, OutPoint) {
    todo!()
}

pub fn deploy_mock_btc_lc_scripts(loader: &Loader, context: &mut Context) -> BTCLCScripts {
    todo!()
}

pub fn deploy_sudt_scripts(loader: &Loader, context: &mut Context) -> SudtScripts {
    todo!()
}

pub fn create_sudt(loader: &Loader, context: &mut Context, sudt_scripts: &SudtScripts) -> Sudt {
    todo!()
}

pub fn deploy_rgbpp_scripts(
    loader: &Loader,
    context: &mut Context,
    btc_lc_type_hash: Byte32,
) -> DeployedScripts {
    // load scripts
    let btc_time_lock_bin = loader.load_binary("btc-time-lock");
    let rgbpp_lock_bin = loader.load_binary("rgbpp-lock");
    // deploy btc time lock with config
    let config = BTCTimeLockConfig::new_builder()
        .btc_lc_type_hash(btc_lc_type_hash.clone())
        .build();
    let (btc_time_lock, btc_time_lock_config) =
        deploy_script_with_config(loader, context, btc_time_lock_bin, config.as_bytes());
    let btc_time_lock_type_hash = {
        let (output, _data) = context.get_cell(&btc_time_lock).unwrap();
        output.type_().to_opt().unwrap().calc_script_hash()
    };

    let btc_time_lock = BTCTimeLockScripts {
        script_out_point: btc_time_lock,
        config_out_point: btc_time_lock_config,
        type_hash: btc_time_lock_type_hash.clone(),
    };
    // deploy rgbpp with config
    let config = RGBPPConfig::new_builder()
        .btc_lc_type_hash(btc_lc_type_hash)
        .btc_time_lock_type_hash(btc_time_lock_type_hash.clone())
        .build();
    let (rgbpp_lock, rgbpp_lock_config) =
        deploy_script_with_config(loader, context, rgbpp_lock_bin, config.as_bytes());

    let rgbpp_lock_type_hash = {
        let (output, _data) = context.get_cell(&rgbpp_lock).unwrap();
        output.type_().to_opt().unwrap().calc_script_hash()
    };
    let rgbpp = RGBPPScripts {
        script_out_point: rgbpp_lock,
        config_out_point: rgbpp_lock_config,
        type_hash: rgbpp_lock_type_hash,
    };
    DeployedScripts {
        rgbpp,
        btc_time_lock,
    }
}

#[test]
fn test_rgbpp_unlock() {
    let loader = Loader::default();
    let mut context = Context::default();

    let btc_lc_scripts = deploy_mock_btc_lc_scripts(&loader, &mut context);
    let deployed_scripts = deploy_rgbpp_scripts(
        &loader,
        &mut context,
        btc_lc_scripts.btc_lc_type_hash.clone(),
    );
    let sudt_scripts = deploy_sudt_scripts(&loader, &mut context);

    // mock btc tx
    let prev_btc_txid = todo!();
    let prev_out_index = todo!();
    let btc_tx = todo!();
    let btc_txid = todo!();
    let btc_tx_proof = todo!();
    let out_index = todo!();

    // prepare a rgbpp tx
    let rgbpp_tx = {
        let tx_builder = Transaction::default().as_advanced_builder();

        let token_a = create_sudt(&loader, &mut context, &sudt_scripts);
        let type_ = token_a.build_type(&sudt_scripts);

        // inputs
        let rgbpp_lock = deployed_scripts
            .rgbpp
            .build_lock(prev_btc_txid, prev_out_index);
        let data = 1000u128.to_le_bytes().to_vec().into();
        let cell = CellOutput::new_builder()
            .lock(rgbpp_lock)
            .type_(Some(type_).pack())
            .build();
        let out_point = context.create_cell(cell, data);

        // outputs
        let new_rgbpp_lock = deployed_scripts.rgbpp.build_lock(btc_txid, out_index);

        let data1 = 300u128.to_le_bytes().to_vec().pack();
        let cell1 = CellOutput::new_builder()
            .lock(new_rgbpp_lock.clone())
            .type_(Some(type_).pack())
            .build();

        let data2 = 700u128.to_le_bytes().to_vec().pack();
        let cell2 = CellOutput::new_builder()
            .lock(new_rgbpp_lock.clone())
            .type_(Some(type_).pack())
            .build();

        tx_builder
            .input(CellInput::new_builder().previous_output(out_point).build())
            .outputs(vec![cell1, cell2])
            .outputs_data(vec![data1, data2])
            .build()
    };

    // unlock cell with btc tx
    let extra_data = ExtraCommitmentData::new_builder()
        .input_len((rgbpp_tx.inputs().len() as u8).into())
        .output_len((rgbpp_tx.outputs().len() as u8).into())
        .build();
    let unlock_witness = RGBPPUnlock::new_builder()
        .version(Uint16::default())
        .extra_data(extra_data)
        .btc_tx(btc_tx)
        .btc_tx_proof(btc_tx_proof)
        .build();

    let tx = rgbpp_tx
        .as_advanced_builder()
        .witness(
            WitnessArgs::new_builder()
                .lock(Some(unlock_witness.as_bytes()).pack())
                .build()
                .as_bytes()
                .pack(),
        )
        .build();

    let tx = context.complete_tx(tx);

    verify_and_dump_failed_tx(&context, &tx, MAX_CYCLES).expect("pass");
}
