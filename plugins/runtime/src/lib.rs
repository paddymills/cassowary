
// linting directives (see https://doc.rust-lang.org/rustc/lints/index.html)
#![warn(missing_docs)]
#![warn(missing_debug_implementations)]
#![warn(noop_method_call, unreachable_pub)]
#![warn(variant_size_differences)]
#![deny(deprecated, legacy_derive_helpers)]
#![deny(non_ascii_idents)]

mod server;
mod client;

pub use server::Server;
pub use client::Client;
