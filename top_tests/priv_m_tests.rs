mod common;
use common::*;
fn main() {
    let mut runner = RsicvTestRunner::new();
    //mi-p-*
    runner.test(XLen::X64, "rv64mi-p-access");
    runner.test(XLen::X64, "rv64mi-p-breakpoint");
    runner.test(XLen::X64, "rv64mi-p-sbreak");
    runner.test(XLen::X64, "rv64mi-p-csr");
    runner.test(XLen::X64, "rv64mi-p-illegal");
    runner.test(XLen::X64, "rv64mi-p-ma_addr");
    runner.test(XLen::X64, "rv64mi-p-ma_fetch");
    runner.test(XLen::X64, "rv64mi-p-mcsr");
    runner.test(XLen::X64, "rv64mi-p-scall");

    runner.test(XLen::X32, "rv32mi-p-breakpoint");
    runner.test(XLen::X32, "rv32mi-p-sbreak");
    runner.test(XLen::X32, "rv32mi-p-csr");
    runner.test(XLen::X32, "rv32mi-p-illegal");
    runner.test(XLen::X32, "rv32mi-p-ma_addr");
    runner.test(XLen::X32, "rv32mi-p-ma_fetch");
    runner.test(XLen::X32, "rv32mi-p-mcsr");
    runner.test(XLen::X32, "rv32mi-p-scall");
    runner.test(XLen::X32, "rv32mi-p-shamt");
}


