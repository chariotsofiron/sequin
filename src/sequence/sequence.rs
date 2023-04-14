use crate::Fraction;

use super::{binom::Binom, differences::Differences, zipped::Zipped};

#[derive(Debug)]
pub enum SimpleSequence {
    Binom(Binom),
    Differences(Differences),
    Zipped(Zipped),
}

impl IntoIterator for SimpleSequence {
    type Item = Fraction;
    type IntoIter = Box<dyn Iterator<Item = Fraction>>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            Self::Binom(seq) => Box::new(seq.into_iter()),
            Self::Differences(seq) => Box::new(seq.into_iter()),
            Self::Zipped(seq) => Box::new(seq.into_iter()),
        }
    }
}

impl TryFrom<&[Fraction]> for SimpleSequence {
    type Error = ();

    fn try_from(value: &[Fraction]) -> Result<Self, Self::Error> {
        if let Ok(seq) = Binom::try_from(value) {
            return Ok(Self::Binom(seq));
        } else if let Ok(seq) = Differences::try_from(value) {
            return Ok(Self::Differences(seq));
        } else if let Ok(seq) = Zipped::try_from(value) {
            return Ok(Self::Zipped(seq));
        }
        Err(())
    }
}


#[cfg(test)]
mod tests {
    fn test_seqs() {
        use super::Fraction;
        use super::*;

        
    }
}