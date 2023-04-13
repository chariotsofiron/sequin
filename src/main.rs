mod sequence;
use crate::sequence::sequence::Sequence;
use crate::sequence::{linear::Linear, zipped::Zipped};

fn main() {
    println!("Hello, world!");

    let nums = [2, 0, 1, 3, 4, 2, 3, 5, 6, 4, 5, 7, 8, 6];
    // let nums = [15, 29, 56, 108, 208];
    let blah = Zipped::try_from(nums.as_slice());

    println!("{:?}", blah);

    let mut iter = blah.unwrap().iter();
    let next = iter.skip(nums.len()).next();

    // let next = blah.iter().skip(nums.len() - 1).next();

    println!("The next number in the sequence is {next:?}");
}
