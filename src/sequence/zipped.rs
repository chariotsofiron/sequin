use crate::Fraction;

use super::sequence::Sequence;

#[derive(Debug, PartialEq)]
pub struct Zipped {
    seqs: Vec<Sequence>,
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
        let mut seqs: Vec<Sequence> = Vec::new();
        for i in 1..value.len() {
            for j in 0..i {
                let tmp = value
                    .iter()
                    .skip(j)
                    .step_by(i)
                    .copied()
                    .collect::<Vec<Fraction>>();
                if let Ok(seq) = Sequence::try_from(tmp.as_slice()) {
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

/// An iterator that zips together multiple iterators.
pub struct Multizip {
    seqs: Vec<Box<dyn Iterator<Item = Fraction>>>,
    index: usize,
}

impl Multizip {
    pub fn new(seqs: Vec<Box<dyn Iterator<Item = Fraction>>>) -> Self {
        Self { seqs, index: 0 }
    }
}

impl Iterator for Multizip {
    type Item = Fraction;

    fn next(&mut self) -> Option<Self::Item> {
        if self.seqs.is_empty() {
            return None;
        }
        loop {
            match self.seqs[self.index].next() {
                Some(x) => {
                    self.index = (self.index + 1) % self.seqs.len();
                    return Some(x);
                }
                None => {
                    let _ = self.seqs.remove(self.index);
                }
            }
        }
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

    #[test]
    fn test3() {
        let nums = [2, 0, 1, 3, 4, 2, 3, 5, 6, 4, 5, 7, 8, 6];
        let nums = nums.iter().map(|x| Fraction::from(*x)).collect::<Vec<_>>();
        let zipped = Zipped::try_from(nums.as_slice()).unwrap();
    }
}
