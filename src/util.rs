use crate::Fraction;

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
