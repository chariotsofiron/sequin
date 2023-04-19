use crate::Term;
use fraction::Zero;
use itertools::iproduct;

/// Sequences where the next term is ax+b where x
/// is the previous term and a and b are constants.
/// Covers the linear sequence case.
#[derive(Debug, PartialEq, Clone)]
pub struct Binomial {
    pub start: Term,
    pub a: Term,
    pub b: Term,
}

impl std::fmt::Display for Binomial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Binom({}, {}, {})", self.start, self.a, self.b)
    }
}

impl Binomial {
    pub fn closed_form(&self) -> String {
        // https://math.stackexchange.com/a/2194232
        format!("a(n) = {}*{}^n + {}*{}^n", self.a, self.a, self.b, self.b)
    }
}

impl TryFrom<&[Term]> for Binomial {
    type Error = ();

    fn try_from(value: &[Term]) -> Result<Self, Self::Error> {
        if value.len() < 4 {
            // too many false positives if only 3 terms
            return Err(());
        }

        const NUMERATOR_RANGE: std::ops::Range<i32> = 0..11;
        const DENOMINATOR_RANGE: std::ops::Range<i32> = 1..11;
        let fractions = iproduct!(DENOMINATOR_RANGE, NUMERATOR_RANGE, [false, true])
            .into_iter()
            .map(|(b, a, is_signed)| {
                if is_signed {
                    Term::new_neg(a, b)
                } else {
                    Term::new(a, b)
                }
            });

        for a in fractions {
            let b = value[1] - a * value[0];
            if b.fract() != Term::zero() {
                continue; // b should be an integer
            }
            let mut ok = true;
            for w in value.windows(2) {
                if a * w[0] + b != w[1] {
                    ok = false;
                    break;
                }
            }
            if ok {
                return Ok(Binomial {
                    start: Term::from(value[0]),
                    a,
                    b,
                });
            }
        }
        Err(())
    }
}

impl Iterator for Binomial {
    type Item = Term;

    fn next(&mut self) -> Option<Self::Item> {
        let ans = self.start;
        self.start = self.a * self.start + self.b;
        Some(ans)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let nums = [104, 152, 200, 248, 296];
        let nums = nums.into_iter().map(|n| Term::from(n)).collect::<Vec<_>>();
        let binom = Binomial::try_from(nums.as_slice()).unwrap();
        println!("{:?}", binom);
    }
}
