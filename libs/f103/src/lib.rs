#![no_std]
#![deny(warnings)]
#![feature(asm)]

#[macro_use] pub mod macros;
pub mod gpio;
pub mod address;

static TRUE: bool = true;
static FALSE: bool = false;

#[export_name = "__aeabi_memcpy"]
#[no_mangle]
pub unsafe extern fn memcpy(dest: *mut u8, src: *const u8, n: isize) -> *mut u8 {
    for i in 0..n {
        *dest.offset(i) = *src.offset(i)
    }
    return dest
}

#[export_name = "__aeabi_memclr4"]
#[no_mangle]
pub unsafe extern fn memclr4(dest: *mut u8, n: isize) -> *mut u8 {
    for i in 0..n {
        *dest.offset(i) = 0;
    }
    return dest
}

