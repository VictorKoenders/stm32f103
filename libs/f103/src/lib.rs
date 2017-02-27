#![no_std]

extern crate peripheral as p;

pub use p as peripheral;
pub mod gpio;

pub enum Side {
    A,
    B,
    C
}

pub enum Pin {
    A(u8),
    B(u8),
    C(u8)
}

