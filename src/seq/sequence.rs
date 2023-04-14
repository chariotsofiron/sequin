use crate::Fraction;

use super::binom::Binom;

#[derive(Debug)]
pub enum SimpleSequence {
    Binom(Binom),
}

impl IntoIterator for SimpleSequence {
    type Item = Fraction;
    type IntoIter = Box<dyn Iterator<Item = Fraction>>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            Self::Binom(seq) => Box::new(seq.into_iter()),
        }
    }
}

impl TryFrom<&[Fraction]> for SimpleSequence {
    type Error = ();

    fn try_from(value: &[Fraction]) -> Result<Self, Self::Error> {
        if let Ok(seq) = Binom::try_from(value) {
            return Ok(Self::Binom(seq));
        }
        Err(())
    }
}
