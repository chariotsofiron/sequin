pub mod alternator;
pub mod binom;
pub mod fibonacci;
pub mod oeis;
pub mod once_diff;
pub mod zipped;

use crate::sequence::{binom::Binomial, fibonacci::Fibonacci, zipped::Zipped};
use crate::Term;

use self::alternator::Alternator;
use self::oeis::Oeis;
use self::once_diff::OnceDiff;

#[derive(Debug, PartialEq, Clone)]
pub enum Sequence {
    /// Binomial recursive
    Binomial(Binomial),
    /// Alternator
    Alternator(Alternator),
    /// Fibonacci
    Fibonacci(Fibonacci),
    // Meta strategies
    Zipped(Zipped),
    /// Once diff
    OnceDiff(OnceDiff),
    Oeis(Oeis),
}

impl std::fmt::Display for Sequence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Binomial(seq) => write!(f, "{}", seq),
            Self::Alternator(seq) => write!(f, "{}", seq),
            Self::Fibonacci(seq) => write!(f, "{}", seq),
            Self::Zipped(seq) => write!(f, "{}", seq),
            Self::OnceDiff(seq) => write!(f, "{}", seq),
            Self::Oeis(seq) => write!(f, "{}", seq),
        }
    }
}

impl IntoIterator for Sequence {
    type Item = Term;
    type IntoIter = Box<dyn Iterator<Item = Term>>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            Self::Binomial(seq) => Box::new(seq.into_iter()),
            Self::Alternator(seq) => Box::new(seq.into_iter()),
            Self::Fibonacci(seq) => Box::new(seq.into_iter()),
            Self::Zipped(seq) => Box::new(seq.into_iter()),
            Self::OnceDiff(seq) => Box::new(seq.into_iter()),
            Self::Oeis(seq) => Box::new(seq.into_iter()),
        }
    }
}

impl TryFrom<&[Term]> for Sequence {
    type Error = ();

    fn try_from(value: &[Term]) -> Result<Self, Self::Error> {
        if let Ok(seq) = Binomial::try_from(value) {
            return Ok(Self::Binomial(seq));
        } else if let Ok(seq) = Alternator::try_from(value) {
            return Ok(Self::Alternator(seq));
        } else if let Ok(seq) = Fibonacci::try_from(value) {
            return Ok(Self::Fibonacci(seq));
        } else if let Ok(seq) = Zipped::try_from(value) {
            return Ok(Self::Zipped(seq));
        } else if let Ok(seq) = OnceDiff::try_from(value) {
            return Ok(Self::OnceDiff(seq));
        } else if let Ok(seq) = Oeis::try_from(value) {
            return Ok(Self::Oeis(seq));
        }
        Err(())
    }
}

#[cfg(test)]
mod tests {
    use fraction::Zero;

    use super::Sequence;
    use super::Term;
    use super::*;
    use crate::sequence::zipped::Zipped;

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
                Sequence::Binomial(Binomial {
                    start: Term::from(3),
                    a: Term::from(3),
                    b: Term::from(-1),
                }),
            ),
            (
                frac![18, 108, 648, 3888],
                Sequence::Binomial(Binomial {
                    start: Term::from(18),
                    a: Term::from(6),
                    b: Term::from(0),
                }),
            ),
            (
                frac![68, 36, 20, 12, 8],
                Sequence::Binomial(Binomial {
                    start: Term::from(68),
                    a: Term::new(1, 2),
                    b: Term::from(2),
                }),
            ),
            (
                frac![8, 20, 50, 125],
                Sequence::Binomial(Binomial {
                    start: Term::from(8),
                    a: Term::new(5, 2),
                    b: Term::from(0),
                }),
            ),
            (
                frac![4, -8, 16, -32],
                Sequence::Binomial(Binomial {
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
                        Sequence::Binomial(Binomial {
                            start: Term::from(2),
                            a: Term::from(3),
                            b: Term::from(0),
                        }),
                        Sequence::Binomial(Binomial {
                            start: Term::from(5),
                            a: Term::from(4),
                            b: Term::from(0),
                        }),
                    ],
                }),
            ),
            // differences
            (
                // 2nd differences converge
                frac![-3, 3, 27, 69, 129, 207],
                Sequence::OnceDiff(OnceDiff {
                    start: Term::from(-3),
                    seq: Box::new(Sequence::Binomial(Binomial {
                        start: Term::from(6),
                        a: Term::from(1),
                        b: Term::from(18),
                    })),
                }),
            ),
            (
                // 3rd differences converge
                frac![9, 73, 241, 561, 1081, 1849, 2913],
                Sequence::OnceDiff(OnceDiff {
                    start: Term::from(9),
                    seq: Box::new(Sequence::OnceDiff(OnceDiff {
                        start: Term::from(64),
                        seq: Box::new(Sequence::Binomial(Binomial {
                            start: Term::from(104),
                            a: Term::from(1),
                            b: Term::from(48),
                        })),
                    })),
                }),
            ),
            (
                // 2 zipped diff binomials
                frac![2, 0, 1, 3, 4, 2, 3, 5, 6, 4, 5, 7, 8, 6],
                Sequence::Zipped(Zipped {
                    seqs: vec![
                        Sequence::OnceDiff(OnceDiff {
                            start: Term::from(2),
                            seq: Box::new(Sequence::Binomial(Binomial {
                                start: Term::from(-1),
                                a: Term::from(-1),
                                b: Term::from(2),
                            })),
                        }),
                        Sequence::OnceDiff(OnceDiff {
                            start: Term::from(0),
                            seq: Box::new(Sequence::Binomial(Binomial {
                                start: Term::from(3),
                                a: Term::from(-1),
                                b: Term::from(2),
                            })),
                        }),
                    ],
                }),
            ),
            (
                // 3 zipped binomials
                frac![31, 23, 15, 27, 20, 13, 23, 17, 11, 19, 14, 9],
                Sequence::Zipped(Zipped {
                    seqs: vec![
                        Sequence::Binomial(Binomial {
                            start: Term::from(31),
                            a: Term::from(1),
                            b: Term::from(-4),
                        }),
                        Sequence::Binomial(Binomial {
                            start: Term::from(23),
                            a: Term::from(1),
                            b: Term::from(-3),
                        }),
                        Sequence::Binomial(Binomial {
                            start: Term::from(15),
                            a: Term::from(1),
                            b: Term::from(-2),
                        }),
                    ],
                }),
            ),
            // fibonacci
            (
                // a(n) = a(n-2) - a(n-1)
                frac![1, 0, 1, -1, 2, -3],
                Sequence::Fibonacci(Fibonacci {
                    s0: Term::from(1),
                    s1: Term::from(0),
                    a: Term::from(1),
                    b: Term::from(-1),
                    c: Term::zero(),
                }),
            ),
            (
                // a(n) = a(n-2) + a(n-1)
                frac![34, -21, 13, -8, 5, -3],
                Sequence::Fibonacci(Fibonacci {
                    s0: Term::from(34),
                    s1: Term::from(-21),
                    a: Term::from(1),
                    b: Term::from(1),
                    c: Term::zero(),
                }),
            ),
            (
                frac![13, -21, 34, -55, 89, -144],
                Sequence::Fibonacci(Fibonacci {
                    s0: Term::from(13),
                    s1: Term::from(-21),
                    a: Term::from(1),
                    b: Term::from(-1),
                    c: Term::zero(),
                }),
            ),
            // Alternator
            (
                frac![18, 6, 24, 8, 32],
                Sequence::Alternator(Alternator {
                    start: Term::from(18),
                    a: Term::new(1, 3),
                    b: Term::from(0),
                    c: Term::from(4),
                    d: Term::from(0),
                }),
            ),
            (
                frac![3, 12, 24, 33, 66],
                Sequence::Alternator(Alternator {
                    start: Term::from(3),
                    a: Term::from(1),
                    b: Term::from(9),
                    c: Term::from(2),
                    d: Term::from(0),
                }),
            ),
            (
                vec![
                    Term::from(230),
                    Term::from(460),
                    Term::from(46),
                    Term::from(92),
                    Term::new(92, 10),
                ],
                Sequence::Alternator(Alternator {
                    start: Term::from(230),
                    a: Term::from(2),
                    b: Term::from(0),
                    c: Term::new(1, 10),
                    d: Term::from(0),
                }),
            ),
            // oeis
            (
                frac![0, 1, 4, 7, 8, 9, 6],
                Sequence::Oeis(Oeis {
                    numbers: frac![0, 1, 4, 7, 8, 9, 6, 3, 2, 5],
                }),
            ),
        ];

        for (input, expected) in test_cases.into_iter() {
            let seq = Sequence::try_from(input.as_slice()).unwrap();
            assert_eq!(seq, expected);
            assert_eq!(seq.into_iter().take(input.len()).collect::<Vec<_>>(), input);
        }
    }

    // #[test]
    // fn test_one() {
    //     let test_cases = [(
    //         frac![31, 23, 15, 27, 20, 13, 23, 17, 11, 19, 14, 9],
    //         Sequence::Zipped(Zipped {
    //             seqs: vec![
    //                 Sequence::Differences(Differences {
    //                     terms: frac![31, -4],
    //                 }),
    //                 Sequence::Differences(Differences {
    //                     terms: frac![23, -3],
    //                 }),
    //                 Sequence::Differences(Differences {
    //                     terms: frac![15, -2],
    //                 }),
    //             ],
    //         }),
    //     )];
    //     for (input, expected) in test_cases.into_iter() {
    //         let seq = Sequence::try_from(input.as_slice()).unwrap();
    //         assert_eq!(seq, expected);
    //         assert_eq!(seq.into_iter().take(input.len()).collect::<Vec<_>>(), input);
    //     }
    // }
}
