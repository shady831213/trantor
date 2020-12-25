mod common;
use common::*;
fn main() {
    let mut runner = RsicvTestRunner::new();
    //um-p-*
    runner.test(XLen::X64, "rv64um-p-mul");
    runner.test(XLen::X64, "rv64um-p-mulh");
    runner.test(XLen::X64, "rv64um-p-mulhsu");
    runner.test(XLen::X64, "rv64um-p-mulhu");
    runner.test(XLen::X64, "rv64um-p-mulw");
    runner.test(XLen::X64, "rv64um-p-div");
    runner.test(XLen::X64, "rv64um-p-divu");
    runner.test(XLen::X64, "rv64um-p-divw");
    runner.test(XLen::X64, "rv64um-p-divuw");
    runner.test(XLen::X64, "rv64um-p-rem");
    runner.test(XLen::X64, "rv64um-p-remu");
    runner.test(XLen::X64, "rv64um-p-remw");
    runner.test(XLen::X64, "rv64um-p-remuw");

    runner.test(XLen::X32, "rv32um-p-mul");
    runner.test(XLen::X32, "rv32um-p-mulh");
    runner.test(XLen::X32, "rv32um-p-mulhsu");
    runner.test(XLen::X32, "rv32um-p-mulhu");
    runner.test(XLen::X32, "rv32um-p-div");
    runner.test(XLen::X32, "rv32um-p-divu");
    runner.test(XLen::X32, "rv32um-p-rem");
    runner.test(XLen::X32, "rv32um-p-remu");
}


