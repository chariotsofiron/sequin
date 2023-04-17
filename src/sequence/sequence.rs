use crate::Term;

use super::{binom::Binom, differences::Differences, zipped::Zipped};

#[derive(Debug, PartialEq)]
pub enum Sequence {
    Differences(Differences),
    Binom(Binom),
    Zipped(Zipped),
}

impl IntoIterator for Sequence {
    type Item = Term;
    type IntoIter = Box<dyn Iterator<Item = Term>>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            Self::Differences(seq) => Box::new(seq.into_iter()),
            Self::Binom(seq) => Box::new(seq.into_iter()),
            Self::Zipped(seq) => Box::new(seq.into_iter()),
        }
    }
}

impl TryFrom<&[Term]> for Sequence {
    type Error = ();

    fn try_from(value: &[Term]) -> Result<Self, Self::Error> {
        if let Ok(seq) = Differences::try_from(value) {
            return Ok(Self::Differences(seq));
        } else if let Ok(seq) = Binom::try_from(value) {
            return Ok(Self::Binom(seq));
        } else if let Ok(seq) = Zipped::try_from(value) {
            return Ok(Self::Zipped(seq));
        }
        Err(())
    }
}

#[cfg(test)]
mod tests {
    use super::Sequence;
    use super::Term;
    use super::*;
    use crate::sequence::{differences::Differences, zipped::Zipped};

    macro_rules! frac {
        ( $( $x:expr ),* ) => {
            vec![
                $( Term::from($x), )*
            ]
        };
    }

    #[test]
    fn test_seqs() {
        let test_cases = [
            // binomial
            (
                frac![3, 8, 23, 68, 203],
                Sequence::Binom(Binom {
                    start: Term::from(3),
                    a: Term::from(3),
                    b: Term::from(-1),
                }),
            ),
            (
                frac![18, 108, 648, 3888],
                Sequence::Binom(Binom {
                    start: Term::from(18),
                    a: Term::from(6),
                    b: Term::from(0),
                }),
            ),
            (
                frac![68, 36, 20, 12, 8],
                Sequence::Binom(Binom {
                    start: Term::from(68),
                    a: Term::new(1, 2),
                    b: Term::from(2),
                }),
            ),
            (
                frac![8, 20, 50, 125],
                Sequence::Binom(Binom {
                    start: Term::from(8),
                    a: Term::new(5, 2),
                    b: Term::from(0),
                }),
            ),
            (
                frac![4, -8, 16, -32],
                Sequence::Binom(Binom {
                    start: Term::from(4),
                    a: Term::from(-2),
                    b: Term::from(0),
                }),
            ),
            (
                // zipped binomial
                frac![2, 5, 6, 20, 18, 80, 54, 320, 162, 1280, 486],
                Sequence::Zipped(Zipped {
                    seqs: vec![
                        Sequence::Binom(Binom {
                            start: Term::from(2),
                            a: Term::from(3),
                            b: Term::from(0),
                        }),
                        Sequence::Binom(Binom {
                            start: Term::from(5),
                            a: Term::from(4),
                            b: Term::from(0),
                        }),
                    ],
                }),
            ),
            (
                // 1st differences converge
                frac![1, 2, 3],
                Sequence::Differences(Differences { terms: frac![1, 1] }),
            ),
            // differences
            (
                // 2nd differences converge
                frac![-3, 3, 27, 69, 129, 207],
                Sequence::Differences(Differences {
                    terms: frac![-3, 6, 18],
                }),
            ),
            (
                // 3rd differences converge
                frac![9, 73, 241, 561, 1081, 1849, 2913],
                Sequence::Differences(Differences {
                    terms: frac![9, 64, 104, 48],
                }),
            ),
            (
                // 4 zipped linear sequences
                frac![2, 0, 1, 3, 4, 2, 3, 5, 6, 4, 5, 7, 8, 6],
                Sequence::Zipped(Zipped {
                    seqs: vec![
                        Sequence::Differences(Differences { terms: frac![2, 2] }),
                        Sequence::Differences(Differences { terms: frac![0, 2] }),
                        Sequence::Differences(Differences { terms: frac![1, 2] }),
                        Sequence::Differences(Differences { terms: frac![3, 2] }),
                    ],
                }),
            ),
            (
                // 3 zipped linear sequences
                frac![31, 23, 15, 27, 20, 13, 23, 17, 11, 19, 14, 9],
                Sequence::Zipped(Zipped {
                    seqs: vec![
                        Sequence::Differences(Differences {
                            terms: frac![31, -4],
                        }),
                        Sequence::Differences(Differences {
                            terms: frac![23, -3],
                        }),
                        Sequence::Differences(Differences {
                            terms: frac![15, -2],
                        }),
                    ],
                }),
            ),
        ];

        for (input, expected) in test_cases.into_iter() {
            let seq = Sequence::try_from(input.as_slice()).unwrap();
            assert_eq!(seq, expected);
            assert_eq!(seq.into_iter().take(input.len()).collect::<Vec<_>>(), input);
        }
    }

    #[test]
    fn test_one() {
        let test_cases = [(
            frac![31, 23, 15, 27, 20, 13, 23, 17, 11, 19, 14, 9],
            Sequence::Zipped(Zipped {
                seqs: vec![
                    Sequence::Differences(Differences {
                        terms: frac![31, -4],
                    }),
                    Sequence::Differences(Differences {
                        terms: frac![23, -3],
                    }),
                    Sequence::Differences(Differences {
                        terms: frac![15, -2],
                    }),
                ],
            }),
        )];
        for (input, expected) in test_cases.into_iter() {
            let seq = Sequence::try_from(input.as_slice()).unwrap();
            assert_eq!(seq, expected);
            assert_eq!(seq.into_iter().take(input.len()).collect::<Vec<_>>(), input);
        }
    }
}
