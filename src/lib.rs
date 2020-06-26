/*!

*/

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

extern crate utf8_width;

mod decode;
mod encode;
mod functions;

pub use decode::*;
pub use encode::*;
