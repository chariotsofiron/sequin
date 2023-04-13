use std::fmt::Debug;

pub trait Sequence: Debug {
    fn iter(&self) -> Box<dyn Iterator<Item = i32>>;
}

pub trait IdentifiableSequence: for<'a> TryFrom<&'a [i32]> + Sequence {}
