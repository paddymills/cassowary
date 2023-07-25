
#![warn(missing_docs)]
#![warn(missing_debug_implementations)]
#![warn(unreachable_pub)]
#![warn(variant_size_differences)]
#![deny(deprecated, legacy_derive_helpers)]

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
