
use regex::Regex;
use std::{
    cell::LazyCell,
    fmt::{self, Display, Formatter},
    str::FromStr
};

const MVMT_RE: LazyCell<Regex> = LazyCell::new(|| {
    Regex::new(r"^(\d{3})(Q)?$")
        .expect("failed to build MovementType parsing regex")
});

/// SAP movement type
/// 
/// id: 3-digit movement type
/// project: if this is a Q-variant
/// 
/// example: 261Q -> MovementType { id: 261, project: true }
#[derive(Debug)]
pub struct MovementType {
    id: u16,
    project: bool
}

impl FromStr for MovementType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match MVMT_RE.captures(s) {
            Some(cap) => {
                // `cap.get(n).unwrap()` should not fail
                let id = match cap.get(1).unwrap().as_str().parse() {
                    Ok(job) => job,
                    Err(_) => return Err(format!("Failed to parse Movement id from `{}`", s))
                };
                let project = cap.get(2).is_some();
                
                Ok( Self { id, project } )
            },
            None => {
                log::error!("Failed to parse MovementType from `{}`", s);

                Err(format!("Failed to parse SAP movement type from `{}`", s))
            }
        }
    }
}

impl Display for MovementType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.id, if self.project {"Q"} else {""})
    }
}

impl PartialEq<&str> for MovementType {
    fn eq(&self, other: &&str) -> bool {
        other == &self.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::MovementType;

    #[test]
    fn test_eq_project() {
        let test_against = MovementType { id: 261, project: true };

        assert_eq!(test_against, "261Q");
        assert_ne!(test_against, "261");
        assert_ne!(test_against, "221Q");
        assert_ne!(test_against, "Q");
    }

    #[test]
    fn test_eq_nonproject() {
        let test_against = MovementType { id: 261, project: false };

        assert_eq!(test_against, "261");
        assert_ne!(test_against, "261Q");
        assert_ne!(test_against, "221");
        assert_ne!(test_against, "");
    }
}
