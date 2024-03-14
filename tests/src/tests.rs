use ckb_testtool::ckb_types::core::TransactionView;
use ckb_testtool::context::Context;
use ckb_testtool::{
    ckb_types::{bytes::Bytes, core::ScriptHashType, packed::*, prelude::*},
    context::random_type_id_script,
};
use rand::{thread_rng, Rng};
use rgbpp_core::bitcoin::{encode_btc_tx, sha2, Digest, Sha256};
use rgbpp_core::schemas::rgbpp::{Uint16, *};

use crate::btc_mock::{gen_seal, open_seal_tx};
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

fn gen_commitment(rgbpp_tx: &TransactionView, extra_data: &ExtraCommitmentData) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(b"RGB++");
    // version
    hasher.update([0]);
    let input_len = u8::from(extra_data.input_len());
    let output_len = u8::from(extra_data.output_len());
    hasher.update([input_len, output_len]);

    for out_point in rgbpp_tx.input_pts_iter().take(input_len as usize) {
        hasher.update(out_point.as_slice());
    }

    for (output, data) in rgbpp_tx.outputs_with_data_iter().take(output_len as usize) {
        hasher.update(output.as_slice());
        hasher.update(&data);
    }
    hasher.finalize().into()
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

    // mock prev btc utxo seal
    let prev_out_index = 0;
    let mock_seal = gen_seal(prev_out_index);
    let prev_btc_txid = mock_seal.txid.pack();
    // new seal out index, index 0 is commitment utxo
    let new_seal_out_index = 1;

    // prepare a rgbpp tx
    let rgbpp_tx = {
        let tx_builder = Transaction::default().as_advanced_builder();

        let token_a = create_sudt();
        let type_ = token_a.build_type(&sudt_scripts);

        // inputs
        let rgbpp_lock = deployed_scripts
            .rgbpp
            .build_lock(prev_btc_txid, prev_out_index);
        let data = 1000u128.to_le_bytes().to_vec().into();
        let cell = CellOutput::new_builder()
            .lock(rgbpp_lock)
            .type_(Some(type_.clone()).pack())
            .build();
        let out_point = context.create_cell(cell, data);

        // outputs
        let btc_txid = Default::default();
        let new_rgbpp_lock = deployed_scripts
            .rgbpp
            .build_lock(btc_txid, new_seal_out_index);

        let data1 = 300u128.to_le_bytes().to_vec().pack();
        let cell1 = CellOutput::new_builder()
            .lock(new_rgbpp_lock.clone())
            .type_(Some(type_.clone()).pack())
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

    let extra_data = ExtraCommitmentData::new_builder()
        .input_len((rgbpp_tx.inputs().len() as u8).into())
        .output_len((rgbpp_tx.outputs().len() as u8).into())
        .build();

    // mock open seal btc tx
    let commitment = gen_commitment(&rgbpp_tx, &extra_data);
    let btc_tx = encode_btc_tx(open_seal_tx(mock_seal, commitment));
    let btc_txid = sha2(&sha2(&btc_tx));
    let btc_tx_proof = Bytes::default().pack();

    // set btc_txid to output lock
    let mut outputs = Vec::new();
    for output in rgbpp_tx
        .outputs()
        .into_iter()
        .take(u8::from(extra_data.output_len()) as usize)
    {
        let new_rgbpp_lock = deployed_scripts
            .rgbpp
            .build_lock(btc_txid.pack(), new_seal_out_index);
        let output = output.as_builder().lock(new_rgbpp_lock).build();
        outputs.push(output);
    }
    let rgbpp_tx = rgbpp_tx.as_advanced_builder().set_outputs(outputs).build();

    // unlock cell with btc tx
    let unlock_witness = RGBPPUnlock::new_builder()
        .version(Uint16::default())
        .extra_data(extra_data)
        .btc_tx(btc_tx.pack())
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
