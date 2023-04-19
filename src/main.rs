mod sequence;
use fraction::GenericFraction;
use sequence::Sequence;
pub type Term = GenericFraction<i32>;

fn main() {
    let nums = [0, 4, 18, 48];
    let seq = nums.iter().map(|&x| Term::from(x)).collect::<Vec<_>>();
    let seq = Sequence::try_from(seq.as_slice());
    println!("{:?}", seq);
    println!("next number: {}", seq.unwrap().into_iter().skip(nums.len()).next().unwrap());
}
