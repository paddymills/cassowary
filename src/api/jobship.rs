
use regex::Regex;
use std::{
    cell::LazyCell,
    fmt::{self, Display, Formatter},
    str::FromStr
};

const JOBSHIP_RE: LazyCell<Regex> = LazyCell::new(|| {
    Regex::new(r"^(\d{7})([[:alpha:]])?-(\d+)$")
        .expect("failed to build JobShipment parsing regex")
});

/// Job and Shipment
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct JobShipment {
    /// Job number
    pub job: u32,
    
    /// Structure letter
    pub structure: Option<String>,

    /// Shipment number
    pub shipment: u8
}

impl FromStr for JobShipment {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match JOBSHIP_RE.captures(s) {
            Some(cap) => {
                // `cap.get(n).unwrap()` should not fail
                let job = match cap.get(1).unwrap().as_str().parse() {
                    Ok(job) => job,
                    Err(_) => return Err(format!("Failed to parse Job from `{}`", s))
                };
                let structure = cap.get(2).map(|m| m.as_str().into());
                let shipment = match cap.get(3).unwrap().as_str().parse() {
                    Ok(job) => job,
                    Err(_) => return Err(format!("Failed to parse Shipment from `{}`", s))
                };
                
                Ok( Self { job, structure, shipment } )
            },
            None => {
                error!("Failed to parse JobShipment from `{}`", s);

                Err(format!("Failed to parse Job, Structure and Shipment from `{}`", s))
            }
        }
    }
}

impl Display for JobShipment {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}-{}", self.job, self.structure.as_ref().map_or("", |s| s.as_str()), self.shipment)
    }
}
