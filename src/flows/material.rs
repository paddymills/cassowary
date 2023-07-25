
//! Material movement flows

use crate::api::sap::{MovementType, Plant};

/// Goods receipt
#[derive(Debug)]
pub struct MigoGr {
    /// 101 for receipt, 102 for reversal
    // TODO: make into enum
    trans: MovementType,

    /// Storage location
    location: String,   // TODO: custom location type

    /// Plant (HS01, HS02)
    plant: Plant,

    /// Material Master
    // maybe make MM and WBS a singular material type
    mm: String,
    /// Material WBS element
    wbs: Option<String>, // TODO: wbs struct

    /// Material Heat number
    heat: String,
    /// Material PO Number
    po: String,
}

/// Goods Issue
#[derive(Debug)]
pub struct MigoGi {
    /// SAP transaction (221, 261Q, etc.)
    // TODO: make movement enum
    trans: MovementType,

    /// Storage location
    location: String,   // TODO: custom location type

    /// Plant (HS01, HS02)
    plant: Plant,

    /// Material Master
    // maybe make MM and WBS a singular material type
    mm: String,
    /// Material WBS element
    wbs: Option<String>, // TODO: wbs struct
}

/// Movement types
#[derive(Debug)]
pub enum Movement {
    /// Goods Receipt (MIGH_GR)
    GoodsReceipt(MigoGr),
    /// Goods Issue (MIGH_GI)
    GoodsIssue(MigoGi)
}
