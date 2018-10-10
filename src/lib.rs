// #![deny(warnings)]
// TODO remove, allow warnings for now
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
extern crate bincode;
extern crate byteorder;
extern crate chrono;
extern crate serde;

#[macro_use]
extern crate num_derive;
extern crate num_traits;

pub mod core;

pub mod prelude {
    pub use super::core::*;
}
