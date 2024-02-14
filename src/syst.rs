/* while i'd like to not use this, i have to as it provides me a way to declare c-like structs */
use volatile_register::RW;

/* base address of the entire SYSTICK module */
const BASE_ADDR: u32 = 0xe0000000 + 0xe010;

pub struct REG {
    p: &'static mut RegisterBlock,
}

/* this is based off of https://docs.rust-embedded.org/book/peripherals/a-first-attempt.html */
#[repr(C)]
struct RegisterBlock {
    /* check how to introduce padding properly */
    syst_csr: RW<u32>,
    syst_rvr: RW<u32>,
    syst_cvr: RW<u32>,
    /* stopping here for now, will extend in the future */
}

impl REG {
    pub fn init() -> REG {
        REG {
            p: unsafe { &mut *(BASE_ADDR as *mut RegisterBlock) },
        }
    }

    pub fn syst_csr(&mut self, reg_val: u32) {
        unsafe { self.p.syst_csr.write(reg_val) }
    }

    pub fn syst_rvr(&mut self, reg_val: u32) {
        unsafe { self.p.syst_rvr.write(reg_val) }
    }

    /* check how to do this properly, for now it works */
    pub fn get_syst_csr(&self) -> u32 {
        self.p.syst_csr.read()
    }
}
