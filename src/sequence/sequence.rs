use super::{geometric::Geometric, linear::Linear};

pub trait Sequence<'a>: Sized + TryFrom<&'a [i32], Error = ()> {
    fn iter(&'a self) -> Box<dyn Iterator<Item = i32> + 'a>;
}

pub enum Seq {
    Linear(Linear),
    Geometric(Geometric),
}

impl Sequence<'_> for Seq {
    fn iter(&self) -> Box<dyn Iterator<Item = i32>> {
        match self {
            Seq::Linear(seq) => seq.iter(),
            Seq::Geometric(seq) => seq.iter(),
        }
    }
}

impl TryFrom<&[i32]> for Seq {
    type Error = ();

    fn try_from(value: &[i32]) -> Result<Self, Self::Error> {
        Err(())
    }
}
