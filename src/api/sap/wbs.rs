
//! WBS element

use regex::Regex;
use std::{
    cell::LazyCell,
    fmt::{self, Display, Formatter},
    str::FromStr
};

use crate::api::JobShipment;

const WBS_RE: LazyCell<Regex> = LazyCell::new(|| {
    Regex::new(r"^D-(\d{7})-(\d{5})$")
        .expect("failed to build MovementType parsing regex")
});
const LEGACY_WBS_RE: LazyCell<Regex> = LazyCell::new(|| {
    Regex::new(r"^S-(\d{7})-2-(\d{2})$")
        .expect("failed to build MovementType parsing regex")
});

/// SAP WBS Element
#[derive(Debug, PartialEq)]
pub enum Wbs {
    /// Current WBS scheme for Hard Dollar system
    HardDollar {
        /// Job number
        job: u32,
        
        /// Hard Dollar line id
        id: u32
    },

    /// Legacy WBS scheme
    // JobShipment is boxed to keep variant size down
    Legacy(Box<JobShipment>)
}

impl FromStr for Wbs {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(caps) = WBS_RE.captures(s) {
            // `parse().unwrap()` should not fail because the regex validated that it is a number
            let job = caps.get(1).unwrap().as_str().parse().unwrap();
            let id = caps.get(2).unwrap().as_str().parse().unwrap();

            return Ok( Self::HardDollar { job, id } );
        }

        if let Some(caps) = LEGACY_WBS_RE.captures(s) {
            // `parse().unwrap()` should not fail because the regex validated that it is a number
            let job = caps.get(1).unwrap().as_str().parse().unwrap();
            let shipment: u8 = caps.get(2).unwrap().as_str().parse().unwrap();

            return Ok( Self::Legacy(Box::new(JobShipment { job, structure: None, shipment })) );
        }

        Err(format!("Failed to parse WBS element from `{}`", s))
    }
}

impl Display for Wbs {
    /// Display WBS element in its known pattern
    /// 
    /// Can be used as a way to covert a WBS element back to its text format
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::HardDollar { job, id } => write!(f, "D-{}-{}", job, id),
            Self::Legacy(job_shipment) => write!(f, "S-{}-2-{:02}", job_shipment.job, job_shipment.shipment)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(Ok(Wbs::HardDollar { job: 1220111, id: 10001 }), "D-1220111-10001".parse());
        assert_eq!(Ok(Wbs::Legacy(Box::new("1220111-04".parse().unwrap()))), "S-1220111-2-04".parse());
    }
}