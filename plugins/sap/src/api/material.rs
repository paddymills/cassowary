
//! SAP material data

use super::{Plant, Wbs};

/// SAP raw material
#[derive(Debug, PartialEq)]
pub struct RawMaterial {
    /// Storage location
    pub location: String,

    /// Plant (HS01, HS02)
    pub plant: Plant,

    /// Material Master
    pub mm: String,
    /// Material WBS element
    pub wbs: Option<Wbs>
}
