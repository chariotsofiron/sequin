mod sequence;
use crate::sequence::linear::Linear;

fn main() {
    println!("Hello, world!");

    let sequence = [2, 4, 6];
    let blah = Linear::try_from(sequence.as_slice());

    println!("The next number in the sequence is {:?}", blah);
}
