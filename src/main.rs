#![no_std]
#![no_main]

use cortex_m::asm;
use cortex_m_rt::entry;
use panic_halt as _;

// needed to init the boot section. dunno why yet
#[link_section = ".boot2"]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_GENERIC_03H;

// GPIO used for LED is 25 on Pico board
const SIO_BLOCK_BASE_ADDR: u32 = 0xD0000000;
const SIO_BLOCK_GPIO_OE_OFFSET: u32 = 0x020;
const SIO_BLOCK_GPIO_OUT_OFFSET: u32 = 0x010;

const GPIO_BANK_BASE_ADDR: u32 = 0x40014000;
const GPIO_BANK_GPIO25_CTRL_OFFSET: u32 = 0x0CC;

const GPIO_BANK_RESET_ADDR: u32 = 0x4000f000;

#[entry]
fn main() -> ! {
    let gpio_bank_gpio25_ctrl = (GPIO_BANK_BASE_ADDR + GPIO_BANK_GPIO25_CTRL_OFFSET) as *mut u32;
    let sio_block_gpio_oe = (SIO_BLOCK_BASE_ADDR + SIO_BLOCK_GPIO_OE_OFFSET) as *mut u32;
    let sio_block_gpio_out = (SIO_BLOCK_BASE_ADDR + SIO_BLOCK_GPIO_OUT_OFFSET) as *mut u32;

    unsafe {
        (GPIO_BANK_RESET_ADDR as *mut u32).write_volatile(1 << 5);
        gpio_bank_gpio25_ctrl.write_volatile(0x05); // write function 5 (SIO) to CTRL register
        sio_block_gpio_oe.write_volatile(1 << 25); // set as output
        sio_block_gpio_out.write_volatile(1 << 25); // set output high
    }

    loop {
        unsafe {
            // mark them this way for better clarity
            for _i in 0..100_000 {
                asm::nop();
            }
            sio_block_gpio_out.write_volatile(0 << 25); // set output low

            for _i in 0..100_000 {
                asm::nop();
            }
            sio_block_gpio_out.write_volatile(1 << 25); // set output high
        }
    }
}
