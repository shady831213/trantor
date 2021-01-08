use crate::devices::bus::Bus;
use crate::prelude::*;
use std::rc::Rc;
use terminus_spaceport::irq::IrqVec;
pub mod trap;

//implement your ProcessorCfg here
#[derive(Debug, Clone)]
pub struct ProcessorCfg {}
//implement your Processor here
pub struct Processor {}

impl Processor {
    pub fn new(
        hartid: usize,
        config: ProcessorCfg,
        bus: &Rc<Bus>,
        clint: Option<IrqVec>,
        plic: Option<IrqVec>,
    ) -> Processor {
        //Add your Code
        Processor {}
    }

    pub fn reset(&mut self, start_address: u64) -> Result<(), String> {
        //Add your Code
        Ok(())
    }

    pub fn step(&mut self, n: usize) {
        assert!(n > 0);
        //Add your Code
    }

    pub fn trace(&self) -> String {
        //Add your Code
        "".to_string()
    }

    pub fn dump_state(&self) -> String {
        //Add your Code
        "".to_string()
    }
}
