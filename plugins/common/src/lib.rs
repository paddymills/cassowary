
// linting directives (see https://doc.rust-lang.org/rustc/lints/index.html)
#![warn(missing_docs)]
#![warn(missing_debug_implementations)]
#![warn(noop_method_call, unreachable_pub)]
#![warn(variant_size_differences)]
#![deny(deprecated, legacy_derive_helpers)]
#![deny(non_ascii_idents)]

//! Cassowary plugin interface and communication types
//! 
//! # Cassowary interfacing via a plugin
//! 
//! A interfacing with Cassowary should be done via a plugin. A plugin should implement
//! the [`CassowaryPlugin`] trait to ensure proper communication with the plugin runtime.
//! 
//! While there is no strict enforcement of the plugin being implemented for the plugin (yet),
//! implementing this trait ensures that proper handling of encoding/decoding of messages
//! over the TCP stream happen. The plugin trait also contains convenience methods for starting
//! the TCP stream and ensuring propper communicate. While there is no requirement that
//! a plugin uses the interface and its associated methods, following the design of
//! this trait are paramount for plugin and runtime communication.
//! 
//! [`CassowaryPlugin`]: crate::CassowaryPlugin


#[macro_use] extern crate serde;

mod interface;
mod message;

pub use interface::CassowaryPlugin;
pub use message::*;
