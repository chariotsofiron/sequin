use fraction::CheckedDiv;

use crate::Term;

use super::{binom::Binomial, Sequence};

#[derive(Debug, PartialEq, Clone)]
pub struct OnceFactor {
    pub start: Term,
    pub seq: Box<Sequence>,
}

impl std::fmt::Display for OnceFactor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Factor({}, {})", self.start, self.seq)
    }
}

impl TryFrom<&[Term]> for OnceFactor {
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

        #[allow(clippy::option_if_let_else)]
        if let Ok(seq) = Binomial::try_from(diffs.as_slice()) {
            Ok(Self {
                start: value[0],
                seq: Box::new(Sequence::Binomial(seq)),
            })
        } else {
            Err(())
        }
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

impl IntoIterator for OnceFactor {
    type Item = Term;
    type IntoIter = SeqIterator;

    fn into_iter(self) -> Self::IntoIter {
        SeqIterator {
            start: self.start,
            seq: self.seq.into_iter(),
        }
    }
}
