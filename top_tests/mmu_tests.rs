mod common;
use common::*;
fn main() {
    let mut runner = RsicvTestRunner::new();
    //*-v-*
    runner.test(XLen::X64, "rv64ui-v-add");
    runner.test(XLen::X32, "rv32ui-v-jalr");
    runner.test(XLen::X64, "rv64um-v-mul");
    runner.test(XLen::X32, "rv32um-v-div");
    runner.test(XLen::X64, "rv64uf-v-fmadd");
    runner.test(XLen::X32, "rv32uf-v-fdiv");
    runner.test(XLen::X64, "rv64ud-v-move");
    runner.test(XLen::X32, "rv32ud-v-fcvt_w");
    runner.test(XLen::X64, "rv64uc-v-rvc");
    runner.test(XLen::X32, "rv32uc-v-rvc");
    runner.test(XLen::X64, "rv64ua-v-amoadd_d");
    runner.test(XLen::X32, "rv32ua-v-amoadd_w");
    runner.test(XLen::X64, "rv64ua-v-lrsc");
    runner.test(XLen::X32, "rv32ua-v-lrsc");
}
