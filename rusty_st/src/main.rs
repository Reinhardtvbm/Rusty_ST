#![no_std]
#![no_main]

use core::panic::PanicInfo;
use cortex_m_rt::entry;
use rusty_st::stm32g431_hal::HAL;

#[entry]
fn main() -> ! {
    let hal = HAL::init();

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
