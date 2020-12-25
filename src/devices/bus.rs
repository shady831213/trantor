use terminus_spaceport::space::Space;
use std::cell::{RefCell, Ref, RefMut};

//You may modify Bus
pub struct Bus {
    space: RefCell<Space>,
}

impl Bus {
    pub fn new() -> Bus {
        Bus {
            space: RefCell::new(Space::new()),
        }
    }
    pub fn write_u8(&self, addr: &u64, data: &u8) -> Result<(), u64> {
        self.space.borrow().write_bytes(addr, unsafe { std::slice::from_raw_parts(data as *const u8, 1) })?;
        Ok(())
    }
    pub fn read_u8(&self, addr: &u64, data: &mut u8) -> Result<(), u64> {
        self.space.borrow().read_bytes(addr, unsafe { std::slice::from_raw_parts_mut(data as *mut u8, 1) })?;
        Ok(())
    }

    pub fn write_u16(&self, addr: &u64, data: &u16) -> Result<(), u64> {
        self.space.borrow().write_bytes(addr, unsafe { std::slice::from_raw_parts((data as *const u16) as *const u8, 2) })?;
        Ok(())
    }

    pub fn read_u16(&self, addr: &u64, data: &mut u16) -> Result<(), u64> {
        self.space.borrow().read_bytes(addr, unsafe { std::slice::from_raw_parts_mut((data as *mut u16) as *mut u8, 2) })?;
        Ok(())
    }

    pub fn write_u32(&self, addr: &u64, data: &u32) -> Result<(), u64> {
        self.space.borrow().write_bytes(addr, unsafe { std::slice::from_raw_parts((data as *const u32) as *const u8, 4) })?;
        Ok(())
    }

    pub fn read_u32(&self, addr: &u64, data: &mut u32) -> Result<(), u64> {
        self.space.borrow().read_bytes(addr, unsafe { std::slice::from_raw_parts_mut((data as *mut u32) as *mut u8, 4) })?;
        Ok(())
    }

    pub fn write_u64(&self, addr: &u64, data: &u64) -> Result<(), u64> {
        self.space.borrow().write_bytes(addr, unsafe { std::slice::from_raw_parts((data as *const u64) as *const u8, 8) })?;
        Ok(())
    }

    pub fn read_u64(&self, addr: &u64, data: &mut u64) -> Result<(), u64> {
        self.space.borrow().read_bytes(addr, unsafe { std::slice::from_raw_parts_mut((data as *mut u64) as *mut u8, 8) })?;
        Ok(())
    }

    pub fn space(&self) -> Ref<'_, Space> {
        self.space.borrow()
    }

    pub fn space_mut(&self) -> RefMut<'_, Space> {
        self.space.borrow_mut()
    }
}