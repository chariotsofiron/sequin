use crate::{util::Multizip, Fraction};

use super::sequence::SimpleSequence;

#[derive(Debug)]
pub struct Zipped {
    seqs: Vec<SimpleSequence>,
}

impl IntoIterator for Zipped {
    type Item = Fraction;
    type IntoIter = Multizip;

    fn into_iter(self) -> Self::IntoIter {
        Multizip::new(self.seqs.into_iter().map(|x| x.into_iter()).collect())
    }
}

impl TryFrom<&[Fraction]> for Zipped {
    type Error = ();

    fn try_from(value: &[Fraction]) -> Result<Self, Self::Error> {
        let mut seqs: Vec<SimpleSequence> = Vec::new();
        for i in 1..value.len() {
            for j in 0..i {
                let tmp = value
                    .iter()
                    .skip(j)
                    .step_by(i)
                    .copied()
                    .collect::<Vec<Fraction>>();
                if let Ok(seq) = SimpleSequence::try_from(tmp.as_slice()) {
                    seqs.push(seq);
                } else {
                    seqs.clear();
                    break;
                }
            }
            if !seqs.is_empty() {
                break;
            }
        }
        if seqs.is_empty() {
            return Err(());
        }

        Ok(Self { seqs })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zipped() {

        let nums = [2, 5, 6, 20, 18, 80, 54, 320, 162, 1280, 486];
        let nums = nums.iter().map(|x| Fraction::from(*x)).collect::<Vec<_>>();
        let zipped = Zipped::try_from(nums.as_slice()).unwrap();
        let ans = zipped.into_iter().take(nums.len()).collect::<Vec<_>>();
        assert_eq!(ans, nums);

    }

    #[test]
    fn test2() {
        let nums = vec![10, 45, 15, 38, 20, 31];
        let nums = nums
            .iter()
            .map(|x| Fraction::from(*x))
            .collect::<Vec<Fraction>>();
        let binom = Zipped::try_from(nums.as_slice()).unwrap();
        let result: Vec<_> = binom.into_iter().take(nums.len()).collect();
        assert_eq!(result, nums);
    }
}
