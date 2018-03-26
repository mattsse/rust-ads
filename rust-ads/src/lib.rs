#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
#![allow(warnings)]
extern crate bincode;
extern crate byteorder;
extern crate chrono;
extern crate serde;
#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate num_derive;
extern crate num_traits;

pub mod api;
pub mod core;

pub mod prelude {
    pub use super::api::*;
    pub use super::core::*;
}
