mod sequence;
use fraction::GenericFraction;
use sequence::Sequence;
pub type Term = GenericFraction<i32>;

fn main() {
    let seq = [1, 4, 10, 20, 35];
    let seq = seq.iter().map(|&x| Term::from(x)).collect::<Vec<_>>();
    let seq = Sequence::try_from(seq.as_slice());
    println!("{:?}", seq);
}
