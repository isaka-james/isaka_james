#![no_std]
#![no_main] // No crt0 (start)

use core::panic::PanicInfo;

const VGA_BUFFER: usize = 0xb8000;

/// Kernel entry point
#[no_mangle]
pub extern "C" fn _start() -> ! {
    let message = b"Welcome to Isaka OS!";
    let color_byte = 0x07; // Light grey on black

    let buffer = VGA_BUFFER as *mut u8;

    for (i, &byte) in message.iter().enumerate() {
        unsafe {
            *buffer.offset(i as isize * 2) = byte;
            *buffer.offset(i as isize * 2 + 1) = color_byte;
        }
    }

    loop {}
}

/// Panic handler for the kernel
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

