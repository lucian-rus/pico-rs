/* while i'd like to not use this, i have to as it provides me a way to declare c-like structs */
use volatile_register::RW;

/* base address of the entire XOSC module */
const BASE_ADDR: u32 = 0x40024000;

pub struct REG {
    p: &'static mut RegisterBlock,
}

/* this is based off of https://docs.rust-embedded.org/book/peripherals/a-first-attempt.html */
#[repr(C)]
struct RegisterBlock {
    ctrl: RW<u32>,
    status: RW<u32>,
    dormant: RW<u32>,
    startup: RW<u32>,
    count: RW<u32>,
    /* stopping here for now, will extend in the future */
}

impl REG {
    pub fn init() -> REG {
        REG {
            p: unsafe { &mut *(BASE_ADDR as *mut RegisterBlock) },
        }
    }

    pub fn ctrl(&mut self, reg_val: u32) {
        unsafe { self.p.ctrl.write(reg_val) }
    }
    
    pub fn startup(&mut self, reg_val: u32) {
        unsafe { self.p.startup.write(reg_val) }
    }

    pub fn status(&self) -> u32 {
        self.p.status.read()
    }
}