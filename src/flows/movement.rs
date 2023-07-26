
use regex::Regex;
use std::{
    cell::LazyCell,
    str::FromStr
};

const MVMT_RE: LazyCell<Regex> = LazyCell::new(|| {
    Regex::new(r"^(\d{3})(Q)?$")
        .expect("failed to build MovementType parsing regex")
});

/// parses a movement string into an `( id, project )` pair
/// 
/// id: 3-digit movement type
/// project: if this is a Q-variant
/// 
/// example: 261Q -> ( 261, true )
fn parse_movement(s: &str) -> Result<(u16, bool), String> {
    match MVMT_RE.captures(s) {
        Some(cap) => {
            // `cap.get(1).unwrap()` should not fail due to being a match
            // `.parse().unwrap()` should not failt due to regex validation
            let id = cap.get(1).unwrap().as_str().parse().unwrap();
            let project = cap.get(2).is_some();
            
            Ok( (id, project) )
        },
        None => {
            log::error!("Failed to parse MovementType from `{}`", s);

            Err(format!("Failed to parse SAP movement type from `{}`", s))
        }
    }
}

/// SAP movement type
/// 
/// represented as `MovementType(is_project_stock: bool)`
#[derive(Debug)]
pub enum MovementType {
    /// MIGO_GR 101 - receiving of material
    MigoGrReceipt(bool),
    /// MIGO_GR 102 - reversal of a material receiving
    MigoGrReversal(bool),

    /// MIGO_GI 261
    MigoGiConsumption(bool)
}

impl FromStr for MovementType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (id, is_project) = parse_movement(s)?;
        match id {
            101 => Ok( Self::MigoGrReceipt(is_project) ),
            102 => Ok( Self::MigoGrReversal(is_project) ),

            261 => Ok( Self::MigoGiConsumption(is_project) ),

            _ => Err(format!("Unmatched movement id for SAP movement type `{}`", s))
        }
    }
}

