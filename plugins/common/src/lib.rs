
#![feature(async_fn_in_trait)]

#![warn(missing_docs)]
#![warn(missing_debug_implementations)]
#![warn(noop_method_call, unreachable_pub)]
#![warn(variant_size_differences)]
#![deny(deprecated, legacy_derive_helpers)]
#![deny(non_ascii_idents)]

//! Cassowary plugin interface and communication types

#[macro_use] extern crate serde;

mod interface;
mod message;

pub use interface::CassowaryPlugin;
pub use message::Message;
