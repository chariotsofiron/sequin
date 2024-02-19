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
    Factor(Factor),
    Alternator(Alternator),
    Fibonacci(Fibonacci),
    Difference(Difference),
    Zipped(Zipped),
    Oeis(Oeis),
}

impl Sequence {
    pub fn binom<A, B, C>(start: A, a: B, b: C) -> Self
    where
        A: Into<Term>,
        B: Into<Term>,
        C: Into<Term>,
    {
        Self::Binomial(Binomial::new(start, a, b))
    }

    pub fn factor<A>(start: A, seq: Self) -> Self
    where
        A: Into<Term>,
    {
        Self::Factor(Factor::new(start, seq))
    }

    pub fn alternator<A, B, C, D, E>(start: A, a: B, b: C, c: D, d: E) -> Self
    where
        A: Into<Term>,
        B: Into<Term>,
        C: Into<Term>,
        D: Into<Term>,
        E: Into<Term>,
    {
        Self::Alternator(Alternator::new(start, a, b, c, d))
    }

    pub fn fibonacci<A, B, C, D>(start: A, b: B, c: C, d: D) -> Self
    where
        A: Into<Term>,
        B: Into<Term>,
        C: Into<Term>,
        D: Into<Term>,
    {
        Self::Fibonacci(Fibonacci::new(start, b, c, d))
    }

    pub fn diff<A>(start: A, seq: Self) -> Self
    where
        A: Into<Term>,
    {
        Self::Difference(Difference::new(start, seq))
    }
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
        // the order these are attempted are quite particular.
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
    use super::factor::Factor;
    use super::fibonacci::Fibonacci;
    use super::oeis::Oeis;
    use super::Sequence;
    use super::Term;
    use crate::sequence::zipped::Zipped;

    macro_rules! frac {
        ( $( $x:expr ),* ) => {
            vec![$( ($x).into(), )*]
        };
    }

    #[test]
    fn test_binomial() {
        let test_cases = [
            (frac![0, 0, 0], Binomial::new(0, 0, 0)),
            (frac![1, 1, 1], Binomial::new(1, 0, 1)),
            (frac![1, 2, 3], Binomial::new(1, 1, 1)),
            (frac![3, 8, 23, 68, 203], Binomial::new(3, 3, -1)),
            (frac![18, 108, 648, 3888], Binomial::new(18, 6, 0)),
            (frac![68, 36, 20, 12], Binomial::new(68, Term::new(1, 2), 2)),
            (frac![8, 20, 50, 125], Binomial::new(8, Term::new(5, 2), 0)),
            (frac![4, -8, 16, -32], Binomial::new(4, -2, 0)),
            (frac![3, 8, 23, 68, 203], Binomial::new(3, 3, -1)),
            (frac![18, 108, 648, 3888], Binomial::new(18, 6, 0)),
            (frac![8, 20, 50, 125], Binomial::new(8, Term::new(5, 2), 0)),
            (frac![52, 56, 48, 64, 32], Binomial::new(52, -2, 160)),
            (frac![4, 13, 40, 121, 364], Binomial::new(4, 3, 1)),
            (
                frac![68, 36, 20, 12, 8],
                Binomial::new(68, Term::new(1, 2), 2),
            ),
        ];

        for (input, expected) in test_cases {
            let seq = Sequence::try_from(input.as_slice()).unwrap();
            assert_eq!(seq, Sequence::Binomial(expected));
            assert!(seq.into_iter().take(input.len()).eq(input));
        }
    }

    #[test]
    fn test_factor() {
        let test_cases = [
            (
                frac![3, 15, 60, 180, 360],
                Factor::new(3, Sequence::binom(5, 1, -1)),
            ),
            (
                frac![5, 5, 15, 75, 525, 4725],
                Factor::new(5, Sequence::binom(1, 1, 2)),
            ),
            (
                frac![1, 2, 10, 20, 100],
                Factor::new(1, Sequence::binom(2, -1, 7)),
            ),
            (
                frac![1, 2, 6, 24, 120],
                Factor::new(1, Sequence::binom(2, 1, 1)),
            ),
        ];

        for (input, expected) in test_cases {
            let seq = Sequence::try_from(input.as_slice()).unwrap();
            assert_eq!(seq, Sequence::Factor(expected));
            assert!(seq.into_iter().take(input.len()).eq(input));
        }
    }

    #[test]
    fn test_alternator() {
        let test_cases = [
            (frac![3, 12, 24, 33, 66], Alternator::new(3, 1, 9, 2, 0)),
            (frac![2, 5, 1, 6, 0], Alternator::new(2, -1, 7, -1, 6)),
            (
                frac![18, 6, 24, 8, 32],
                Alternator::new(18, Term::new(1, 3), 0, 4, 0),
            ),
            (
                frac![230, 460, 46, 92, Term::new(92, 10)],
                Alternator::new(230, 2, 0, Term::new(1, 10), 0),
            ),
            (
                frac![230, 460, 46, 92, Term::new(92, 10)],
                Alternator::new(230, 2, 0, Term::new(1, 10), 0),
            ),
        ];

        for (input, expected) in test_cases {
            let seq = Sequence::try_from(input.as_slice()).unwrap();
            assert_eq!(seq, Sequence::Alternator(expected));
            assert!(seq.into_iter().take(input.len()).eq(input));
        }
    }

    #[test]
    fn test_fibonacci() {
        let test_cases = [
            (frac![1, 0, 1, -1, 2, -3], Fibonacci::new(1, 0, 1, -1)),
            (frac![34, -21, 13, -8, 5, -3], Fibonacci::new(34, -21, 1, 1)),
            (
                frac![13, -21, 34, -55, 89, -144],
                Fibonacci::new(13, -21, 1, -1),
            ),
            (frac![1, 3, 7, 17, 41, 99, 239], Fibonacci::new(1, 3, 1, 2)),
            (frac![25, 49, 96, 188, 368], Fibonacci::new(25, 49, -4, 4)),
        ];

        for (input, expected) in test_cases {
            let seq = Sequence::try_from(input.as_slice()).unwrap();
            assert_eq!(seq, Sequence::Fibonacci(expected));
            assert_eq!(seq.into_iter().take(input.len()).collect::<Vec<_>>(), input);
        }
    }

    #[test]
    fn test_differences() {
        // if the binomial term has a negative coefficient, differences won't converge
        let test_cases = [
            (
                frac![-3, 3, 27, 69, 129],
                Sequence::diff(-3, Sequence::binom(6, 1, 18)),
            ),
            (
                frac![9, 73, 241, 561, 1081, 1849, 2913],
                Sequence::diff(9, Sequence::diff(64, Sequence::binom(104, 1, 48))),
            ),
            (
                frac![-2, 1, 6, 13, 22, 33],
                Sequence::diff(-2, Sequence::binom(3, 1, 2)),
            ),
            (
                frac![2, 3, 7, 17, 33, 58],
                Sequence::diff(
                    2,
                    Sequence::diff(1, Sequence::diff(3, Sequence::binom(3, -1, 3))),
                ),
            ),
            (
                frac![1, 7, 20, 44, 81, 135],
                Sequence::diff(
                    1,
                    Sequence::diff(6, Sequence::diff(7, Sequence::binom(4, -1, 6))),
                ),
            ),
        ];

        for (input, expected) in test_cases {
            let seq = Sequence::try_from(input.as_slice()).unwrap();
            assert_eq!(seq, expected);
            assert_eq!(seq.into_iter().take(input.len()).collect::<Vec<_>>(), input);
        }
    }

    #[test]
    fn test_seqs() {
        let test_cases = [
            (
                // zipped binomial
                frac![2, 5, 6, 20, 18, 80, 54, 320, 162, 1280, 486],
                Sequence::Zipped(Zipped::new(vec![
                    Sequence::binom(2, 3, 0),
                    Sequence::binom(5, 4, 0),
                ])),
            ),
            (
                // 3 zipped binomials
                frac![31, 23, 15, 27, 20, 13, 23, 17, 11, 19, 14, 9],
                Sequence::Zipped(Zipped::new(vec![
                    Sequence::binom(31, 1, -4),
                    Sequence::binom(23, 1, -3),
                    Sequence::binom(15, 1, -2),
                ])),
            ),
            // // oeis
            // (
            //     frac![0, 1, 4, 7, 8, 9, 6],
            //     Sequence::Oeis(Oeis {
            //         numbers: frac![0, 1, 4, 7, 8, 9, 6, 3, 2, 5],
            //     }),
            // ),
        ];

        for (input, expected) in test_cases {
            let seq = Sequence::try_from(input.as_slice()).unwrap();
            assert_eq!(seq, expected);
            assert_eq!(seq.into_iter().take(input.len()).collect::<Vec<_>>(), input);
        }
    }
}
