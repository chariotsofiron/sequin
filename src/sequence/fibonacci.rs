use crate::Term;

const NUMERATOR_RANGE: std::ops::RangeInclusive<i32> = -10..=10;
const DENOMINATOR_RANGE: std::ops::RangeInclusive<i32> = 1..=10;

/// Sequences where the next term is ax+b where x
/// is the previous term and a and b are constants.
/// Covers the linear sequence case.
#[derive(Debug, PartialEq)]
pub struct Fibonacci {
    pub start: Vec<Term>,
    pub a: Term,
    pub b: Term,
}

impl TryFrom<&[Term]> for Fibonacci {
    type Error = ();

    fn try_from(value: &[Term]) -> Result<Self, Self::Error> {
        if value.len() < 3 {
            return Err(());
        }

        for num in NUMERATOR_RANGE {
            for denom in DENOMINATOR_RANGE {
                let a = Term::from(num) / Term::from(denom);
                for offset in NUMERATOR_RANGE {
                    let b = Term::from(offset);
                    let mut ok = true;
                    for w in value.windows(2) {
                        if a * w[0] + b != w[1] {
                            ok = false;
                            break;
                        }
                    }
                }
            }
        }
        Err(())
    }
}
