//! Can I implement this using binom?
use fraction::Zero;
use itertools::iproduct;

use crate::{Size, Term};

/// Sequence produced by alternating an operation
/// a(n) = a * x(n-1) + b if n odd
/// a(n) = c * x(n-1) + d if n even
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Alternator {
    start: Term,
    a: Term,
    b: Term,
    c: Term,
    d: Term,
}

impl Alternator {
    pub fn new<A, B, C, D, E>(start: A, a: B, b: C, c: D, d: E) -> Self
    where
        A: Into<Term>,
        B: Into<Term>,
        C: Into<Term>,
        D: Into<Term>,
        E: Into<Term>,
    {
        Self {
            start: start.into(),
            a: a.into(),
            b: b.into(),
            c: c.into(),
            d: d.into(),
        }
    }
}

impl std::fmt::Display for Alternator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Alternator({}, {}, {}, {}, {})",
            self.start, self.a, self.b, self.c, self.d
        )
    }
}

impl Iterator for Alternator {
    type Item = Term;

    fn next(&mut self) -> Option<Self::Item> {
        let tmp = self.start;
        self.start = self.a * tmp + self.b;
        std::mem::swap(&mut self.a, &mut self.c);
        std::mem::swap(&mut self.b, &mut self.d);
        Some(tmp)
    }
}

impl TryFrom<&[Term]> for Alternator {
    type Error = ();

    fn try_from(value: &[Term]) -> Result<Self, Self::Error> {
        // 0, 1, 2, ..., 1/2, 2/2, 3/2, ..., 1/3, 2/3, 3/3, ...
        const NUMERATOR_RANGE: std::ops::Range<Size> = 1..10;
        const DENOMINATOR_RANGE: std::ops::Range<Size> = 1..12;

        if value.len() < 4 {
            return Err(());
        }
        let fractions = iproduct!(DENOMINATOR_RANGE, NUMERATOR_RANGE, [false, true]).map(
            |(b, a, is_signed)| {
                if is_signed {
                    Term::new(-a, b)
                } else {
                    Term::new(a, b)
                }
            },
        );
        let mut a = Term::zero();
        let mut b = Term::zero();
        let mut c;
        let mut d;

        let mut ok = true;
        for w in fractions.clone() {
            a = w;
            b = value[1] - a * value[0];
            if b.fract() != Term::zero() {
                continue; // b should be an integer
            }
            ok = true;
            for i in (0..value.len()).step_by(2) {
                if i + 1 < value.len() && value[i + 1] != a * value[i] + b {
                    ok = false;
                    break;
                }
            }
            if ok {
                break;
            }
        }

        if !ok {
            return Err(());
        }

        for y in fractions.clone() {
            c = y;
            d = value[2] - c * value[1];
            if d.fract() != Term::zero() {
                continue; // b should be an integer
            }
            ok = true;
            for i in (1..value.len()).step_by(2) {
                if i + 1 < value.len() && value[i + 1] != c * value[i] + d {
                    ok = false;
                    break;
                }
            }
            if ok {
                return Ok(Self::new(value[0], a, b, c, d));
            }
        }
        Err(())
    }
}
