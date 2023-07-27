
//! Material movement flows

use super::{RawMaterial, MtrIdentifier};


/// Movement types for raw material
#[derive(Debug)]
pub enum RawMaterialMovement {
    /// Goods Receipt (MIGO_GR)
    GoodsReceipt {
        /// Material data
        material: RawMaterial,
        /// Heat number data
        mtr: MtrIdentifier,
        /// Number of pieces
        qty: u32
    },

    /// Goods Issue (MIGO_GI)
    GoodsIssue {
        /// Material data
        material: RawMaterial,
        /// qty of material issued
        qty: f64
    },
    
    /// Goods Transfer (MIGO_TR)
    GoodsTransfer {
        /// Material data being transfered from
        from: RawMaterial,
        /// Material data being transfered to
        to: RawMaterial,
        /// qt of material transfered
        qty: f64
    }
}
