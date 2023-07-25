
// linting directives (see https://doc.rust-lang.org/rustc/lints/index.html)
#![warn(missing_docs)]
#![warn(missing_debug_implementations)]
#![warn(noop_method_call, unreachable_pub)]
#![warn(variant_size_differences)]
#![deny(deprecated, legacy_derive_helpers)]
#![deny(non_ascii_idents)]

//! Middle manager for inventory management
//! 
//! todo:
//! - [ ] interfaces
//!     - [ ] SAP
//!     - [ ] Sigmanest
//! - [ ] plugin system
//! - [ ] database

#![feature(lazy_cell)]

pub mod api;
pub mod flows;
pub mod logging;
pub mod db;
