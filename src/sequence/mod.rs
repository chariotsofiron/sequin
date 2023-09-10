pub mod alternator;
pub mod binom;
pub mod difference;
pub mod factor;
pub mod fibonacci;
pub mod oeis;
pub mod zipped;

use crate::sequence::{binom::Binomial, fibonacci::Fibonacci, zipped::Zipped};
use crate::Term;

use self::alternator::Alternator;
use self::difference::Difference;
use self::factor::Factor;
use self::oeis::Oeis;

#[derive(Debug, PartialEq, Clone)]
pub enum Sequence {
    Binomial(Binomial),
    Difference(Difference),
    Factor(Factor),
    Alternator(Alternator),
    Fibonacci(Fibonacci),
    Zipped(Zipped),
    Oeis(Oeis),
}

impl std::fmt::Display for Sequence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Binomial(seq) => write!(f, "{seq}"),
            Self::Factor(seq) => write!(f, "{seq}"),
            Self::Alternator(seq) => write!(f, "{seq}"),
            Self::Fibonacci(seq) => write!(f, "{seq}"),
            Self::Difference(seq) => write!(f, "{seq}"),
            Self::Zipped(seq) => write!(f, "{seq}"),
            Self::Oeis(seq) => write!(f, "{seq}"),
        }
    }
}

impl IntoIterator for Sequence {
    type Item = Term;
    type IntoIter = Box<dyn Iterator<Item = Term>>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            Self::Binomial(seq) => Box::new(seq.into_iter()),
            Self::Factor(seq) => Box::new(seq.into_iter()),
            Self::Alternator(seq) => Box::new(seq.into_iter()),
            Self::Fibonacci(seq) => Box::new(seq.into_iter()),
            Self::Difference(seq) => Box::new(seq.into_iter()),
            Self::Zipped(seq) => Box::new(seq.into_iter()),
            Self::Oeis(seq) => Box::new(seq.into_iter()),
        }
    }
}

impl TryFrom<&[Term]> for Sequence {
    type Error = ();

    fn try_from(value: &[Term]) -> Result<Self, Self::Error> {
        if let Ok(seq) = Binomial::try_from(value) {
            return Ok(Self::Binomial(seq));
        } else if let Ok(seq) = Factor::try_from(value) {
            return Ok(Self::Factor(seq));
        } else if let Ok(seq) = Alternator::try_from(value) {
            return Ok(Self::Alternator(seq));
        } else if let Ok(seq) = Fibonacci::try_from(value) {
            return Ok(Self::Fibonacci(seq));
        } else if let Ok(seq) = Difference::try_from(value) {
            return Ok(Self::Difference(seq));
        } else if let Ok(seq) = Zipped::try_from(value) {
            return Ok(Self::Zipped(seq));
        } else if let Ok(seq) = Oeis::try_from(value) {
            return Ok(Self::Oeis(seq));
        }
        Err(())
    }
}

#[cfg(test)]
mod tests {
    use super::alternator::Alternator;
    use super::binom::Binomial;
    use super::difference::Difference;
    use super::fibonacci::Fibonacci;
    use super::oeis::Oeis;
    use super::Sequence;
    use super::Term;
    use crate::sequence::zipped::Zipped;

    macro_rules! frac {
        ( $( $x:expr ),* ) => {
            vec![
                $( Term::from($x), )*
            ]
        };
    }

    #[test]
    fn test_binomial() {
        let test_cases = [
            (
                frac![1, 1, 1],
                Binomial {
                    start: Term::from(1),
                    a: Term::from(0),
                    b: Term::from(1),
                },
            ),
            (
                frac![1, 2, 3],
                Binomial {
                    start: Term::from(1),
                    a: Term::from(1),
                    b: Term::from(1),
                },
            ),
            (
                frac![3, 8, 23, 68, 203],
                Binomial {
                    start: Term::from(3),
                    a: Term::from(3),
                    b: Term::from(-1),
                },
            ),
            (
                frac![18, 108, 648, 3888],
                Binomial {
                    start: Term::from(18),
                    a: Term::from(6),
                    b: Term::from(0),
                },
            ),
            (
                frac![68, 36, 20, 12, 8],
                Binomial {
                    start: Term::from(68),
                    a: Term::new(1, 2),
                    b: Term::from(2),
                },
            ),
            (
                frac![8, 20, 50, 125],
                Binomial {
                    start: Term::from(8),
                    a: Term::new(5, 2),
                    b: Term::from(0),
                },
            ),
            (
                frac![4, -8, 16, -32],
                Binomial {
                    start: Term::from(4),
                    a: Term::from(-2),
                    b: Term::from(0),
                },
            ),
        ];

        for (input, expected) in test_cases {
            let seq = Sequence::try_from(input.as_slice()).unwrap();
            assert_eq!(seq, Sequence::Binomial(expected));
            assert_eq!(seq.into_iter().take(input.len()).collect::<Vec<_>>(), input);
        }
    }

    #[test]
    fn test_alternator() {
        let test_cases = [
            (
                frac![18, 6, 24, 8, 32],
                Alternator {
                    start: Term::from(18),
                    a: Term::new(1, 3),
                    b: Term::from(0),
                    c: Term::from(4),
                    d: Term::from(0),
                },
            ),
            (
                frac![3, 12, 24, 33, 66],
                Alternator {
                    start: Term::from(3),
                    a: Term::from(1),
                    b: Term::from(9),
                    c: Term::from(2),
                    d: Term::from(0),
                },
            ),
            (
                vec![
                    Term::from(230),
                    Term::from(460),
                    Term::from(46),
                    Term::from(92),
                    Term::new(92, 10),
                ],
                Alternator {
                    start: Term::from(230),
                    a: Term::from(2),
                    b: Term::from(0),
                    c: Term::new(1, 10),
                    d: Term::from(0),
                },
            ),
            (
                frac![2, 5, 1, 6, 0],
                Alternator {
                    start: Term::from(2),
                    a: Term::from(-1),
                    b: Term::from(7),
                    c: Term::from(-1),
                    d: Term::from(6),
                },
            ),
            (
                frac![3, 12, 24, 33, 66],
                Alternator {
                    start: Term::from(3),
                    a: Term::from(1),
                    b: Term::from(9),
                    c: Term::from(2),
                    d: Term::from(0),
                },
            ),
            (
                vec![
                    Term::from(230),
                    Term::from(460),
                    Term::from(46),
                    Term::from(92),
                    Term::new(92, 10),
                ],
                Alternator {
                    start: Term::from(230),
                    a: Term::from(2),
                    b: Term::from(0),
                    c: Term::new(1, 10),
                    d: Term::from(0),
                },
            ),
        ];

        for (input, expected) in test_cases {
            let seq = Sequence::try_from(input.as_slice()).unwrap();
            assert_eq!(seq, Sequence::Alternator(expected));
            assert_eq!(seq.into_iter().take(input.len()).collect::<Vec<_>>(), input);
        }
    }

    #[test]
    fn test_differences() {
        let test_cases = [
            (
                // 2nd differences converge
                frac![-3, 3, 27, 69, 129],
                Difference {
                    start: Term::from(-3),
                    seq: Box::new(Sequence::Binomial(Binomial {
                        start: Term::from(6),
                        a: Term::from(1),
                        b: Term::from(18),
                    })),
                },
            ),
            (
                // 3rd differences converge
                frac![9, 73, 241, 561, 1081, 1849, 2913],
                Difference {
                    start: Term::from(9),
                    seq: Box::new(Sequence::Difference(Difference {
                        start: Term::from(64),
                        seq: Box::new(Sequence::Binomial(Binomial {
                            start: Term::from(104),
                            a: Term::from(1),
                            b: Term::from(48),
                        })),
                    })),
                },
            ),
        ];

        for (input, expected) in test_cases {
            let seq = Sequence::try_from(input.as_slice()).unwrap();
            assert_eq!(seq, Sequence::Difference(expected));
            assert_eq!(seq.into_iter().take(input.len()).collect::<Vec<_>>(), input);
        }
    }

    #[test]
    fn test_fibonacci() {
        let test_cases = [
            (
                // a(n) = a(n-2) - a(n-1)
                frac![1, 0, 1, -1, 2, -3],
                Fibonacci {
                    s0: Term::from(1),
                    s1: Term::from(0),
                    a: Term::from(1),
                    b: Term::from(-1),
                },
            ),
            (
                // a(n) = a(n-2) + a(n-1)
                frac![34, -21, 13, -8, 5, -3],
                Fibonacci {
                    s0: Term::from(34),
                    s1: Term::from(-21),
                    a: Term::from(1),
                    b: Term::from(1),
                },
            ),
            (
                frac![13, -21, 34, -55, 89, -144],
                Fibonacci {
                    s0: Term::from(13),
                    s1: Term::from(-21),
                    a: Term::from(1),
                    b: Term::from(-1),
                },
            ),
            (
                frac![1, 3, 7, 17, 41, 99, 239],
                Fibonacci {
                    s0: Term::from(1),
                    s1: Term::from(3),
                    a: Term::from(1),
                    b: Term::from(2),
                },
            ),
        ];

        for (input, expected) in test_cases {
            let seq = Sequence::try_from(input.as_slice()).unwrap();
            assert_eq!(seq, Sequence::Fibonacci(expected));
            assert_eq!(seq.into_iter().take(input.len()).collect::<Vec<_>>(), input);
        }
    }

    #[test]
    fn test_seqs() {
        let test_cases = [
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
            // (
            //     // 2 zipped diff binomials
            //     frac![2, 0, 1, 3, 4, 2, 3, 5, 6, 4, 5, 7, 8, 6],
            //     Sequence::Zipped(Zipped {
            //         seqs: vec![
            //             Sequence::OnceDiff(OnceDiff {
            //                 start: Term::from(2),
            //                 seq: Box::new(Sequence::Binomial(Binomial {
            //                     start: Term::from(-1),
            //                     a: Term::from(-1),
            //                     b: Term::from(2),
            //                 })),
            //             }),
            //             Sequence::OnceDiff(OnceDiff {
            //                 start: Term::from(0),
            //                 seq: Box::new(Sequence::Binomial(Binomial {
            //                     start: Term::from(3),
            //                     a: Term::from(-1),
            //                     b: Term::from(2),
            //                 })),
            //             }),
            //         ],
            //     }),
            // ),
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
            // oeis
            (
                frac![0, 1, 4, 7, 8, 9, 6],
                Sequence::Oeis(Oeis {
                    numbers: frac![0, 1, 4, 7, 8, 9, 6, 3, 2, 5],
                }),
            ),
        ];

        for (input, expected) in test_cases {
            let seq = Sequence::try_from(input.as_slice()).unwrap();
            assert_eq!(seq, expected);
            assert_eq!(seq.into_iter().take(input.len()).collect::<Vec<_>>(), input);
        }
    }
}
