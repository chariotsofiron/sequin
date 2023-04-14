use crate::Fraction;

use super::{binom::Binom, differences::Differences, zipped::Zipped};

pub enum SimpleSequence;

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

    macro_rules! frac {
        ( $( $x:expr ),* ) => {
            vec![
                $( Fraction::from($x), )*
            ]
        };
    }

    #[test]
    fn test_seqs() {
        use super::Fraction;
        use super::*;

        let test_case = [
            // binomial
            (
                frac![3, 8, 23, 68, 203],
                Sequence::Binom(Binom {
                    start: Fraction::from(3),
                    a: Fraction::from(3),
                    b: Fraction::from(-1),
                }),
            ),
            (
                frac![18, 108, 648, 3888],
                Sequence::Binom(Binom {
                    start: Fraction::from(18),
                    a: Fraction::from(6),
                    b: Fraction::from(0),
                }),
            ),
            (
                frac![68, 36, 20, 12, 8],
                Sequence::Binom(Binom {
                    start: Fraction::from(68),
                    a: Fraction::new(1, 2),
                    b: Fraction::from(2),
                }),
            ),
            (
                frac![8, 20, 50, 125],
                Sequence::Binom(Binom {
                    start: Fraction::from(8),
                    a: Fraction::new(5, 2),
                    b: Fraction::from(0),
                }),
            ),
            (
                frac![4, -8, 16, -32],
                Sequence::Binom(Binom {
                    start: Fraction::from(4),
                    a: Fraction::from(-2),
                    b: Fraction::from(0),
                }),
            ),
            // zipped binomial
            (
                frac![2, 5, 6, 20, 18, 80, 54, 320, 162, 1280, 486],
                Sequence::Zipped(Zipped {
                    seqs: vec![
                        Sequence::Binom(Binom {
                            start: Fraction::from(2),
                            a: Fraction::from(3),
                            b: Fraction::from(-1),
                        }),
                        Sequence::Binom(Binom {
                            start: Fraction::from(5),
                            a: Fraction::from(4),
                            b: Fraction::from(-1),
                        }),
                    ],
                }),
            ),
            // differences
            (
                frac![-3, 3, 27, 69, 129, 207],
                Sequence::Differences(Differences {
                    diffs: frac![-3, 6, 18],
                }),
            ),
            (
                frac![9, 73, 241, 561, 1081, 1849, 2913],
                Sequence::Differences(Differences {
                    diffs: frac![9, 64, 104, 48],
                }),
            ),
        ];

        for (input, expected) in test_case.into_iter() {
            let seq = Sequence::try_from(input.as_slice()).unwrap();
            assert_eq!(seq, expected);
            assert_eq!(seq.into_iter().take(input.len()).collect::<Vec<_>>(), input);
        }
    }
}
