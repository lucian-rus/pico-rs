#![no_std]
#![no_main]

/* import modules from project */
mod clock;
mod gpio;
mod reset;
mod sio;
mod syst;
mod xosc;

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

    let mut syst_reg = syst::REG::init();
    syst_reg.syst_rvr(3_000_000);
    syst_reg.syst_csr(1 << 2 | 1);

    let mut sio_reg = sio::REG::init();
    sio_reg.gpio_oe(1 << 25);
    let mut boolean = true;

    loop {
        if (syst_reg.get_syst_csr() & (1 << 16)) == (1 << 16) {
            if boolean == true {
                sio_reg.gpio_out(1 << 25);
                boolean = false;
            } else {
                sio_reg.gpio_out(0 << 25);
                boolean = true;
            }
        }
    }
}

fn config_xosc() {
    /* get XOSC instance */
    let mut xosc_reg = xosc::REG::init();

    /* set XOSC frequency range */
    xosc_reg.ctrl(0xAA0);

    /* value is default 0xc4 */
    xosc_reg.startup(0xc4);
    /* enable XOSC */
    xosc_reg.ctrl(0xFAB << 12);

    /* wait for XOSC to stabilize */
    while (xosc_reg.status() & 0x80000000) != 0x80000000 {}
}

fn config_clock() {
    /* get CLOCK instance */
    let mut clock_reg = clock::REG::init();

    /* select clock source. set REF to XOSC, SYS to REF */
    clock_reg.clk_ref_ctrl(0x02);
    clock_reg.clk_sys_ctrl(0x00);

    /* do NOT divide. inverted logic */
    clock_reg.clk_ref_div(1 << 8);

    clock_reg.clk_peri_ctrl(1 << 11 | 0x04 << 5);
}

/* avoids warnings until implemented */
#[warn(dead_code)]
fn config_pll() {}

/* not the best, as this does not return the instances. will re-write */
fn config_gpio() {
    let mut reset_reg = reset::REG::init();
    reset_reg.reset(1 << 5);

    /* wait for reset to be done */
    while(reset_reg.reset_done() & (1 << 5)) == (1 << 5) {}

    let mut io_reg = gpio::REG::init();
    io_reg.gpio25_ctrl(0x05);
}
