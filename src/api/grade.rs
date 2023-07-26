
use std::fmt::{self, Display, Formatter};

const DEFAULT_ZONE: u8 = 2;
const HPS_ZONE: u8 = 3;

/// Material grade
#[derive(Debug)]
pub struct Grade<'a> {
    spec: &'a str,
    grade: &'a str,
    test: Test,
    zone: Option<u8>
}

impl<'a> Grade<'a> {
    /// Crates a new grade from a given spec, grade, test and zone
    pub fn new(spec: &str, grade: &str, test: &str, mut zone: Option<u8>) -> Self {
        match spec {
            "A240 Type 304" => Self::new("A240", "304",   "", None),
            "A240 Type 316" => Self::new("A240", "316",   "", None),
            "A606-TYPE4"    => Self::new("A606", "TYPE4", "", None),
            _ => {
                if grade.contains("HPS") {
                    zone = Some(HPS_ZONE);
                }

                let test = match spec {
                    "A240" | "A606" => Test::NotApplicable,
                    _ => test.into()
                };
        
                Self { spec, grade, test, zone }
            }
        }
    }

    /// Coerces non-charpy materials to charpy (i.e. A709-50 as A709-50T2).
    /// 
    /// Useful for Sigmanest, where all plate is charpy at the least.
    /// Note that materials that are not applicable to charpy (i.e. A240-304)
    /// will not return with the charpy designation.
    pub fn force_cvn(&self) -> String {
        match self.test {
            Test::None => format!("{}-{}{:}{}", self.spec, self.grade, Test::Charpy, self.zone.unwrap_or(DEFAULT_ZONE)),
            _          => format!("{:}", self)
        }
    }

    /// Check if a part requires charpy testing
    pub fn requires_charpy(&self) -> bool {
        match self.test {
            Test::Charpy | Test::Fcm => true,
            _ => false
        }
    }
}

impl Display for Grade<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.test {
            Test::None          => write!(f, "{}-{}", self.spec, self.grade),
            Test::NotApplicable => write!(f, "{}-{}", self.spec, self.grade),
            _                   => write!(f, "{}-{}{:}{}", self.spec, self.grade, self.test, self.zone.unwrap_or(DEFAULT_ZONE))
        }
    }
}

#[derive(Debug)]
enum Test {
    Fcm,
    Charpy,
    None,
    NotApplicable
}

impl From<&str> for Test {
    fn from(test: &str) -> Self {
        match test {
            "FCM" => Test::Fcm,
            "T"   => Test::Charpy,
            _     => Test::None
        }
    }
}

impl Default for Test {
    fn default() -> Self {
        Test::None
    }
}

impl Display for Test {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let val = match self {
            Test::Fcm           => "F",
            Test::Charpy        => "T",
            Test::None          => "",
            Test::NotApplicable => ""
        };

        write!(f, "{}", val)
    }
}
