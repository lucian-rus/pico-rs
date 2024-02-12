/* base address of XOSC register */
const BASE_ADDR: u32 = 0x40024000;

/* offsets to different registers */
const CTRL_OFFSET: u32 = 0x00;
const STATUS_OFFSET: u32 = 0x04;
const STARTUP_OFFSET: u32 = 0x0c;

/* registers to be exported as public */
pub const CTRL: *mut u32 = (BASE_ADDR + CTRL_OFFSET) as *mut u32;
pub const STATUS: *mut u32 = (BASE_ADDR + STATUS_OFFSET) as *mut u32;
pub const STARTUP: *mut u32 = (BASE_ADDR + STARTUP_OFFSET) as *mut u32;

/* will be updated to include more registers if used */