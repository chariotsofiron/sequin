use itertools::iproduct;

use crate::Term;

#[derive(Debug, PartialEq)]
pub struct Binomial {
    pub start: Term,
    pub a: Term,
    pub b: Term,
}

impl TryFrom<&[Term]> for Binomial {
    type Error = ();

    fn try_from(value: &[Term]) -> Result<Self, Self::Error> {
        if value.len() < 3 {
            return Err(());
        }

        Err(())
    }
}
