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
    let message = b"Hello, New Kernel!";

    for (i, &byte) in message.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte; // caractere
            *vga_buffer.offset(i as isize * 2 + 1) = 0x0f; // cor do caractere
        }
    }
}

const VGA_WIDTH: usize = 80;

fn ascii_image() {
    let image = r#"
        .--.
       |o_o |
       |:_/ |
      //   \ \
     (|     | )
    /'\_   _/`\
    \___)=(___/
    "#;

    let vga_buffer = 0xb8000 as *mut u8;
    let start_row = 5; // define posição inicial da linha
    let start_col = 10; // define posição inicial da coluna
    let mut current_row = start_row; // inicializa a linha atual
    let mut current_col = start_col; // inicializa a coluna atual

    for byte in image.bytes() {
        if byte == b'\n' {
            current_row += 1;
            current_col = start_col;
        } else {
            unsafe {
                *vga_buffer.offset((current_row * VGA_WIDTH + current_col) as isize * 2) = byte; // caractere
                *vga_buffer.offset((current_row * VGA_WIDTH + current_col) as isize * 2 + 1) = 0x0f; // cor do caractere
            }
            current_col += 1;
        }
    }
}
