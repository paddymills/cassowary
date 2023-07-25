
//! Material movement flows

use crate::api::sap::{MovementType, Plant};

/// Goods receipt
#[derive(Debug)]
pub struct MigoGr {
    /// 101 for receipt, 102 for reversal
    trans: MovementType,
    location: String,   // TODO: custom location type
    plant: Plant,

    // maybe make MM and WBS a singular material type
    mm: String,
    wbs: Option<String>, // TODO: wbs struct

    heat: String,
    po: String,
}

/// Goods Issue
#[derive(Debug)]
pub struct MigoGi {
    trans: u8,
    location: String,
    plant: String,
    mm: String,
    wbs: Option<String>,
}

/// Movement types
#[derive(Debug)]
pub enum Movement {
    GoodsReceipt(MigoGr),
    GoodsIssue(MigoGi)
}
