#![no_std]
#![no_main]

use core::arch::asm;
use core::fmt;
use core::mem::transmute;
use core::panic::PanicInfo;

extern "C" {
    static __stack_top: u8;
    static mut __bss: u8;
    static __bss_end: u8;
}

unsafe fn memset(buf: *mut u8, value: u8, len: usize) {
    (0..len).for_each(|i| *buf.add(i) = value);
}

#[repr(i32)]
#[derive(Debug)]
enum SbiError {
    Failed = -1,
    NotSupported = -2,
    InvalidParam = -3,
    Denied = -4,
    InvalidAddress = -5,
    AlreadyAvailable = -6,
    AlreadyStarted = -7,
    AlreadyStopped = -8,
    NoShmem = -9,
    InvalidState = -10,
    BadRange = -11,
    Timeout = -12,
    Io = -13,
}

impl From<SbiError> for fmt::Error {
    fn from(_: SbiError) -> Self {
        Self
    }
}

struct SbiValue(i32);

type SbiResult = Result<SbiValue, SbiError>;

fn sbi_call(
    arg0: i32,
    arg1: i32,
    arg2: i32,
    arg3: i32,
    arg4: i32,
    arg5: i32,
    fid: i32,
    eid: i32,
) -> SbiResult {
    let (error, value): (i32, i32);
    unsafe {
        asm!(
            "ecall",
            in("a0") arg0,
            in("a1") arg1,
            in("a2") arg2,
            in("a3") arg3,
            in("a4") arg4,
            in("a5") arg5,
            in("a6") fid,
            in("a7") eid,
            lateout("a0") error,
            lateout("a1") value,
        );

        if error == 0 {
            Ok(SbiValue(value))
        } else {
            Err(transmute(error))
        }
    }
}

fn putchar(c: i32) -> SbiResult {
    sbi_call(c, 0, 0, 0, 0, 0, 0, 1)
}

struct ConsoleWriter;

impl fmt::Write for ConsoleWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            putchar(c as i32)?;
        }
        Ok(())
    }
}

macro_rules! print {
    ($($arg:tt)*) => {{
        fmt::write(&mut ConsoleWriter, format_args!($($arg)*)).unwrap();
    }};
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        print!(
            "PANIC: {}:{}: {}\n",
            location.file(),
            location.line(),
            info.message()
        );
    } else {
        print!("PANIC: {}\n", info.message());
    }
    loop {}
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

    // Print to console.
    print!("\n\nHello {}\n", "World!");
    print!("1 + 2 = {}, {:#x}\n", 1 + 2, 0x1234abcd);

    loop {}
}

#[no_mangle]
pub extern "C" fn boot() {
    unsafe {
        asm!("mv sp, {}", "j kernel_main", in(reg) &__stack_top);
    }
}
