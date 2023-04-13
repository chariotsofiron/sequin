use super::{
    linear::Linear,
    sequence::{IdentifiableSequence, Sequence},
};

#[derive(Debug)]
pub struct Zipped {
    seqs: Vec<Box<dyn Sequence>>,
}

impl Sequence for Zipped {
    /// Zips the vector of iterable sequences into a single sequence.
    fn iter(&self) -> Box<dyn Iterator<Item = i32>> {
        Box::new(Multizip {
            seqs: self.seqs.iter().map(|x| x.iter()).collect(),
            index: 0,
        })
    }
}

impl TryFrom<&[i32]> for Zipped {
    type Error = ();

    fn try_from(value: &[i32]) -> Result<Self, Self::Error> {
        let mut seqs: Vec<Box<dyn Sequence>> = Vec::new();
        for i in 1..value.len() {
            for j in 0..i {
                let tmp = value
                    .iter()
                    .skip(j)
                    .step_by(i)
                    .copied()
                    .collect::<Vec<i32>>();
                if let Ok(seq) = Linear::try_from(tmp.as_slice()) {
                    seqs.push(Box::new(seq));
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

impl IdentifiableSequence for Zipped {}

/// An iterator that zips together multiple iterators.
struct Multizip {
    seqs: Vec<Box<dyn Iterator<Item = i32>>>,
    index: usize,
}

impl Iterator for Multizip {
    type Item = i32;

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
    fn test_multizip() {
        let mut iter = Multizip {
            seqs: vec![
                Box::new((0..).map(|x| x * 2)),
                Box::new((0..).map(|x| x * 3)),
            ],
            index: 0,
        };
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(4));
        assert_eq!(iter.next(), Some(6));
    }

    #[test]
    fn try_from() {
        let nums = [0, 1, 2, 1, 4, 1, 6, 1];

        let zipped = Zipped::try_from(nums.as_slice());
        assert!(zipped.is_ok());

        let seq = [31, 23, 15, 27, 20, 13, 23, 17, 11, 19, 14, 9];
        let zipped = Zipped::try_from(seq.as_slice());
        assert!(zipped.is_ok());

        // take 12 numbers from the sequence
        let nums = zipped.unwrap().iter().take(13).collect::<Vec<i32>>();
        println!("{:?}", nums);
    }
}
