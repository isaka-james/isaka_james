#![no_std]
#![no_main]

use core::panic::PanicInfo;


#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
struct MultibootHeader {
    magic: u32,
    flags: u32,
    checksum: u32,
}

#[no_mangle]
#[link_section = ".multiboot_header"]
static MULTIBOOT_HEADER: MultibootHeader = MultibootHeader {
    magic: 0x1BADB002,
    flags: 0,
    checksum: (0x1BADB002u32 + 0u32).wrapping_neg() as u32,
};

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

