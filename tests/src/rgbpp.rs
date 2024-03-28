use ckb_testtool::ckb_types::core::{DepType, TransactionView};
use ckb_testtool::ckb_types::{bytes::Bytes, packed::*, prelude::*};
use ckb_testtool::context::Context;
use rand::random;
use rgbpp_core::bitcoin::{encode_btc_tx, sha2, Digest, Sha256, MIN_BTC_TIME_LOCK_AFTER};
use rgbpp_core::schemas::rgbpp::{Uint16, *};

use crate::btc_mock::{gen_seal, open_seal_tx};
use crate::utils::{create_sudt, TestScripts};

fn gen_commitment(rgbpp_tx: &TransactionView, extra_data: &ExtraCommitmentData) -> [u8; 32] {
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
        hasher.update(output.as_slice());
        hasher.update((data.len() as u32).to_le_bytes());
        hasher.update(&data);
    }
    sha2(&hasher.finalize())
}

pub fn build_rgbpp_tx(context: &mut Context, scripts: &TestScripts) -> TransactionView {
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
        let data = 1000u128.to_le_bytes().to_vec().into();
        let cell = CellOutput::new_builder()
            .lock(rgbpp_lock)
            .type_(Some(type_.clone()).pack())
            .build();
        let out_point = context.create_cell(cell, data);

        // outputs
        let btc_txid = Default::default();
        let new_rgbpp_lock = scripts.app.rgbpp.build_lock(btc_txid, new_seal_out_index);

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
        let new_rgbpp_lock = scripts
            .app
            .rgbpp
            .build_lock(btc_txid.pack(), new_seal_out_index);
        let output = output.as_builder().lock(new_rgbpp_lock).build();
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

pub fn build_btc_time_lock_tx(context: &mut Context, scripts: &TestScripts) -> TransactionView {
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
    let data = 1000u128.to_le_bytes().to_vec().into();
    let cell = CellOutput::new_builder()
        .lock(lock)
        .type_(Some(type_.clone()).pack())
        .build();
    let out_point = context.create_cell(cell, data);

    // outputs
    let data1 = 1000u128.to_le_bytes().to_vec().pack();
    let cell1 = CellOutput::new_builder()
        .lock(user_lock.clone())
        .type_(Some(type_.clone()).pack())
        .build();

    // unlock cell with btc tx
    let unlock = BTCTimeUnlock::new_builder()
        .btc_tx_proof(Default::default())
        .build();

    let unlock_witness = WitnessArgs::new_builder()
        .lock(Some(unlock.as_bytes()).pack())
        .build();

    tx_builder
        .input(CellInput::new_builder().previous_output(out_point).build())
        .outputs(vec![cell1])
        .outputs_data(vec![data1])
        .witness(unlock_witness.as_bytes().pack())
        .cell_dep(
            CellDep::new_builder()
                .out_point(scripts.app.btc_time_lock.config_out_point.clone())
                .dep_type(DepType::Code.into())
                .build(),
        )
        .build()
}
