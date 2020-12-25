mod common;
use common::*;
fn main() {
    let mut runner = RsicvTestRunner::new();
    //uc-p-*
    runner.test(XLen::X64, "rv64uc-p-rvc");

    runner.test(XLen::X32, "rv32uc-p-rvc");
}


