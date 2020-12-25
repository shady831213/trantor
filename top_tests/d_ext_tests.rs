mod common;
use common::*;
fn main() {
    let mut runner = RsicvTestRunner::new();
    //ud-p-*
    runner.test(XLen::X64, "rv64ud-p-fadd");
    runner.test(XLen::X64, "rv64ud-p-fclass");
    runner.test(XLen::X64, "rv64ud-p-fcmp");
    runner.test(XLen::X64, "rv64ud-p-fcvt");
    runner.test(XLen::X64, "rv64ud-p-fcvt_w");
    runner.test(XLen::X64, "rv64ud-p-fdiv");
    runner.test(XLen::X64, "rv64ud-p-fmadd");
    runner.test(XLen::X64, "rv64ud-p-fmin");
    runner.test(XLen::X64, "rv64ud-p-ldst");
    runner.test(XLen::X64, "rv64ud-p-move");
    runner.test(XLen::X64, "rv64ud-p-recoding");
    runner.test(XLen::X64, "rv64ud-p-structural");

    runner.test(XLen::X32, "rv32ud-p-fadd");
    runner.test(XLen::X32, "rv32ud-p-fclass");
    runner.test(XLen::X32, "rv32ud-p-fcmp");
    runner.test(XLen::X32, "rv32ud-p-fcvt");
    runner.test(XLen::X32, "rv32ud-p-fcvt_w");
    runner.test(XLen::X32, "rv32ud-p-fdiv");
    runner.test(XLen::X32, "rv32ud-p-fmadd");
    runner.test(XLen::X32, "rv32ud-p-fmin");
    runner.test(XLen::X32, "rv32ud-p-ldst");
    runner.test(XLen::X32, "rv32ud-p-recoding");
}


