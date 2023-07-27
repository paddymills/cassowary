

use std::fmt::{self, Display, Formatter};
use super::Material;

/// Part (piecemark)
#[derive(Debug, Default)]
pub struct Part<'a> {
    /// Piecemark
    pub mark: String,
    /// Quantity
    pub qty: i32,

    /// Drawing name
    pub dwg: Option<String>,
    /// Description
    pub desc: Option<String>,
    /// Geometry information
    pub matl: Option<Material<'a>>,
    
    /// Additional remarks
    pub remark: Option<String>
}

impl Part<'_> {
    /// Create a new part from a given mark
    pub fn new(mark: String) -> Self {
        Self { mark, ..Default::default() }
    }

    /// Is part a plate
    /// Re-elevated from [`Material`]
    /// 
    /// ['Material']: crate::Material
    pub fn is_pl(&self) -> bool {
        self.matl.as_ref().map_or(false, |m| m.is_pl())
    }
}

impl Display for Part<'_> {
    /// Displays the part in the format `{mark} ({material}) [{material.grade}]`
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self.matl {
            Some(matl) => write!(f, "{} ({}) [{}]", self.mark, matl, matl.grade),
            None       => write!(f, "{}", self.mark)
        }
    }
}
