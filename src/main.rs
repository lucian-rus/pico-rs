#![no_std]
#![no_main]

/* import modules from project */
mod clock;
mod reg;
mod xosc;
mod gpio;

#[warn(unused_imports)]
use cortex_m::asm;

use cortex_m_rt::entry;
use panic_halt as _;

/* needed to init the boot section. dunno why yet */
#[link_section = ".boot2"]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_GENERIC_03H;

#[entry]
fn main() -> ! {
    config_xosc();
    config_clock();

    config_gpio();
    
    /* used for counting using XOSC */
    let cortex_syst_rvr: *mut u32 = ((0xe0000000 as u32) + 0xe014) as *mut u32;
    let cortex_syst_csr: *mut u32 = ((0xe0000000 as u32) + 0xe010) as *mut u32;

    unsafe {
        /* 1 second */
        cortex_syst_rvr.write_volatile(3_000_000);
        cortex_syst_csr.write_volatile(1 << 2 | 1);
    }

    let mut io_reg = gpio::REG::init();
    io_reg.config_gpio25_ctrl(0x05);

    let mut sio_reg = reg::SIO::init();
    sio_reg.config_output(1 << 25);
    let mut boolean = true;

    loop {
        unsafe {
            if (cortex_syst_csr.read_volatile() & (1 << 16)) == (1 << 16) {
                if boolean == true {
                    sio_reg.set_output(1 << 25);
                    boolean = false;
                } else {
                    sio_reg.set_output(0 << 25);
                    boolean = true;
                }
            }
        }
    }
}

fn config_xosc() {
    unsafe {
        /* set XOSC frequency range */
        xosc::CTRL.write_volatile(0xAA0);

        /* value is default 0xc4 */
        xosc::STARTUP.write_volatile(0xc4);
        /* enable XOSC */
        xosc::CTRL.write_volatile(0xFAB << 12);

        /* wait for XOSC to stabilize */
        while (xosc::STATUS.read_volatile() & 0x80000000) != 0x80000000 {}
    }
}

fn config_clock() {
    unsafe {
        /* select clock source. set REF to XOSC, SYS to REF */
        clock::REF_CTRL.write_volatile(0x02);
        clock::SYS_CTRL.write_volatile(0x00);

        /* do NOT divide. inverted logic */
        clock::REF_DIV.write_volatile(1 << 8);

        clock::PERI_CTRL.write_volatile(1 << 11 | 0x04 << 5);
    }
}

/* avoids warnings until implemented */
#[warn(dead_code)]
fn config_pll() {}

fn config_gpio() {
    // let gpio_bank_gpio25_ctrl =
    //     (reg::GPIO_BANK_BASE_ADDR + reg::GPIO_BANK_GPIO25_CTRL_OFFSET) as *mut u32;


    unsafe {
        (gpio::GPIO_BANK_RESET_ADDR as *mut u32).write_volatile(1 << 5);
        // gpio_bank_gpio25_ctrl.write_volatile(0x05); // write function 5 (SIO) to CTRL register
    }
}
