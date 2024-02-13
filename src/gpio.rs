/* while i'd like to not use this, i have to as it provides me a way to declare c-like structs */
use volatile_register::{RO, RW};

/* base address of the entire SIO module */
const BASE_ADDR: u32 = 0x40014000;

pub struct REG {
    p: &'static mut RegisterBlock,
}

/* this is based off of https://docs.rust-embedded.org/book/peripherals/a-first-attempt.html */
#[repr(C)]
struct RegisterBlock {
    /* there must be a better way to do this, check if possible with array of struct types */
    gpio0_status: RO<u32>,
    gpio0_ctrl: RW<u32>,
    gpio1_status: RO<u32>,
    gpio1_ctrl: RW<u32>,
    gpio2_status: RO<u32>,
    gpio2_ctrl: RW<u32>,
    gpio3_status: RO<u32>,
    gpio3_ctrl: RW<u32>,
    gpio4_status: RO<u32>,
    gpio4_ctrl: RW<u32>,
    gpio5_status: RO<u32>,
    gpio5_ctrl: RW<u32>,
    gpio6_status: RO<u32>,
    gpio6_ctrl: RW<u32>,
    gpio7_status: RO<u32>,
    gpio7_ctrl: RW<u32>,
    gpio8_status: RO<u32>,
    gpio8_ctrl: RW<u32>,
    gpio9_status: RO<u32>,
    gpio9_ctrl: RW<u32>,
    gpio10_status: RO<u32>,
    gpio10_ctrl: RW<u32>,
    gpio11_status: RO<u32>,
    gpio11_ctrl: RW<u32>,
    gpio12_status: RO<u32>,
    gpio12_ctrl: RW<u32>,
    gpio13_status: RO<u32>,
    gpio13_ctrl: RW<u32>,
    gpio14_status: RO<u32>,
    gpio14_ctrl: RW<u32>,
    gpio15_status: RO<u32>,
    gpio15_ctrl: RW<u32>,
    gpio16_status: RO<u32>,
    gpio16_ctrl: RW<u32>,
    gpio17_status: RO<u32>,
    gpio17_ctrl: RW<u32>,
    gpio18_status: RO<u32>,
    gpio18_ctrl: RW<u32>,
    gpio19_status: RO<u32>,
    gpio19_ctrl: RW<u32>,
    gpio20_status: RO<u32>,
    gpio20_ctrl: RW<u32>,
    gpio21_status: RO<u32>,
    gpio21_ctrl: RW<u32>,
    gpio22_status: RO<u32>,
    gpio22_ctrl: RW<u32>,
    gpio23_status: RO<u32>,
    gpio23_ctrl: RW<u32>,
    gpio24_status: RO<u32>,
    gpio24_ctrl: RW<u32>,
    gpio25_status: RO<u32>,
    gpio25_ctrl: RW<u32>,
    /* stopping here for now, will extend in the future */
}

impl REG {
    pub fn init() -> REG {
        REG {
            p: unsafe { &mut *(BASE_ADDR as *mut RegisterBlock) },
        }
    }

    /* can config multiple output pins, must OR them */
    pub fn config_gpio25_ctrl(&mut self, reg_val: u32) {
        unsafe { self.p.gpio25_ctrl.write(reg_val) }
    }

    /* can set multiple pins, must OR them */
    pub fn read_gpio25_status(&self) -> u32 {
        self.p.gpio25_status.read()
    }
}

pub const GPIO_BANK_RESET_ADDR: u32 = 0x4000f000;
