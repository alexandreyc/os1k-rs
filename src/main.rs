#![no_std]
#![no_main]
// Unstable features.
#![feature(fn_align)]
#![feature(naked_functions)]

use core::arch::{asm, naked_asm};
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

macro_rules! read_csr {
    ($reg:tt) => {{
        let value: u32;
        unsafe {
            asm!(concat!("csrr {}, ", stringify!($reg)), out(reg) value);
        }
        value
    }};
}

macro_rules! write_csr {
    ($reg:tt, $val:expr) => {
        unsafe {
            asm!(concat!("csrw ", stringify!($reg), ", {}"), in(reg) $val);
        }
    };
}

#[repr(packed)]
struct TrapFrame {
    ra: u32,
    gp: u32,
    tp: u32,
    t0: u32,
    t1: u32,
    t2: u32,
    t3: u32,
    t4: u32,
    t5: u32,
    t6: u32,
    a0: u32,
    a1: u32,
    a2: u32,
    a3: u32,
    a4: u32,
    a5: u32,
    a6: u32,
    a7: u32,
    s0: u32,
    s1: u32,
    s2: u32,
    s3: u32,
    s4: u32,
    s5: u32,
    s6: u32,
    s7: u32,
    s8: u32,
    s9: u32,
    s10: u32,
    s11: u32,
    sp: u32,
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
#[naked]
extern "C" fn kernel_entry() {
    unsafe {
        naked_asm!(
            "csrw sscratch, sp",
            "addi sp, sp, -4 * 31",
            "sw ra, 4 * 0(sp)",
            "sw gp, 4 * 1(sp)",
            "sw tp, 4 * 2(sp)",
            "sw t0, 4 * 3(sp)",
            "sw t1, 4 * 4(sp)",
            "sw t2, 4 * 5(sp)",
            "sw t3, 4 * 6(sp)",
            "sw t4, 4 * 7(sp)",
            "sw t5, 4 * 8(sp)",
            "sw t6, 4 * 9(sp)",
            "sw a0, 4 * 10(sp)",
            "sw a1, 4 * 11(sp)",
            "sw a2, 4 * 12(sp)",
            "sw a3, 4 * 13(sp)",
            "sw a4, 4 * 14(sp)",
            "sw a5, 4 * 15(sp)",
            "sw a6, 4 * 16(sp)",
            "sw a7, 4 * 17(sp)",
            "sw s0, 4 * 18(sp)",
            "sw s1, 4 * 19(sp)",
            "sw s2, 4 * 20(sp)",
            "sw s3, 4 * 21(sp)",
            "sw s4, 4 * 22(sp)",
            "sw s5, 4 * 23(sp)",
            "sw s6, 4 * 24(sp)",
            "sw s7, 4 * 25(sp)",
            "sw s8, 4 * 26(sp)",
            "sw s9, 4 * 27(sp)",
            "sw s10, 4 * 28(sp)",
            "sw s11, 4 * 29(sp)",
            "csrr a0, sscratch",
            "sw a0, 4 * 30(sp)",
            "mv a0, sp",
            "call handle_trap",
            "lw ra, 4 * 0(sp)",
            "lw gp, 4 * 1(sp)",
            "lw tp, 4 * 2(sp)",
            "lw t0, 4 * 3(sp)",
            "lw t1, 4 * 4(sp)",
            "lw t2, 4 * 5(sp)",
            "lw t3, 4 * 6(sp)",
            "lw t4, 4 * 7(sp)",
            "lw t5, 4 * 8(sp)",
            "lw t6, 4 * 9(sp)",
            "lw a0, 4 * 10(sp)",
            "lw a1, 4 * 11(sp)",
            "lw a2, 4 * 12(sp)",
            "lw a3, 4 * 13(sp)",
            "lw a4, 4 * 14(sp)",
            "lw a5, 4 * 15(sp)",
            "lw a6, 4 * 16(sp)",
            "lw a7, 4 * 17(sp)",
            "lw s0, 4 * 18(sp)",
            "lw s1, 4 * 19(sp)",
            "lw s2, 4 * 20(sp)",
            "lw s3, 4 * 21(sp)",
            "lw s4, 4 * 22(sp)",
            "lw s5, 4 * 23(sp)",
            "lw s6, 4 * 24(sp)",
            "lw s7, 4 * 25(sp)",
            "lw s8, 4 * 26(sp)",
            "lw s9, 4 * 27(sp)",
            "lw s10, 4 * 28(sp)",
            "lw s11, 4 * 29(sp)",
            "lw sp, 4 * 30(sp)",
            "sret",
        );
    }
}

#[no_mangle]
extern "C" fn handle_trap(_frame: &TrapFrame) {
    let scause = read_csr!(scause);
    let stval = read_csr!(stval);
    let sepc = read_csr!(sepc);
    panic!(
        "unexpected trap scause={:#010x} stval={:#010x} sepc={:#010x}",
        scause, stval, sepc
    );
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

    // Register exception handler.
    write_csr!(stvec, kernel_entry as u32);

    // Test exception handler.
    unsafe {
        asm!("unimp");
    }

    loop {}
}

#[no_mangle]
pub extern "C" fn boot() {
    unsafe {
        asm!("mv sp, {}", "j kernel_main", in(reg) &__stack_top);
    }
}
