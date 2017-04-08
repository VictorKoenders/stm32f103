#![feature(asm)]
#![feature(lang_items)]

#![no_main]
#![no_std]

#[macro_export]
macro_rules! bkpt {
    () => {
        unsafe { asm!("bkpt" :::: "volatile") }
    };
    ($imm:expr) => {
        unsafe { asm!(concat!("bkpt #", stringify!($imm)) :::: "volatile") }
    };
}

pub mod exceptions;
pub mod interrupts;


#[inline(never)]
#[no_mangle]
#[export_name = "main"]
pub fn main() -> ! {
    bkpt!();
    loop {}
}

#[repr(C)]
pub struct StackFrame {
    /// (General purpose) Register 0
    pub r0: u32,
    /// (General purpose) Register 1
    pub r1: u32,
    /// (General purpose) Register 2
    pub r2: u32,
    /// (General purpose) Register 3
    pub r3: u32,
    /// (General purpose) Register 12
    pub r12: u32,
    /// Linker Register
    pub lr: u32,
    /// Program Counter
    pub pc: u32,
    /// Program Status Register
    pub xpsr: u32,
}

#[doc(hidden)]
#[export_name = "_default_exception_handler"]
pub extern "C" fn default_handler(_sf: &StackFrame) -> ! {
    bkpt!();

    loop {}
}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn rust_begin_panic(_msg: core::fmt::Arguments,
                               _file: &'static str,
                               _line: u32) -> ! {
    use core::fmt::{Result, Write};
    struct Test;
    impl Write for Test {
        fn write_str(&mut self, _s: &str) -> Result {
            bkpt!();
            Ok(())
        }
    }

    let _ = Test{}.write_fmt(_msg);
    bkpt!();
    loop {}
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern fn rust_eh_personality() {
}

#[lang = "eh_unwind_resume"]
#[no_mangle]
pub extern fn rust_eh_unwind_resume() {
}
