
//! Material Test Reports

/// Material Test Report data
#[derive(Debug, PartialEq)]
pub struct MtrIdentifier {
    /// Material Heat number
    pub heat: String,
    /// Material PO Number
    pub po: u64,
    /// Bill of lading
    pub bol: String
}

impl TryFrom<(&str, &str, &str)> for MtrIdentifier {
    type Error = std::num::ParseIntError;

    fn try_from(value: (&str, &str, &str)) -> Result<Self, Self::Error> {
        Ok( Self {
            heat: String::from( value.0 ),
            po: value.1.parse()?,
            bol: String::from( value.2 )
        } )
    }
}

#[cfg(test)]
mod tests {
    use super::MtrIdentifier;

    #[test]
    fn test_po_parse() {
        let test_against = Ok(MtrIdentifier { heat: "D1234".into(), po: 4500123456, bol: "11".into() });
        assert_eq!(("D1234", "4500123456", "11").try_into(), test_against);
        
        assert!(MtrIdentifier::try_from(("D1234", "4500x", "11")).is_err());
    }
}
