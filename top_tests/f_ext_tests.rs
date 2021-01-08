mod common;
use common::*;
fn main() {
    let mut runner = RsicvTestRunner::new();
    //uf-p-*
    runner.test(XLen::X64, "rv64uf-p-fadd");
    runner.test(XLen::X64, "rv64uf-p-fclass");
    runner.test(XLen::X64, "rv64uf-p-fcmp");
    runner.test(XLen::X64, "rv64uf-p-fcvt");
    runner.test(XLen::X64, "rv64uf-p-fcvt_w");
    runner.test(XLen::X64, "rv64uf-p-fdiv");
    runner.test(XLen::X64, "rv64uf-p-fmadd");
    runner.test(XLen::X64, "rv64uf-p-fmin");
    runner.test(XLen::X64, "rv64uf-p-ldst");
    runner.test(XLen::X64, "rv64uf-p-move");
    runner.test(XLen::X64, "rv64uf-p-recoding");

    runner.test(XLen::X32, "rv32uf-p-fadd");
    runner.test(XLen::X32, "rv32uf-p-fclass");
    runner.test(XLen::X32, "rv32uf-p-fcmp");
    runner.test(XLen::X32, "rv32uf-p-fcvt");
    runner.test(XLen::X32, "rv32uf-p-fcvt_w");
    runner.test(XLen::X32, "rv32uf-p-fdiv");
    runner.test(XLen::X32, "rv32uf-p-fmadd");
    runner.test(XLen::X32, "rv32uf-p-fmin");
    runner.test(XLen::X32, "rv32uf-p-ldst");
    runner.test(XLen::X32, "rv32uf-p-move");
    runner.test(XLen::X32, "rv32uf-p-recoding");
}
