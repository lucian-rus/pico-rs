/* base address of CLOCKS register */
const BASE_ADDR: u32 = 0x40008000;

/* offsets to different registers */
const REF_CTRL_OFFSET: u32 = 0x30;
const REF_DIV_OFFSET: u32 = 0x34;

const SYS_CTRL_OFFSET: u32 = 0x3C;

const PERI_CTRL_OFFSET: u32 = 0x48;

/* registers to be exported as public */
pub const REF_CTRL: *mut u32 = (BASE_ADDR + REF_CTRL_OFFSET) as *mut u32;
pub const REF_DIV: *mut u32 = (BASE_ADDR + REF_DIV_OFFSET) as *mut u32;

pub const SYS_CTRL: *mut u32 = (BASE_ADDR + SYS_CTRL_OFFSET) as *mut u32;

pub const PERI_CTRL: *mut u32 = (BASE_ADDR + PERI_CTRL_OFFSET) as *mut u32;

/* will be updated to include more registers if used */