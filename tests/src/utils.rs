use ckb_testtool::context::Context;
use ckb_testtool::{
    ckb_types::{bytes::Bytes, core::ScriptHashType, packed::*, prelude::*},
    context::random_type_id_script,
};
use rand::{thread_rng, Rng};
use rgbpp_core::schemas::rgbpp::*;

use crate::Loader;

pub struct TestScripts {
    pub app: DeployedScripts,
    pub btc_spv: BTCLCScripts,
    pub sudt: SudtScripts,
}

impl TestScripts {
    pub fn setup(loader: &Loader, context: &mut Context) -> Self {
        let btc_spv = deploy_mock_btc_lc_scripts(loader, context);
        let app = deploy_rgbpp_scripts(loader, context, btc_spv.btc_lc_type_hash.clone());
        let sudt = deploy_sudt_scripts(loader, context);
        TestScripts { app, btc_spv, sudt }
    }
}

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

impl BTCTimeLockScripts {
    pub fn build_lock(&self, lock_script: Script, after: u32, btc_txid: Byte32) -> Script {
        let args = BTCTimeLock::new_builder()
            .lock_script(lock_script)
            .after(after.pack())
            .btc_txid(btc_txid)
            .build();
        Script::new_builder()
            .code_hash(self.type_hash.clone())
            .hash_type(ScriptHashType::Type.into())
            .args(args.as_bytes().pack())
            .build()
    }
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
        Script::new_builder()
            .code_hash(sudt_scripts.sudt_type_hash.clone())
            .hash_type(ScriptHashType::Type.into())
            .args(self.owner_script.calc_script_hash().raw_data().pack())
            .build()
    }
}

fn create_cells(context: &mut Context, cells: Vec<(CellOutput, Bytes)>) -> Byte32 {
    let mut rng = thread_rng();
    let tx_hash: [u8; 32] = rng.gen();
    for (index, (cell, data)) in cells.into_iter().enumerate() {
        let out_point = OutPoint::new_builder()
            .tx_hash(tx_hash.pack())
            .index((index as u32).pack())
            .build();
        context.create_cell_with_out_point(out_point, cell, data);
    }
    tx_hash.pack()
}

/// deploy scripts with config
/// return script out_point and config out_point
fn deploy_script_with_config(
    context: &mut Context,
    script_bin: Bytes,
    config: Bytes,
) -> (OutPoint, OutPoint) {
    let cell1 = CellOutput::new_builder()
        .type_(Some(random_type_id_script()).pack())
        .build();
    let cell2 = CellOutput::new_builder()
        .type_(Some(random_type_id_script()).pack())
        .build();
    let tx_hash = create_cells(context, vec![(cell1, script_bin), (cell2, config)]);
    (
        OutPoint::new_builder()
            .tx_hash(tx_hash.clone())
            .index(0u32.pack())
            .build(),
        OutPoint::new_builder()
            .tx_hash(tx_hash.clone())
            .index(1u32.pack())
            .build(),
    )
}

pub fn deploy_mock_btc_lc_scripts(loader: &Loader, context: &mut Context) -> BTCLCScripts {
    let btc_lc = {
        let data = loader.load_tests_binary("always_success");
        context.deploy_cell(data)
    };
    let btc_lc_type_hash = {
        context
            .get_cell(&btc_lc)
            .unwrap()
            .0
            .type_()
            .to_opt()
            .unwrap()
            .calc_script_hash()
    };
    BTCLCScripts {
        btc_lc,
        btc_lc_type_hash,
    }
}

pub fn deploy_sudt_scripts(loader: &Loader, context: &mut Context) -> SudtScripts {
    let sudt_scripts = {
        let data = loader.load_tests_binary("simple_udt");
        context.deploy_cell(data)
    };
    let sudt_type_hash = {
        context
            .get_cell(&sudt_scripts)
            .unwrap()
            .0
            .type_()
            .to_opt()
            .unwrap()
            .calc_script_hash()
    };
    SudtScripts {
        sudt_scripts,
        sudt_type_hash,
    }
}

/// Return a random Sudt
pub fn create_sudt() -> Sudt {
    let mut rng = thread_rng();
    let random_args: [u8; 32] = rng.gen();
    let owner_script = Script::new_builder()
        .args(random_args.to_vec().pack())
        .build();
    Sudt { owner_script }
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
        deploy_script_with_config(context, btc_time_lock_bin, config.as_bytes());
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
        deploy_script_with_config(context, rgbpp_lock_bin, config.as_bytes());

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
