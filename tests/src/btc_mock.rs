use ckb_testtool::ckb_types::prelude::Pack;
use rand::{thread_rng, Rng};
use rgbpp_core::bitcoin::{BTCTx, TxIn, TxOut};

pub struct Seal {
    pub txid: [u8; 32],
    pub out_index: u32,
}

pub fn gen_seal(out_index: u32) -> Seal {
    let mut rng = thread_rng();
    let txid = rng.gen();
    Seal { txid, out_index }
}

pub fn open_seal_tx(prev_seal: Seal, commitment: [u8; 32]) -> BTCTx {
    let mut rng = thread_rng();
    let txid: [u8; 32] = rng.gen();
    let mut inputs = Vec::new();
    let mut outputs = Vec::new();

    let mock_script: [u8; 32] = rng.gen();

    // open prev seal utxo
    inputs.push(TxIn {
        previous_output: (prev_seal.txid.pack(), prev_seal.out_index),
        script: mock_script.to_vec().into(),
        sequence: 0,
    });

    // gen commitment output
    outputs.push(TxOut::new_seal(500, commitment));

    // gen new seal utxo
    let mock_pk_script: [u8; 32] = rng.gen();
    outputs.push(TxOut {
        value: 500,
        script: mock_pk_script.to_vec().into(),
    });

    // gen mocked btc tx
    BTCTx {
        txid: txid.pack(),
        lock_time: 0,
        version: 0,
        inputs,
        outputs,
    }
}
