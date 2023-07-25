
#![warn(missing_docs)]
#![warn(missing_debug_implementations)]

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
