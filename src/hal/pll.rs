/* while i'd like to not use this, i have to as it provides me a way to declare c-like structs */
use volatile_register::RW;

/* base address of the entire PLL module */
const BASE_ADDR: u32 = 0x40028000;

pub struct REG {
    p: &'static mut RegisterBlock,
}

/* this is based off of https://docs.rust-embedded.org/book/peripherals/a-first-attempt.html */
#[repr(C)]
struct RegisterBlock {
    cs: RW<u32>,
    pwr: RW<u32>,
    fbdiv_int: RW<u32>,
    prim: RW<u32>,
}

impl REG {
    pub fn init() -> REG {
        REG {
            p: unsafe { &mut *(BASE_ADDR as *mut RegisterBlock) },
        }
    }

    pub fn cs(&mut self, reg_val: u32) {
        unsafe { self.p.cs.write(reg_val) }
    }

    pub fn pwr(&mut self, reg_val: u32) {
        unsafe { self.p.pwr.write(reg_val) }
    }

    pub fn fbdiv_int(&mut self, reg_val: u32) {
        unsafe { self.p.fbdiv_int.write(reg_val) }
    }

    pub fn prim(&mut self, reg_val: u32) {
        unsafe { self.p.prim.write(reg_val) }
    }

    pub fn get_cs(&self) -> u32 {
        self.p.cs.read()
    }

}
