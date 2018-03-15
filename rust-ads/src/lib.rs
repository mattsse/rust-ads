#![allow(warnings)]
extern crate bincode;
#[macro_use]
extern crate serde_derive;

pub mod core;
pub mod api;

pub mod prelude {
    pub use super::core::*;
    pub use super::api::*;
}
