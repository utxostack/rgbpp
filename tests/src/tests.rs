use crate::rgbpp::{
    assert_script_error, build_btc_time_lock_tx, build_rgbpp_tx, BtcTimeLockDesc, LockDesc,
    OutputDesc, UserLockDesc,
};
use crate::utils::TestScripts;
use crate::{verify_and_dump_failed_tx, Loader};
use ckb_testtool::context::Context;
use rand::random;
use rgbpp_core::bitcoin::MIN_BTC_TIME_LOCK_AFTER;
use rgbpp_core::error::Error as RgbppError;
use rgbpp_core::schemas::blockchain::{BytesOpt, Script, WitnessArgs};
use rgbpp_core::schemas::{
    blockchain::CellOutput,
    ckb_gen_types::{bytes::Bytes, prelude::*},
};

const MAX_CYCLES: u64 = 10_000_000;

#[test]
fn test_rgbpp_unlock() {
    let loader = Loader::default();
    let mut context = Context::default();

    let scripts = TestScripts::setup(&loader, &mut context);

    let tx = build_rgbpp_tx(
        &mut context,
        &scripts,
        1000,
        vec![
            OutputDesc {
                lock: LockDesc::Rgbpp,
                amount: 300,
            },
            OutputDesc {
                lock: LockDesc::Rgbpp,
                amount: 700,
            },
        ],
    );
    let tx = context.complete_tx(tx);

    verify_and_dump_failed_tx(&context, &tx, MAX_CYCLES).expect("pass");
}

#[test]
fn test_rgbpp_wrong_commitment() {
    let loader = Loader::default();
    let mut context = Context::default();

    let scripts = TestScripts::setup(&loader, &mut context);

    let tx = build_rgbpp_tx(
        &mut context,
        &scripts,
        1000,
        vec![
            OutputDesc {
                lock: LockDesc::Rgbpp,
                amount: 300,
            },
            OutputDesc {
                lock: LockDesc::Rgbpp,
                amount: 700,
            },
        ],
    );

    // modify committed cells
    let mut outputs: Vec<CellOutput> = tx.outputs().into_iter().collect();
    let capacity: u64 = outputs[0].capacity().unpack();
    outputs[0] = outputs[0]
        .to_owned()
        .as_builder()
        .capacity((capacity + 1).pack())
        .build();
    let tx = tx.as_advanced_builder().set_outputs(outputs).build();

    let tx = context.complete_tx(tx);

    let err = verify_and_dump_failed_tx(&context, &tx, MAX_CYCLES).expect_err("fail");
    assert_script_error(err, RgbppError::CommitmentMismatch);
}

#[test]
fn test_rgbpp_move_assets_to_ckb() {
    let loader = Loader::default();
    let mut context = Context::default();

    let scripts = TestScripts::setup(&loader, &mut context);

    let tx = build_rgbpp_tx(
        &mut context,
        &scripts,
        1000,
        vec![
            OutputDesc {
                lock: LockDesc::Rgbpp,
                amount: 300,
            },
            OutputDesc {
                lock: LockDesc::BtcTimeLock(BtcTimeLockDesc::with_after(MIN_BTC_TIME_LOCK_AFTER)),
                amount: 700,
            },
        ],
    );

    let tx = context.complete_tx(tx);

    verify_and_dump_failed_tx(&context, &tx, MAX_CYCLES).expect("pass");
}

#[test]
fn test_rgbpp_move_assets_to_ckb_with_longer_after() {
    let loader = Loader::default();
    let mut context = Context::default();

    let scripts = TestScripts::setup(&loader, &mut context);

    let tx = build_rgbpp_tx(
        &mut context,
        &scripts,
        1000,
        vec![
            OutputDesc {
                lock: LockDesc::Rgbpp,
                amount: 300,
            },
            OutputDesc {
                lock: LockDesc::BtcTimeLock(BtcTimeLockDesc::with_after(10)),
                amount: 700,
            },
        ],
    );

    let tx = context.complete_tx(tx);

    verify_and_dump_failed_tx(&context, &tx, MAX_CYCLES).expect("pass");
}

#[test]
fn test_rgbpp_move_assets_to_ckb_with_wrong_after() {
    let loader = Loader::default();
    let mut context = Context::default();

    let scripts = TestScripts::setup(&loader, &mut context);

    let tx = build_rgbpp_tx(
        &mut context,
        &scripts,
        1000,
        vec![
            OutputDesc {
                lock: LockDesc::Rgbpp,
                amount: 300,
            },
            OutputDesc {
                lock: LockDesc::BtcTimeLock(BtcTimeLockDesc::with_after(1)),
                amount: 700,
            },
        ],
    );

    let tx = context.complete_tx(tx);

    let err = verify_and_dump_failed_tx(&context, &tx, MAX_CYCLES).expect_err("fail");
    assert_script_error(err, RgbppError::OutputCellWithUnknownLock);
}

#[test]
fn test_rgbpp_move_assets_to_ckb_with_wrong_txid() {
    let loader = Loader::default();
    let mut context = Context::default();

    let scripts = TestScripts::setup(&loader, &mut context);

    let wrong_btc_txid: [u8; 32] = random();
    let tx = build_rgbpp_tx(
        &mut context,
        &scripts,
        1000,
        vec![
            OutputDesc {
                lock: LockDesc::Rgbpp,
                amount: 300,
            },
            OutputDesc {
                lock: LockDesc::BtcTimeLock(BtcTimeLockDesc {
                    after: MIN_BTC_TIME_LOCK_AFTER,
                    user_lock_opt: None,
                    btc_txid_opt: Some(wrong_btc_txid.pack()),
                }),
                amount: 700,
            },
        ],
    );

    let tx = context.complete_tx(tx);

    let err = verify_and_dump_failed_tx(&context, &tx, MAX_CYCLES).expect_err("fail");
    assert_script_error(err, RgbppError::OutputCellWithUnknownLock);
}

#[test]
fn test_rgbpp_move_assets_without_timelock() {
    let loader = Loader::default();
    let mut context = Context::default();

    let scripts = TestScripts::setup(&loader, &mut context);

    let tx = build_rgbpp_tx(
        &mut context,
        &scripts,
        1000,
        vec![
            OutputDesc {
                lock: LockDesc::Rgbpp,
                amount: 300,
            },
            OutputDesc {
                lock: LockDesc::UserLock(Default::default()),
                amount: 700,
            },
        ],
    );

    let tx = context.complete_tx(tx);

    let err = verify_and_dump_failed_tx(&context, &tx, MAX_CYCLES).expect_err("fail");
    assert_script_error(err, RgbppError::OutputCellWithUnknownLock);
}

#[test]
fn test_btc_time_lock() {
    let loader = Loader::default();
    let mut context = Context::default();
    let scripts = TestScripts::setup(&loader, &mut context);
    let tx = build_btc_time_lock_tx(
        &mut context,
        &scripts,
        1000,
        vec![OutputDesc {
            lock: LockDesc::UserLock(Default::default()),
            amount: 1000,
        }],
    );
    let tx = context.complete_tx(tx);
    verify_and_dump_failed_tx(&context, &tx, MAX_CYCLES).expect("pass");
}

#[test]
fn test_btc_time_lock_with_wrong_user_lock() {
    let loader = Loader::default();
    let mut context = Context::default();
    let scripts = TestScripts::setup(&loader, &mut context);
    let wrong_user_lock = {
        let code_hash: [u8; 32] = random();
        let args: [u8; 32] = random();
        Script::new_builder()
            .code_hash(code_hash.pack())
            .args(args.to_vec().pack())
            .build()
    };
    let tx = build_btc_time_lock_tx(
        &mut context,
        &scripts,
        1000,
        vec![OutputDesc {
            lock: LockDesc::UserLock(UserLockDesc {
                lock_opt: Some(wrong_user_lock),
            }),
            amount: 1000,
        }],
    );
    let tx = context.complete_tx(tx);
    let err = verify_and_dump_failed_tx(&context, &tx, MAX_CYCLES).expect_err("fail");
    assert_script_error(err, RgbppError::OutputCellMismatch);
}

#[test]
fn test_btc_time_lock_with_incorrect_output() {
    let loader = Loader::default();
    let mut context = Context::default();
    let scripts = TestScripts::setup(&loader, &mut context);
    let tx = build_btc_time_lock_tx(
        &mut context,
        &scripts,
        1000,
        vec![
            OutputDesc {
                lock: LockDesc::UserLock(Default::default()),
                amount: 900,
            },
            OutputDesc {
                lock: LockDesc::UserLock(Default::default()),
                amount: 100,
            },
        ],
    );
    let tx = context.complete_tx(tx);
    let err = verify_and_dump_failed_tx(&context, &tx, MAX_CYCLES).expect_err("fail");
    assert_script_error(err, RgbppError::OutputCellMismatch);
}

#[test]
fn test_rgbpp_unlock_with_index_witness() {
    let loader = Loader::default();
    let mut context = Context::default();

    let scripts = TestScripts::setup(&loader, &mut context);

    let tx = build_rgbpp_tx(
        &mut context,
        &scripts,
        1000,
        vec![
            OutputDesc {
                lock: LockDesc::Rgbpp,
                amount: 300,
            },
            OutputDesc {
                lock: LockDesc::Rgbpp,
                amount: 700,
            },
        ],
    );
    let tx = context.complete_tx(tx);
    let mut witnesses: Vec<_> = tx.witnesses().unpack();
    assert!(!witnesses[0].is_empty(), "unlock witness must isn't empty");

    // append unlock witness to the last
    let index: u32 = witnesses.len() as u32;
    witnesses.push(witnesses[0].clone());

    witnesses[0] = {
        let index = Bytes::copy_from_slice(&index.to_le_bytes());
        WitnessArgs::new_builder()
            .lock(BytesOpt::new_builder().set(Some(index.pack())).build())
            .build()
            .as_bytes()
    };

    let tx = tx
        .as_advanced_builder()
        .set_witnesses(witnesses.into_iter().map(|b| b.pack()).collect())
        .build();

    verify_and_dump_failed_tx(&context, &tx, MAX_CYCLES).expect("pass");
}
