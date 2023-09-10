#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
mod sequence;

use fraction::GenericFraction;
use sequence::Sequence;
use std::str::FromStr;
pub type Term = GenericFraction<i128>;

use clap::Parser;

/// Arguments for the program
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of terms to generate
    #[arg(short, default_value_t = 1)]
    pub n: u8,

    /// Start at the beggining of the sequence
    #[arg(short, long, default_value = "false")]
    pub info: bool,

    /// Start at the beggining of the sequence
    #[arg(short, long, default_value = "false")]
    pub beginning: bool,

    /// Terms of the sequence, comma or space separated
    #[clap(value_parser, num_args = 1.., value_delimiter = ' ')]
    pub terms: Vec<String>,
}

fn main() -> Result<(), String> {
    let args = Args::parse();

    let terms = args
        .terms
        .iter()
        .flat_map(|x| x.trim_end_matches(',').split([',', ' ']))
        .map(Term::from_str)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| format!("Failed to parse sequence: {:?}", args.terms))?;

    let seq =
        Sequence::try_from(terms.as_slice()).map_err(|_err| "Couldn't find pattern".to_owned())?;

    if args.info {
        println!("{seq}");
    }

    seq.into_iter()
        .skip(if args.beginning { 0 } else { terms.len() })
        .take(usize::from(args.n))
        .for_each(|x| println!("{x}"));

    Ok(())
}
