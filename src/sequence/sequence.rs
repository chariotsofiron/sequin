pub trait Sequence<'a>: Sized + TryFrom<&'a [i32]> {
    fn iter(&'a self) -> Box<dyn Iterator<Item = i32> + 'a>;
}
