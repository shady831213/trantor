mod common;
use common::*;
fn main() {
    let mut runner = RsicvTestRunner::new();
    //si-p-*
    runner.test(XLen::X64, "rv64si-p-csr");
    runner.test(XLen::X64, "rv64si-p-dirty");
    runner.test(XLen::X64, "rv64si-p-ma_fetch");
    runner.test(XLen::X64, "rv64si-p-sbreak");
    runner.test(XLen::X64, "rv64si-p-scall");
    runner.test(XLen::X64, "rv64si-p-wfi");

    runner.test(XLen::X32, "rv32si-p-csr");
    runner.test(XLen::X32, "rv32si-p-dirty");
    runner.test(XLen::X32, "rv32si-p-ma_fetch");
    runner.test(XLen::X32, "rv32si-p-sbreak");
    runner.test(XLen::X32, "rv32si-p-scall");
    runner.test(XLen::X32, "rv32si-p-wfi");
}


