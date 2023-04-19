mod sequence;
use fraction::GenericFraction;
use sequence::Sequence;
pub type Term = GenericFraction<i64>;

fn main() {
    let nums = [268, 145, 617, 189, 347];
    let seq = nums.iter().map(|&x| Term::from(x)).collect::<Vec<_>>();
    let seq = Sequence::try_from(seq.as_slice());
    println!("{:?}", seq);
    println!(
        "next number: {}",
        seq.unwrap().into_iter().skip(nums.len()).next().unwrap()
    );
}
