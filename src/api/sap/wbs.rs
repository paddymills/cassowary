

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
#[derive(Debug)]
pub enum Wbs {
    HardDollar { job: u32, id: u32 },
    Legacy(JobShipment)
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

            return Ok( Self::Legacy(JobShipment { job, structure: None, shipment }) );
        }

        Err(format!("Failed to parse WBS element from `{}`", s))
    }
}

impl Display for Wbs {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::HardDollar { job, id } => write!(f, "D-{}-{}", job, id),
            Self::Legacy(job_shipment) => write!(f, "S-{}-2-{}", job_shipment.job, job_shipment.shipment)
        }
    }
}


