use crate::Fraction;

const N_DIFFERENCE_TERMS: usize = 3;

#[derive(Debug, PartialEq)]
pub struct Differences {
    pub diffs: Vec<Fraction>,
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
        let mut differences: Vec<Fraction> = value.windows(2).map(|w| w[1] - w[0]).collect();
        for _ in 0..N_DIFFERENCE_TERMS {
            nums.push(differences[0]);
            if differences.windows(2).all(|w| w[0] == w[1]) {
                return Ok(Differences { diffs: nums });
            }
            // compute next differences
            differences = differences.windows(2).map(|w| w[1] - w[0]).collect();
        }
        Err(())
    }
}
