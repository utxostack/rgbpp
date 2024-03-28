use crate::rgbpp::{build_btc_time_lock_tx, build_rgbpp_tx};
use crate::utils::TestScripts;
use crate::{verify_and_dump_failed_tx, Loader};
use ckb_testtool::context::Context;

const MAX_CYCLES: u64 = 10_000_000;

#[test]
fn test_rgbpp_unlock() {
    let loader = Loader::default();
    let mut context = Context::default();

    let scripts = TestScripts::setup(&loader, &mut context);

    let tx = build_rgbpp_tx(&mut context, &scripts);
    let tx = context.complete_tx(tx);

    verify_and_dump_failed_tx(&context, &tx, MAX_CYCLES).expect("pass");
}

#[test]
fn test_btc_time_lock_unlock() {
    let loader = Loader::default();
    let mut context = Context::default();
    let scripts = TestScripts::setup(&loader, &mut context);
    let tx = build_btc_time_lock_tx(&mut context, &scripts);
    let tx = context.complete_tx(tx);
    verify_and_dump_failed_tx(&context, &tx, MAX_CYCLES).expect("pass");
}
