use super::sequence::{Seq, Sequence};

pub struct Combination {
    seqs: Vec<dyn Sequence>,
}

// impl Sequence<'_> for Combination {
//     fn iter(&self) -> Box<dyn Iterator<Item = i32>> {
//         Box::new(Multizip(self.seqs.iter().map(Sequence::iter).collect()).iter())intersper
//     }
// }

// impl TryFrom<&[i32]> for Combination {
//     type Error = ();

//     fn try_from(value: &[i32]) -> Result<Self, Self::Error> {
//         Err(())
//     }
// }

// struct Multizip<T>(Vec<T>);

// impl<T> Iterator for Multizip<T>
// where
//     T: Iterator,
// {
//     type Item = Vec<T::Item>;

//     fn next(&mut self) -> Option<Self::Item> {
//         self.0.iter_mut().map(Iterator::next).collect()
//     }
// }
