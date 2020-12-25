# trantor
帮你一步一步实现RISCV指令集模拟器的框架。这个框架是[terminus](https://github.com/shady831213/terminus)裁剪出来的。

## 第一步
- 实现RV32/64I
- 你需要定义RV32/64I 指令集， 指令的实现基于[terminus_vault](https://github.com/shady831213/terminus_vault), 如何定义指令可以参考[这里](https://github.com/shady831213/terminus_vault/blob/master/proc_macros/src/lib.rs#L28)
- 你需要定义RV32/64I 必要的csr, csr的实现基于[terminus_vault](https://github.com/shady831213/terminus_vault), 如何定义csr可以参考[这里](https://github.com/shady831213/terminus_vault/blob/master/proc_macros/src/lib.rs#L75)
- 你需要实现在[这里](https://github.com/shady831213/trantor/blob/master/src/processor/mod.rs)实现Processor的基本功能，如取指令，译码(可参考[这里](https://github.com/shady831213/terminus_vault/blob/master/proc_macros/src/lib.rs#L51)), 执行等
- 你需要通过测试[basic_tests](https://github.com/shady831213/trantor/blob/master/top_tests/basic_tests.rs), 你可能要修改[riscv_test](https://github.com/shady831213/trantor/blob/master/top_tests/common.rs#L76)的内容

```
  //run all test
  cargo run --example basic_tests
  //or run single test
  cargo run --example basic_tests -- -r rv64ui-p-add
```
- 如果需要debug testcase, 请参考[riscv-tests](https://github.com/riscv/riscv-tests), 或自行objdump elf, 所有的elf在[这里](https://github.com/shady831213/trantor/tree/master/top_tests/elf)

## 第二步
- TBD

## 第三步
- TBD

...
