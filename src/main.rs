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
    print_message();
    ascii_image();
    loop {}
}

fn print_message() {
    let vga_buffer = 0xb8000 as *mut u8;
    let message = b"Hello, New kernel!";

    for (i, &byte) in message.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte; // caractere
            *vga_buffer.offset(i as isize * 2 + 1) = 0x0f; // cor do caractere
        }
    }
}
