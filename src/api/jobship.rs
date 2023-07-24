
use regex::Regex;
use std::{
    cell::LazyCell,
    fmt::{self, Display, Formatter},
    str::FromStr
};

const JOBSHIP_RE: LazyCell<Regex> = LazyCell::new(|| {
    Regex::new(r"^(\d{7}[[:alpha:]])-(\d+)$").expect("failed to build regex")
});

/// Job and Shipment
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct JobShipment {
    /// Job number (with structure letter)
    // TODO: split job into project and structure
    pub job: String,
    /// Shipment number
    // TODO: refactor as number
    pub shipment: u8
}

impl FromStr for JobShipment {
    type Err = std::fmt::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match JOBSHIP_RE.captures(s) {
            Some(cap) => {
                Ok(
                    Self {
                        // TODO: handle the unwraps
                        job: cap.get(1).unwrap().as_str().to_uppercase().into(),
                        shipment: cap.get(2).unwrap().as_str().parse().unwrap(),
                    }
                )
            },
            None => {
                eprintln!("Failed to parse job-shipment: {}", s);

                // TODO: don't panic (custom error?)
                panic!("invalid job-shipment")
            }
        }
    }
}

impl Display for JobShipment {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}", self.job, self.ship)
    }
}
