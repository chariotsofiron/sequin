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

pub struct SeqIterator {
    start: Term,
    seq: Box<dyn Iterator<Item = Term>>,
}

impl Iterator for SeqIterator {
    type Item = Term;

    fn next(&mut self) -> Option<Self::Item> {
        let tmp = self.start;
        self.start += self.seq.next()?;
        Some(tmp)
    }
}

impl IntoIterator for OnceDiff {
    type Item = Term;
    type IntoIter = SeqIterator;

    fn into_iter(self) -> Self::IntoIter {
        SeqIterator {
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

        #[allow(clippy::option_if_let_else)]
        if let Ok(seq) = Binomial::try_from(diffs.as_slice()) {
            Ok(Self {
                start: value[0],
                seq: Box::new(Sequence::Binomial(seq)),
            })
        } else if let Ok(seq) = Self::try_from(diffs.as_slice()) {
            Ok(Self {
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
        let _nums = [1, 4, 7, 8, 9, 6];
        let nums = [-3, 3, 27, 69, 129, 207];
        let nums = nums.into_iter().map(Term::from).collect::<Vec<_>>();
        let diff = OnceDiff::try_from(nums.as_slice()).unwrap();
        println!("{diff}");
    }
}
