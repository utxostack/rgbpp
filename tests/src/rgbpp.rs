use ckb_testtool::ckb_error::Error;
use ckb_testtool::ckb_types::core::{DepType, TransactionView};
use ckb_testtool::ckb_types::{bytes::Bytes, packed::*, prelude::*};
use ckb_testtool::context::Context;
use rand::random;
use rgbpp_core::bitcoin::{encode_btc_tx, sha2, Digest, Sha256, MIN_BTC_TIME_LOCK_AFTER};
use rgbpp_core::error::Error as RgbppError;
use rgbpp_core::schemas::rgbpp::{Uint16, *};

use crate::btc_mock::{gen_seal, open_seal_tx};
use crate::utils::{create_sudt, TestScripts};

#[derive(Debug, Clone, Default)]
pub struct BtcTimeLockDesc {
    pub user_lock_opt: Option<Script>,
    pub after: u32,
    pub btc_txid_opt: Option<Byte32>,
}

impl BtcTimeLockDesc {
    pub fn with_after(after: u32) -> Self {
        Self {
            after,
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct UserLockDesc {
    pub lock_opt: Option<Script>,
}

#[derive(Debug, Clone)]
pub enum LockDesc {
    Rgbpp,
    BtcTimeLock(BtcTimeLockDesc),
    UserLock(UserLockDesc),
}

#[derive(Debug, Clone)]
pub struct OutputDesc {
    pub lock: LockDesc,
    pub amount: u128,
}

fn set_cell_btc_txid(scripts: &TestScripts, output: CellOutput, btc_txid: Byte32) -> CellOutput {
    let lock = output.lock();
    match lock.code_hash() {
        code_hash if code_hash == scripts.app.rgbpp.type_hash => {
            let lock_args = RGBPPLock::from_slice(&lock.args().raw_data())
                .unwrap()
                .as_builder()
                .btc_txid(btc_txid.clone())
                .build();
            let lock = lock.as_builder().args(lock_args.as_bytes().pack()).build();
            output.as_builder().lock(lock).build()
        }
        code_hash if code_hash == scripts.app.btc_time_lock.type_hash => {
            let lock_args = BTCTimeLock::from_slice(&lock.args().raw_data())
                .unwrap()
                .as_builder()
                .btc_txid(btc_txid.clone())
                .build();
            let lock = lock.as_builder().args(lock_args.as_bytes().pack()).build();
            output.as_builder().lock(lock).build()
        }
        _ => output,
    }
}

fn clear_cell_btc_txid(scripts: &TestScripts, output: CellOutput) -> CellOutput {
    let lock = output.lock();
    match lock.code_hash() {
        code_hash if code_hash == scripts.app.rgbpp.type_hash => {
            let lock_args = RGBPPLock::from_slice(&lock.args().raw_data())
                .unwrap()
                .as_builder()
                .btc_txid(Byte32::default())
                .build();
            let lock = lock.as_builder().args(lock_args.as_bytes().pack()).build();
            output.as_builder().lock(lock).build()
        }
        code_hash if code_hash == scripts.app.btc_time_lock.type_hash => {
            let lock_args = BTCTimeLock::from_slice(&lock.args().raw_data())
                .unwrap()
                .as_builder()
                .btc_txid(Byte32::default())
                .build();
            let lock = lock.as_builder().args(lock_args.as_bytes().pack()).build();
            output.as_builder().lock(lock).build()
        }
        _ => output,
    }
}

fn gen_commitment(
    scripts: &TestScripts,
    rgbpp_tx: &TransactionView,
    extra_data: &ExtraCommitmentData,
) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(b"RGB++");
    // version
    hasher.update([0, 0]);
    let input_len = u8::from(extra_data.input_len());
    let output_len = u8::from(extra_data.output_len());
    hasher.update([input_len, output_len]);

    for out_point in rgbpp_tx.input_pts_iter().take(input_len as usize) {
        hasher.update(out_point.as_slice());
    }

    for (output, data) in rgbpp_tx.outputs_with_data_iter().take(output_len as usize) {
        let output = clear_cell_btc_txid(scripts, output);
        hasher.update(output.as_slice());
        hasher.update((data.len() as u32).to_le_bytes());
        hasher.update(&data);
    }
    sha2(&hasher.finalize())
}

pub fn build_rgbpp_tx(
    context: &mut Context,
    scripts: &TestScripts,
    input_amount: u128,
    outputs_desc: Vec<OutputDesc>,
) -> TransactionView {
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
        let type_ = token_a.build_type(&scripts.sudt);

        // inputs
        let rgbpp_lock = scripts.app.rgbpp.build_lock(prev_btc_txid, prev_out_index);
        let data = input_amount.to_le_bytes().to_vec().into();
        let cell = CellOutput::new_builder()
            .lock(rgbpp_lock)
            .type_(Some(type_.clone()).pack())
            .build();
        let out_point = context.create_cell(cell, data);

        // outputs
        let empty_btc_txid: Byte32 = Default::default();
        let new_rgbpp_lock = scripts
            .app
            .rgbpp
            .build_lock(empty_btc_txid.clone(), new_seal_out_index);

        let mut outputs = Vec::with_capacity(outputs_desc.len());
        let mut outputs_data = Vec::with_capacity(outputs_desc.len());

        for OutputDesc { lock, amount } in outputs_desc.clone() {
            let data = amount.to_le_bytes().to_vec().pack();

            let cell_lock = match lock {
                LockDesc::Rgbpp => new_rgbpp_lock.clone(),
                LockDesc::BtcTimeLock(BtcTimeLockDesc {
                    user_lock_opt,
                    after,
                    btc_txid_opt: _,
                }) => {
                    let user_lock = user_lock_opt.unwrap_or_else(|| {
                        Script::new_builder()
                            .args(b"mocked user lock".to_vec().pack())
                            .build()
                    });
                    scripts
                        .app
                        .btc_time_lock
                        .build_lock(user_lock, after, empty_btc_txid.clone())
                }
                LockDesc::UserLock(UserLockDesc { lock_opt }) => lock_opt.unwrap_or_else(|| {
                    Script::new_builder()
                        .args(b"mocked user lock".to_vec().pack())
                        .build()
                }),
            };
            let cell = CellOutput::new_builder()
                .lock(cell_lock)
                .type_(Some(type_.clone()).pack())
                .build();
            outputs.push(cell);
            outputs_data.push(data);
        }

        tx_builder
            .input(CellInput::new_builder().previous_output(out_point).build())
            .outputs(outputs)
            .outputs_data(outputs_data)
            .build()
    };

    let extra_data = ExtraCommitmentData::new_builder()
        .input_len((rgbpp_tx.inputs().len() as u8).into())
        .output_len((rgbpp_tx.outputs().len() as u8).into())
        .build();

    // mock open seal btc tx
    let commitment = gen_commitment(scripts, &rgbpp_tx, &extra_data);
    let btc_tx = encode_btc_tx(open_seal_tx(mock_seal, commitment));
    let btc_txid = sha2(&sha2(&btc_tx));
    let btc_tx_proof = Bytes::default().pack();

    // set btc_txid to output lock
    let mut outputs = Vec::new();
    for (output, desc) in rgbpp_tx
        .outputs()
        .into_iter()
        .take(u8::from(extra_data.output_len()) as usize)
        .zip(outputs_desc)
    {
        let btc_txid = if let LockDesc::BtcTimeLock(BtcTimeLockDesc {
            btc_txid_opt: Some(btc_txid),
            ..
        }) = desc.lock
        {
            // orverride btc_txid
            btc_txid
        } else {
            btc_txid.pack()
        };
        let output = set_cell_btc_txid(scripts, output, btc_txid);
        outputs.push(output);
    }
    let rgbpp_tx = rgbpp_tx.as_advanced_builder().set_outputs(outputs).build();

    // unlock cell with btc tx
    let rgbpp_unlock = RGBPPUnlock::new_builder()
        .version(Uint16::default())
        .extra_data(extra_data)
        .btc_tx(btc_tx.pack())
        .btc_tx_proof(btc_tx_proof)
        .build();

    let unlock_witness = WitnessArgs::new_builder()
        .lock(Some(rgbpp_unlock.as_bytes()).pack())
        .build();

    rgbpp_tx
        .as_advanced_builder()
        .witness(unlock_witness.as_bytes().pack())
        .cell_dep(
            CellDep::new_builder()
                .out_point(scripts.app.rgbpp.config_out_point.clone())
                .dep_type(DepType::Code.into())
                .build(),
        )
        .cell_dep(
            CellDep::new_builder()
                .out_point(scripts.app.btc_time_lock.config_out_point.clone())
                .dep_type(DepType::Code.into())
                .build(),
        )
        .build()
}

pub fn build_btc_time_lock_tx(
    context: &mut Context,
    scripts: &TestScripts,
    input_amount: u128,
    outputs_desc: Vec<OutputDesc>,
) -> TransactionView {
    // mock prev btc utxo seal
    let mocked_btc_txid: [u8; 32] = random();
    // new seal out index, index 0 is commitment utxo

    // prepare a rgbpp tx
    let tx_builder = Transaction::default().as_advanced_builder();

    let token_a = create_sudt();
    let type_ = token_a.build_type(&scripts.sudt);

    // inputs
    let user_lock = {
        Script::new_builder()
            .args(b"mock user lock".to_vec().pack())
            .build()
    };
    let after = MIN_BTC_TIME_LOCK_AFTER;
    let lock =
        scripts
            .app
            .btc_time_lock
            .build_lock(user_lock.clone(), after, mocked_btc_txid.pack());
    let data = input_amount.to_le_bytes().to_vec().into();
    let cell = CellOutput::new_builder()
        .lock(lock)
        .type_(Some(type_.clone()).pack())
        .build();
    let out_point = context.create_cell(cell, data);

    // outputs

    let mut outputs = Vec::with_capacity(outputs_desc.len());
    let mut outputs_data = Vec::with_capacity(outputs_desc.len());

    for OutputDesc { lock, amount } in outputs_desc.clone() {
        let data = amount.to_le_bytes().to_vec().pack();

        let cell_lock = match lock {
            LockDesc::Rgbpp | LockDesc::BtcTimeLock(..) => {
                panic!("Can't build RGBPP or BtcTimeLock");
            }
            LockDesc::UserLock(UserLockDesc { lock_opt }) => {
                lock_opt.unwrap_or_else(|| user_lock.clone())
            }
        };
        let cell = CellOutput::new_builder()
            .lock(cell_lock)
            .type_(Some(type_.clone()).pack())
            .build();
        outputs.push(cell);
        outputs_data.push(data);
    }

    // unlock cell with btc tx
    let unlock = BTCTimeUnlock::new_builder()
        .btc_tx_proof(Default::default())
        .build();

    let unlock_witness = WitnessArgs::new_builder()
        .lock(Some(unlock.as_bytes()).pack())
        .build();

    tx_builder
        .input(CellInput::new_builder().previous_output(out_point).build())
        .outputs(outputs)
        .outputs_data(outputs_data)
        .witness(unlock_witness.as_bytes().pack())
        .cell_dep(
            CellDep::new_builder()
                .out_point(scripts.app.btc_time_lock.config_out_point.clone())
                .dep_type(DepType::Code.into())
                .build(),
        )
        .build()
}

pub fn assert_script_error(err: Error, script_error: RgbppError) {
    let error_string = err.to_string();
    let code = script_error as i8;
    assert!(
        error_string.contains(format!("error code {}", code).as_str()),
        "error_string: {}, expected_error_code: {}",
        error_string,
        code
    );
}
