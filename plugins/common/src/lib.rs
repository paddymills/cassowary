
#![feature(async_fn_in_trait)]

#![warn(missing_docs)]
#![warn(missing_debug_implementations)]
#![warn(noop_method_call, unreachable_pub)]
#![warn(variant_size_differences)]
#![deny(deprecated, legacy_derive_helpers)]
#![deny(non_ascii_idents)]

//! Cassowary plugin interface and communication types
//! 
//! A plugin must implement the [`CassowaryPlugin`] trait to ensure proper communication with the
//! plugin runtime. While there is no strict enforcement of the plugin being
//! implemented for the plugin (yet), implementing this trait ensures that proper
//! handling of encoding/decoding of messages over the TCP stream happen.
//! 
//! [`CassowaryPlugin`]: crate::CassowaryPlugin


#[macro_use] extern crate serde;

mod interface;
mod message;

pub use interface::CassowaryPlugin;
pub use message::Message;
