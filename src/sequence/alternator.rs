use itertools::iproduct;

use crate::Term;

/// Sequence produced by alternating an operation
#[derive(Debug, PartialEq)]
pub struct Alternator {
    pub start: Term,
    pub a: Term,
    pub b: Term,
    pub c: Term,
    pub d: Term,
}

impl TryFrom<&[Term]> for Alternator {
    type Error = ();

    fn try_from(value: &[Term]) -> Result<Self, Self::Error> {
        if value.len() < 3 {
            return Err(());
        }

        Err(())
    }
}
