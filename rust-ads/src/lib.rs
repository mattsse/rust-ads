#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
#![allow(warnings)]
extern crate bincode;
extern crate byteorder;
extern crate chrono;
extern crate serde;
#[macro_use]
extern crate serde_derive;

pub mod core;
pub mod api;

pub mod prelude {
    pub use super::core::*;
    pub use super::api::*;
}
