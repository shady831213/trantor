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
- 实现RV32/64M
- 你需要定义RV32/64M 指令集, 相信通过第一步，你已经发现通过[terminus_vault](https://github.com/shady831213/terminus_vault)是很容易的
- 你需要通过测试[m_ext_tests](https://github.com/shady831213/trantor/blob/master/top_tests/m_ext_tests.rs)

## 第三步
- 实现M/S/U privilege
- 你需要修改processor, 定义新的csr，并添加相应的privilege指令
- 你需要通过[priv_m_tests](https://github.com/shady831213/trantor/blob/master/top_tests/priv_m_tests.rs), [priv_m_tests](https://github.com/shady831213/trantor/blob/master/top_tests/priv_m_tests.rs)

## 第四步
- 实现atomic extension并支持多核
- 你需要添加atomic指令，修改processor和bus来支持atoimc操作
- 你需要通过[multi_core_tests](https://github.com/shady831213/trantor/blob/master/top_tests/multi_core_tests.rs)

## 第五步
- 实现compress extensionf
- 你需要添加compress指令集
- 你需要通过[c_ext_tests](https://github.com/shady831213/trantor/blob/master/top_tests/c_ext_tests.rs)

## 第六步
- 实现浮点指令集
- 你需要添加单精度和双精度浮点指令集
- 你需要通过[f_ext_tests](https://github.com/shady831213/trantor/blob/master/top_tests/f_ext_tests.rs), [d_ext_tests](https://github.com/shady831213/trantor/blob/master/top_tests/d_ext_tests.rs)

## 第七步
- 支持虚拟地址
- 你需要修改该processor,为processor添加mmu,以支持虚拟地址，pmp
- 你需要通过[mmu_tests](https://github.com/shady831213/trantor/blob/master/top_tests/mmu_tests.rs)
- 至此你应该能通过[riscv_tests](https://github.com/shady831213/trantor/blob/master/top_tests/riscv_tests.rs)

## 第八步
- boot linux
- 通过之前步骤，你现在可以尝试用模拟器来启动linux, [这里](https://github.com/shady831213/trantor/tree/master/examples/linux/image)提供了一个bbl+linux的image
- 你需要添加一些必要的外设，[terminus_transport](https://github.com/shady831213/terminus_spaceport)可以帮助到你，你也可以借助[terminus_transport](https://github.com/shady831213/terminus_spaceport)定义自己的外设
- 你可能需要利用到elf和dts的一些处理，[这里](https://github.com/shady831213/trantor/tree/master/src/system)可能可以帮助到你
- 在此步骤你可能会发现之前测试没有覆盖的bug，为processor提供一些简单的trace和debug功能会很很有帮助

## Enjoy ^_^
