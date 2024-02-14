/* while i'd like to not use this, i have to as it provides me a way to declare c-like structs */
use volatile_register::{RW, RO};

/* base address of the entire RESETS module */
// check why this only works as base <0x4000c000> + set <0x3000>
const BASE_ADDR: u32 = 0x4000f000;

pub struct REG {
    p: &'static mut RegisterBlock,
}

/* this is based off of https://docs.rust-embedded.org/book/peripherals/a-first-attempt.html */
#[repr(C)]
struct RegisterBlock {
    reset: RW<u32>,
    wdsel: RW<u32>,
    reset_done: RO<u32>,
    /* stopping here for now, will extend in the future */
}

impl REG {
    pub fn init() -> REG {
        REG {
            p: unsafe { &mut *(BASE_ADDR as *mut RegisterBlock) },
        }
    }

    pub fn reset(&mut self, reg_val: u32) {
        unsafe { self.p.reset.write(reg_val) }
    }

    pub fn reset_done(&self) -> u32 {
        self.p.reset_done.read()
    }
}
