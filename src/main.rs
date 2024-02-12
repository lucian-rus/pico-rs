#![no_std]
#![no_main]

/* import modules from project */
mod clock;
mod reg;
mod xosc;

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

    unsafe {
        let reset_reg: *mut u32 = (0x4000C000 | 0x3000) as *mut u32;
        let reset_status_reg: *mut u32 = 0x4000C008 as *mut u32;

        reset_reg.write_volatile(0x01 << 5);
        while (reset_status_reg.read_volatile() & (1 << 5)) == 0 {}
    }

    config_gpio();

    let sio_block_gpio_out =
        (reg::SIO_BLOCK_BASE_ADDR + reg::SIO_BLOCK_GPIO_OUT_OFFSET) as *mut u32;

    loop {
        unsafe {
            /* mark them this way for better clarity */
            for _i in 0..12_000_000 {
                asm::nop();
            }
            sio_block_gpio_out.write_volatile(1 << 25); // set output high

            for _i in 0..12_000_000 {
                asm::nop();
            }
            sio_block_gpio_out.write_volatile(0 << 25); // set output low
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
    let gpio_bank_gpio25_ctrl =
        (reg::GPIO_BANK_BASE_ADDR + reg::GPIO_BANK_GPIO25_CTRL_OFFSET) as *mut u32;
    let sio_block_gpio_oe = (reg::SIO_BLOCK_BASE_ADDR + reg::SIO_BLOCK_GPIO_OE_OFFSET) as *mut u32;

    unsafe {
        (reg::GPIO_BANK_RESET_ADDR as *mut u32).write_volatile(1 << 5);
        gpio_bank_gpio25_ctrl.write_volatile(0x05); // write function 5 (SIO) to CTRL register
        sio_block_gpio_oe.write_volatile(1 << 25); // set as output
    }
}
