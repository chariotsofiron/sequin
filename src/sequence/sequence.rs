pub trait Sequence<'a>: Sized + Iterator + TryFrom<&'a [i32]> {
    fn iter() -> Iterator<Item=i32>;
}
