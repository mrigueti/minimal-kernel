#![no_std]
#![no_main]

use core::panic::PanicInfo;

/// Esta é a função chamada quando ocorre um panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/// Esta é a função de entrada do kernel.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}

