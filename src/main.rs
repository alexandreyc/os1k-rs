#![no_std]
#![no_main]

use core::arch::asm;
use core::panic::PanicInfo;

extern "C" {
    static __stack_top: u8;
    static mut __bss: u8;
    static __bss_end: u8;
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

unsafe fn memset(buf: *mut u8, value: u8, len: usize) {
    (0..len).for_each(|i| *buf.add(i) = value);
}

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    // Zero-clear the .bss section.
    unsafe {
        let bss = &raw mut __bss;
        let bss_end = &raw const __bss_end;
        let len = bss_end as usize - bss as usize;
        memset(bss, 0, len);
    }

    loop {}
}

#[no_mangle]
pub extern "C" fn boot() {
    unsafe {
        asm!("mv sp, {}", "j kernel_main", in(reg) &__stack_top);
    }
}
