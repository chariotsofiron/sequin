mod sequence;
use fraction::GenericFraction;
use sequence::Sequence;
pub type Term = GenericFraction<i64>;

fn main() {
    let nums = [6, 13, 6, 18, 6, 23];
    let seq = nums.iter().map(|&x| Term::from(x)).collect::<Vec<_>>();
    let seq = Sequence::try_from(seq.as_slice()).unwrap();
    println!("{}", seq);
    println!(
        "next number: {}",
        seq.into_iter().skip(nums.len()).next().unwrap()
    );
}
