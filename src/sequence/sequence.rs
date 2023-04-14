use crate::Fraction;

use super::{binom::Binom, differences::Differences, zipped::Zipped};

#[derive(Debug, PartialEq)]
pub enum Sequence {
    Binom(Binom),
    Differences(Differences),
    Zipped(Zipped),
}

impl IntoIterator for Sequence {
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

impl TryFrom<&[Fraction]> for Sequence {
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
    #[test]
    fn test_seqs() {
        use super::Fraction;
        use super::*;

        let test_case = [
            (
                vec![3, 8, 23, 68, 203],
                Sequence::Binom(Binom {
                    start: Fraction::from(3),
                    a: Fraction::from(3),
                    b: Fraction::from(-1),
                }),
            )

        ];

        for (input, expected) in test_case.iter() {
            let input = input.iter().map(|&x| Fraction::from(x)).collect::<Vec<_>>();
            let seq = Sequence::try_from(input.as_slice()).unwrap();
            assert_eq!(seq, *expected);
        }
    }
}