#![no_std]
#![no_main]

use core::{arch::asm, panic::PanicInfo};

#[no_mangle]
#[link_section = ".text.start"]
pub extern "C" fn _start() -> ! {
    unsafe {
        // Turn pin21 to output
        core::ptr::write_volatile(0x3F200008 as *mut u32, 1 << 3);

        loop {
            // Enable pin 21
            core::ptr::write_volatile(0x3F20001C as *mut u32, 1 << 21);

            for _ in 1..500000 {
                asm!("nop");
            }

            // Disable pin 21
            core::ptr::write_volatile(0x3F200028 as *mut u32, 1 << 21);

            for _ in 1..500000 {
                asm!("nop");
            }
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
