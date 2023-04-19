use crate::Term;

use super::{binom::Binomial, Sequence};

#[derive(Debug, PartialEq, Clone)]
pub struct OnceDiff {
    pub start: Term,
    pub seq: Box<Sequence>,
}

impl std::fmt::Display for OnceDiff {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Diff({}, {})", self.start, self.seq)
    }
}

pub struct OnceDiffIterator {
    start: Term,
    seq: Box<dyn Iterator<Item = Term>>,
}

impl Iterator for OnceDiffIterator {
    type Item = Term;

    fn next(&mut self) -> Option<Self::Item> {
        let tmp = self.start;
        self.start += self.seq.next()?;
        Some(tmp)
    }
}

impl IntoIterator for OnceDiff {
    type Item = Term;
    type IntoIter = OnceDiffIterator;

    fn into_iter(self) -> Self::IntoIter {
        OnceDiffIterator {
            start: self.start,
            seq: self.seq.into_iter(),
        }
    }
}

impl TryFrom<&[Term]> for OnceDiff {
    type Error = ();

    fn try_from(value: &[Term]) -> Result<Self, Self::Error> {
        if value.len() < 4 {
            return Err(());
        }

        let diffs: Vec<Term> = value.windows(2).map(|w| w[1] - w[0]).collect();
        if let Ok(seq) = Binomial::try_from(diffs.as_slice()) {
            Ok(OnceDiff {
                start: value[0],
                seq: Box::new(Sequence::Binomial(seq)),
            })
        } else if let Ok(seq) = Self::try_from(diffs.as_slice()) {
            Ok(OnceDiff {
                start: value[0],
                seq: Box::new(Sequence::OnceDiff(seq)),
            })
        } else {
            Err(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        // let nums = [2, 0, 1, 3, 4, 2, 3, 5, 6, 4, 5, 7, 8, 6];
        // let nums = nums.into_iter().map(|n| Term::from(n)).collect::<Vec<_>>();
        // let diff = OnceDiff::try_from(nums.as_slice()).unwrap();
        // println!("{}", diff);
    }
}
