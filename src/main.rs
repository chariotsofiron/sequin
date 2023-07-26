mod sequence;
use fraction::GenericFraction;
use sequence::Sequence;
use std::str::FromStr;
pub type Term = GenericFraction<i64>;

use clap::Parser;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of terms to generate
    #[arg(short, default_value_t = 1)]
    pub n: u8,

    /// Terms of the sequence
    #[clap(value_parser, num_args = 1.., value_delimiter = ' ')]
    pub terms: Vec<String>,
}

fn main() -> Result<(), String> {
    let args = Args::parse();

    let terms = args
        .terms
        .iter()
        .map(|x| Term::from_str(x.trim_matches([',', ' '].as_slice())))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    let seq =
        Sequence::try_from(terms.as_slice()).map_err(|_| "Couldn't find pattern".to_owned())?;

    seq.into_iter()
        .skip(terms.len())
        .take(args.n as usize)
        .for_each(|x| println!("{}", x));

    Ok(())
}
