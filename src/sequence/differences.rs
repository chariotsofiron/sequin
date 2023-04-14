use crate::Fraction;

#[derive(Debug, PartialEq)]
pub struct Differences {
    diffs: Vec<Fraction>,
}

impl Iterator for Differences {
    type Item = Fraction;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.diffs[0];

        for i in 1..self.diffs.len() {
            self.diffs[i - 1] = self.diffs[i - 1] + self.diffs[i];
        }
        Some(next)
    }
}

impl TryFrom<&[Fraction]> for Differences {
    type Error = ();

    fn try_from(value: &[Fraction]) -> Result<Self, Self::Error> {
        if value.len() < 3 {
            return Err(());
        }
        // this vec of nums defines the sequence
        let mut nums = Vec::<Fraction>::new();
        nums.push(value[0]);
        let mut diffs: Vec<Fraction> = value.windows(2).map(|w| w[1] - w[0]).collect();
        for _ in 0..5 {
            nums.push(diffs[0]);
            if diffs.windows(2).all(|w| w[0] == w[1]) {
                return Ok(Differences { diffs: nums });
            }
            // compute next differences
            diffs = diffs.windows(2).map(|w| w[1] - w[0]).collect();
        }
        Err(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_differences() {
        let nums = [-3, 3, 27, 69, 129, 207];
        let nums = nums.iter().map(|&x| Fraction::from(x)).collect::<Vec<_>>();
        let diffs = Differences::try_from(nums.as_slice());
        assert_eq!(
            diffs,
            Ok(Differences {
                diffs: vec![Fraction::from(-3), Fraction::from(6), Fraction::from(18)],
            })
        );

        let nums = [9, 73, 241, 561, 1081, 1849, 2913];
        let nums = nums.iter().map(|&x| Fraction::from(x)).collect::<Vec<_>>();
        let diffs = Differences::try_from(nums.as_slice()).unwrap();
        assert_eq!(
            diffs,
            Differences {
                diffs: vec![
                    Fraction::from(9),
                    Fraction::from(64),
                    Fraction::from(104),
                    Fraction::from(48)
                ],
            }
        );

        let mut iter = diffs.into_iter();
        assert_eq!(iter.next(), Some(Fraction::from(9)));
        assert_eq!(iter.next(), Some(Fraction::from(73)));
        assert_eq!(iter.next(), Some(Fraction::from(241)));
        assert_eq!(iter.next(), Some(Fraction::from(561)));
    }
}
