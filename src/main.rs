#![no_std]
#![no_main]

// use core::ptr;
use core::arch::asm;
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Black = 0x0,
    Blue = 0x1,
    Green = 0x2,
    Cyan = 0x3,
    Red = 0x4,
    Magenta = 0x5,
    Brown = 0x6,
    LightGray = 0x7,
    DarkGray = 0x8,
    LightBlue = 0x9,
    LightGreen = 0xA,
    LightCyan = 0xB,
    LightRed = 0xC,
    LightMagenta = 0xD,
    LightBrown = 0xE,
    White = 0xF,
}

impl Color {
    pub fn to_u8(self) -> u8 {
        self as u8
    }
}

const VGA_WIDTH: usize = 80;
const VGA_HEIGHT: usize = 25;
const VGA_BUFFER: *mut u16 = 0xB8000 as *mut u16;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ColorChar {
    pub character: u8,
    pub color: u8,
}

pub struct VGA {
    pub buffer: *mut ColorChar,
}

impl VGA {
    pub fn new() -> VGA {
        VGA {
            buffer: VGA_BUFFER as *mut ColorChar,
        }
    }

    pub fn move_cursor(&self, row: usize, col: usize) {
        let index = (row * VGA_WIDTH + col) as u16;
        unsafe {
            // VGA port 0x3D4 is the index register, 0x3D5 is the data register
            // Move cursor to the specified position (row, col)
            outb(0x3D4, 0x0F); // Cursor position high byte
            outb(0x3D5, (index & 0xFF) as u8);
            outb(0x3D4, 0x0E); // Cursor position low byte
            outb(0x3D5, ((index >> 8) & 0xFF) as u8);
        }
    }

    // Clear the screen
    pub fn clear_screen(&self) {
        for row in 0..VGA_HEIGHT {
            for col in 0..VGA_WIDTH {
                let index = row * VGA_WIDTH + col;
                unsafe {
                    *self.buffer.offset(index as isize) = ColorChar {
                        character: b' ' as u8,
                        color: (Color::Blue.to_u8() << 4) | Color::White.to_u8(),
                    };
                }
            }
        }
    }

    // Print a string at a specific location
    pub fn print_at(&self, row: usize, col: usize, message: &str, color: Color) {
        let color_byte = (Color::Black.to_u8() << 4) | color.to_u8();
        for (i, byte) in message.bytes().enumerate() {
            let index = (row * VGA_WIDTH) + col + i;
            unsafe {
                *self.buffer.offset(index as isize) = ColorChar {
                    character: byte,
                    color: color_byte,
                };
            }
        }
    }
}

unsafe fn outb(port: u16, val: u8) {
    asm!("out dx, al", in("dx") port, in("al") val);
}


#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga = VGA::new();
    
    // Clear the screen with a bluish background
    vga.clear_screen();

    // Retro title at the top center
    let title = "isaka_james OS";
    let title_col = (VGA_WIDTH - title.len()) / 2;
    vga.print_at(0, title_col, title, Color::LightBlue);

    // Display username and password section
    vga.print_at(5, 5, "Username:", Color::LightCyan);
    vga.print_at(6, 5, "Password:", Color::LightCyan);

    // Simulate placeholder text for username and password
    vga.print_at(5, 15, "__________", Color::LightGreen);
    vga.print_at(6, 15, "__________", Color::LightGreen);

    // Move the cursor to the username input area (after the placeholder)
    vga.move_cursor(5, 15);

    loop {}
}
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {} // Infinite loop on panic, as there's no way to recover
}

