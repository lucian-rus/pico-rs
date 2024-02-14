
/* while i'd like to not use this, i have to as it provides me a way to declare c-like structs */
use volatile_register::RW;

/* base address of the entire CLOCK module */
const BASE_ADDR: u32 = 0x40008000;

pub struct REG {
    p: &'static mut RegisterBlock,
}

/* this is based off of https://docs.rust-embedded.org/book/peripherals/a-first-attempt.html */
#[repr(C)]
struct RegisterBlock {
    clk_gpout0_ctrl : RW<u32>,
    clk_gpout0_div : RW<u32>,
    clk_gpout0_selected : RW<u32>,
    clk_gpout1_ctrl : RW<u32>,
    clk_gpout1_div: RW<u32>,
    clk_gpout1_selected : RW<u32>,
    clk_gpout2_ctrl : RW<u32>,
    clk_gpout2_div : RW<u32>,
    clk_gpout2_selected: RW<u32>,
    clk_gpout3_ctrl : RW<u32>,
    clk_gpout3_div : RW<u32>,
    clk_gpout3_selected : RW<u32>,
    clk_ref_ctrl: RW<u32>,
    clk_ref_div : RW<u32>,
    clk_ref_selected: RW<u32>,
    clk_sys_ctrl: RW<u32>,
    clk_sys_div : RW<u32>,
    clk_sys_selected: RW<u32>,
    clk_peri_ctrl: RW<u32>,
    clk_peri_selected : RW<u32>,
    /* stopping here for now, will extend in the future */
}

impl REG {
    pub fn init() -> REG {
        REG {
            p: unsafe { &mut *(BASE_ADDR as *mut RegisterBlock) },
        }
    }

    pub fn clk_ref_ctrl(&mut self, reg_val: u32) {
        unsafe { self.p.clk_ref_ctrl.write(reg_val) }
    }
    
    pub fn clk_ref_div(&mut self, reg_val: u32) {
        unsafe { self.p.clk_ref_div.write(reg_val) }
    }

    pub fn clk_sys_ctrl(&mut self, reg_val: u32) {
        unsafe { self.p.clk_sys_ctrl.write(reg_val) }
    }

    pub fn clk_peri_ctrl(&mut self, reg_val: u32) {
        unsafe { self.p.clk_peri_ctrl.write(reg_val) }
    }

}
