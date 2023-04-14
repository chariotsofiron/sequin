use crate::Fraction;

/// Sequences where the next term is ax+b where x
/// is the previous term and a and b are constants.
#[derive(Debug, PartialEq)]
pub struct Binom {
    start: Fraction,
    a: Fraction,
    b: Fraction,
}

pub struct BinomIter {
    current: Fraction,
    a: Fraction,
    b: Fraction,
}

impl Iterator for BinomIter {
    type Item = Fraction;

    fn next(&mut self) -> Option<Self::Item> {
        let ans = self.current;
        self.current = self.a * self.current + self.b;
        Some(ans)
    }
}

impl IntoIterator for Binom {
    type Item = Fraction;
    type IntoIter = BinomIter;

    fn into_iter(self) -> Self::IntoIter {
        BinomIter {
            current: self.start,
            a: self.a,
            b: self.b,
        }
    }
}

impl TryFrom<&[Fraction]> for Binom {
    type Error = ();

    fn try_from(value: &[Fraction]) -> Result<Self, Self::Error> {
        if value.len() < 3 {
            return Err(());
        }

        for num in -10..10 {
            for denom in 1..10 {
                let a = Fraction::from(num) / Fraction::from(denom);
                for offset in -10..10 {
                    let b = Fraction::from(offset);
                    let mut prev = value[0];
                    let mut ok = true;
                    for &next in value.iter().skip(1) {
                        if a * prev + b != next {
                            ok = false;
                            break;
                        }
                        prev = next;
                    }
                    if ok {
                        return Ok(Binom {
                            start: Fraction::from(value[0]),
                            a,
                            b,
                        });
                    }
                }
            }
        }
        Err(())
    }
}

#[cfg(test)]
mod tests {
    use super::Fraction;
    use super::*;

    #[test]
    fn test_prev_binom() {
        let test_cases = [
            (
                vec![3, 8, 23, 68, 203],
                Binom {
                    start: Fraction::from(3),
                    a: Fraction::from(3),
                    b: Fraction::from(-1),
                },
            ),
            (
                vec![18, 108, 648, 3888],
                Binom {
                    start: Fraction::from(18),
                    a: Fraction::from(6),
                    b: Fraction::from(0),
                },
            ),
            (
                vec![68, 36, 20, 12, 8],
                Binom {
                    start: Fraction::from(68),
                    a: Fraction::new(1, 2),
                    b: Fraction::from(2),
                },
            ),
            (
                vec![8, 20, 50, 125],
                Binom {
                    start: Fraction::from(8),
                    a: Fraction::new(5, 2),
                    b: Fraction::from(0),
                },
            ),
            (
                vec![4, -8, 16, -32],
                Binom {
                    start: Fraction::from(4),
                    a: Fraction::from(-2),
                    b: Fraction::from(0),
                },
            ),
        ];

        for (input, expected) in test_cases.iter() {
            let input: Vec<Fraction> = input.iter().map(|x| Fraction::from(*x)).collect();
            let actual = Binom::try_from(input.as_slice()).unwrap();
            assert_eq!(actual, *expected);
        }
    }

    #[test]
    fn test_iter() {
        let nums = vec![3, 8, 23, 68, 203];
        let nums = nums
            .iter()
            .map(|x| Fraction::from(*x))
            .collect::<Vec<Fraction>>();
        let binom = Binom::try_from(nums.as_slice()).unwrap();
        let result: Vec<_> = binom.into_iter().take(nums.len()).collect();
        assert_eq!(result, nums);
    }
}
