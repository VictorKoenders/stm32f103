#![cfg_attr(test, feature(test))]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)] extern crate test;

#[cfg(feature = "std")]
mod iter_slice_before_after;
#[cfg(feature = "std")]
pub use iter_slice_before_after::iter_slice_before_after;