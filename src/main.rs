mod sequence;
use crate::sequence::linear::Linear;

fn main() {
    println!("Hello, world!");

    let sequence = [2i32, 4i32, 6i32];
    let blah = Linear::try_from(sequence.as_slice());

    println!("The next number in the sequence is {blah:?}");
}
