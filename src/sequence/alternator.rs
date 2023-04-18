//! Can I implement this using binom?
use fraction::Zero;
use itertools::iproduct;

use crate::Term;

/// Sequence produced by alternating an operation
/// a(n) = a * x(n-1) + b if n odd
/// a(n) = c * x(n-1) + d if n even
#[derive(Debug, PartialEq)]
pub struct Alternator {
    pub start: Term,
    pub a: Term,
    pub b: Term,
    pub c: Term,
    pub d: Term,
}

impl std::fmt::Display for Alternator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "a(n) = {} * x(n-1) + {} | {} * x(n-1) + {}",
            self.a, self.b, self.c, self.d
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
        if value.len() < 3 {
            return Err(());
        }

        // 0, 1, 2, ..., 1/2, 2/2, 3/2, ..., 1/3, 2/3, 3/3, ...
        const NUMERATOR_RANGE: std::ops::Range<i32> = 1..10;
        const DENOMINATOR_RANGE: std::ops::Range<i32> = 1..12;
        let fractions = iproduct!(DENOMINATOR_RANGE, NUMERATOR_RANGE, [false, true])
            .into_iter()
            .map(|(b, a, is_signed)| {
                if is_signed {
                    Term::new_neg(a, b)
                } else {
                    Term::new(a, b)
                }
            });
        let mut a = Term::zero();
        let mut b = Term::zero();
        let mut c;
        let mut d;

        let mut ok = true;
        for w in fractions.clone() {
            a = w;
            b = value[1] - a * value[0];
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
            ok = true;
            for i in (1..value.len()).step_by(2) {
                if i + 1 < value.len() && value[i + 1] != c * value[i] + d {
                    ok = false;
                    break;
                }
            }
            if ok {
                return Ok(Alternator {
                    start: value[0],
                    a,
                    b,
                    c,
                    d,
                });
            }
        }
        Err(())
    }
}
