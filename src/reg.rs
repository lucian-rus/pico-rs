// GPIO used for LED is 25 on Pico board

// update this as its bs
pub const SIO_BLOCK_BASE_ADDR: u32 = 0xD0000000;
pub const SIO_BLOCK_GPIO_OE_OFFSET: u32 = 0x020;
pub const SIO_BLOCK_GPIO_OUT_OFFSET: u32 = 0x010;

pub const GPIO_BANK_BASE_ADDR: u32 = 0x40014000;
pub const GPIO_BANK_GPIO25_CTRL_OFFSET: u32 = 0x0CC;

pub const GPIO_BANK_RESET_ADDR: u32 = 0x4000f000;
