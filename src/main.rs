#![no_std]
#![no_main]

use panic_halt as _;
use cortex_m_rt::entry;

// needed to init the boot section. dunno why yet
#[link_section = ".boot2"]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_GENERIC_03H;

// GPIO used for LED is 25 on Pico board
const SIO_BLOCK_BASE_ADDR: u32 = 0xD0000000;
const SIO_BLOCK_GPIO_OE_OFFSET: u32 = 0x020;
const SIO_BLOCK_GPIO_OUT_OFFSET: u32 = 0x010;

const GPIO_BANK_BASE_ADDR: u32 = 0x40014000;
const GPIO_BANK_GPIO25_CTRL_OFFSET:u32 = 0x0CC;

const GPIO_BANK_RESET_ADDR: u32 = 0x4000f000;

#[entry]
fn main() -> ! {

    let GPIO_BANK_GPIO25_CTRL = (GPIO_BANK_BASE_ADDR + GPIO_BANK_GPIO25_CTRL_OFFSET) as *mut u32;
    let SIO_BLOCK_GPIO_OE: *mut u32 = (SIO_BLOCK_BASE_ADDR + SIO_BLOCK_GPIO_OE_OFFSET) as *mut u32;
    let SIO_BLOCK_GPIO_OUT = (SIO_BLOCK_BASE_ADDR + SIO_BLOCK_GPIO_OUT_OFFSET) as *mut u32;    

    unsafe {
        (GPIO_BANK_RESET_ADDR as *mut u32).write_volatile(1 << 5);
        GPIO_BANK_GPIO25_CTRL.write_volatile(0x05); // write function 5 (SIO) to CTRL register
        SIO_BLOCK_GPIO_OE.write_volatile(1 << 25); // set as output
        SIO_BLOCK_GPIO_OUT.write_volatile(1 << 25); // set output high
        }

    loop {}
}
