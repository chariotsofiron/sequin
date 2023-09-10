use super::{binom::Binomial, difference::Difference, Sequence};
use crate::Term;

#[derive(Debug, PartialEq, Clone)]
pub struct Zipped {
    pub seqs: Vec<Sequence>,
}

impl std::fmt::Display for Zipped {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut display = String::new();
        display.push('[');
        display.push_str(
            &self
                .seqs
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<String>>()
                .join(", "),
        );
        display.push(']');
        write!(f, "Zipped({display})")
    }
}

impl IntoIterator for Zipped {
    type Item = Term;
    type IntoIter = Multizip;

    fn into_iter(self) -> Self::IntoIter {
        Multizip::new(self.seqs.into_iter().map(IntoIterator::into_iter).collect())
    }
}

impl TryFrom<&[Term]> for Zipped {
    type Error = ();

    fn try_from(value: &[Term]) -> Result<Self, Self::Error> {
        let mut seqs: Vec<Sequence> = Vec::new();
        for i in 2..value.len() {
            for j in 0..i {
                let tmp = value
                    .iter()
                    .skip(j)
                    .step_by(i)
                    .copied()
                    .collect::<Vec<Term>>();
                if let Ok(seq) = Binomial::try_from(tmp.as_slice()) {
                    seqs.push(Sequence::Binomial(seq));
                } else if let Ok(seq) = Difference::try_from(tmp.as_slice()) {
                    seqs.push(Sequence::Difference(seq));
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
    seqs: Vec<Box<dyn Iterator<Item = Term>>>,
    index: usize,
}

impl Multizip {
    pub fn new(seqs: Vec<Box<dyn Iterator<Item = Term>>>) -> Self {
        Self { seqs, index: 0 }
    }
}

impl Iterator for Multizip {
    type Item = Term;

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
