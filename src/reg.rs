/* while i'd like to not use this, i have to as it provides me a way to declare c-like structs */
use volatile_register::RW;

/* base address of the entire SIO module */
const BASE_ADDR: u32 = 0xD0000000;

pub struct SIO {
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

impl SIO {
    pub fn init() -> SIO {
        SIO {
            p: unsafe { &mut *(BASE_ADDR as *mut RegisterBlock) },
        }
    }

    /* can config multiple output pins, must OR them */
    pub fn config_output(&mut self, reg_val: u32) {
        unsafe { self.p.gpio_oe.write(reg_val) }
    }

    /* can set multiple pins, must OR them */
    pub fn set_output(&mut self, reg_val: u32) {
        unsafe { self.p.gpio_out.write(reg_val) }
    }
}