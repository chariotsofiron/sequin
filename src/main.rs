mod sequence;
use fraction::GenericFraction;
use sequence::Sequence;
pub type Term = GenericFraction<i64>;

fn main() {
    let nums = [0, 2, 24, 108, 320, 750];
    let seq = nums.iter().map(|&x| Term::from(x)).collect::<Vec<_>>();
    let seq = Sequence::try_from(seq.as_slice()).unwrap();
    println!("{}", seq);
    let mut iter = seq.into_iter().skip(nums.len());
    println!(
        "next numbers: {}, {}, {}",
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap()
    );
}
