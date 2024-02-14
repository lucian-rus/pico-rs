/* while i'd like to not use this, i have to as it provides me a way to declare c-like structs */
use volatile_register::RW;

/* base address of the entire SIO module */
const BASE_ADDR: u32 = 0xD0000000;

pub struct REG {
    p: &'static mut RegisterBlock,
}

/* this is based off of https://docs.rust-embedded.org/book/peripherals/a-first-attempt.html */
#[repr(C)]
struct RegisterBlock {
    cpuid: RW<u32>,
    gpio_in: RW<u32>,
    gpio_hi_in: RW<u32>,
    pad_01: RW<u32>, // padding so the register gets properly alligned
    gpio_out: RW<u32>,
    gpio_out_set: RW<u32>,
    gpio_out_clr: RW<u32>,
    gpio_out_xor: RW<u32>,
    gpio_oe: RW<u32>,
    gpio_oe_set: RW<u32>,
    gpio_oe_clr: RW<u32>,
    gpio_oe_xor: RW<u32>,
    /* stopping here for now, will extend in the future */
}

impl REG {
    pub fn init() -> REG {
        REG {
            p: unsafe { &mut *(BASE_ADDR as *mut RegisterBlock) },
        }
    }

    pub fn gpio_oe(&mut self, reg_val: u32) {
        unsafe { self.p.gpio_oe.write(reg_val) }
    }

    pub fn gpio_out(&mut self, reg_val: u32) {
        unsafe { self.p.gpio_out.write(reg_val) }
    }
}