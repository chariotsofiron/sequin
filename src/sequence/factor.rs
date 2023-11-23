use fraction::CheckedDiv;

use crate::Term;

use super::{binom::Binomial, Sequence};

#[derive(Debug, PartialEq, Clone)]
pub struct Factor {
    start: Term,
    seq: Box<Sequence>,
}

impl Factor {
    pub fn new<A>(start: A, seq: Sequence) -> Self
    where
        A: Into<Term>,
    {
        Self {
            start: start.into(),
            seq: Box::new(seq),
        }
    }
}

impl std::fmt::Display for Factor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Factor({}, {})", self.start, self.seq)
    }
}

impl TryFrom<&[Term]> for Factor {
    type Error = ();

    fn try_from(value: &[Term]) -> Result<Self, Self::Error> {
        if value.len() < 4 {
            return Err(());
        }
        let diffs: Vec<Term> = value
            .windows(2)
            .map(|w| w[1].checked_div(&w[0]))
            .collect::<Option<Vec<_>>>()
            .ok_or(())?;

        let seq = Binomial::try_from(diffs.as_slice())?;
        Ok(Self::new(value[0], Sequence::Binomial(seq)))
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
        self.start *= self.seq.next()?;
        Some(tmp)
    }
}

impl IntoIterator for Factor {
    type Item = Term;
    type IntoIter = SeqIterator;

    fn into_iter(self) -> Self::IntoIter {
        SeqIterator {
            start: self.start,
            seq: self.seq.into_iter(),
        }
    }
}
