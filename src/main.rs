#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
mod sequence;

use fraction::Ratio;
use sequence::Sequence;
use std::str::FromStr;

pub type Size = i128;
pub type Term = Ratio<Size>;

use clap::Parser;

/// Arguments for the program
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The number of terms to generate
    #[arg(short, default_value_t = 1)]
    pub n: u8,

    /// Print the sequence type
    #[arg(long, default_value = "false")]
    pub info: bool,

    /// Start at the beggining of the sequence
    #[arg(short, long, default_value = "false")]
    pub beginning: bool,

    /// Terms of the sequence, comma or space separated
    #[clap(value_parser, num_args = 1.., value_delimiter = ' ')]
    pub terms: Vec<String>,
}

/// Tokenize sequence of arguments.
/// - a,b,c
/// - a, b, c
/// - 1 2 3
/// - 1,2,3
fn tokenize_args<'a>(args: &'a [String]) -> Vec<&'a str> {
    args.iter()
        .flat_map(|x| x.trim_end_matches(',').split([',', ' ']))
        .collect::<Vec<_>>()
}

/// Parse a sequence of characters. Contains letters, symbols, other characters...
/// a=1, b=2, c=3, ...
/// treat each term as a single character
fn parse_symbol_sequence(terms: &[&str]) -> Option<Vec<Term>> {
    let terms = terms
        .iter()
        .map(|x| try_str_to_char(*x))
        .collect::<Result<Vec<char>, _>>()
        .ok()?;

    let terms = terms
        .into_iter()
        .map(|x| u32::from(x) - u32::from('a') + 1)
        .map(|x| Term::new(Size::from(x), 1))
        .collect::<Vec<Term>>();

    Some(terms)
}

/// Parses sequence of digits / fractions.
fn parse_ratio_sequence(terms: &[&str]) -> Option<Vec<Term>> {
    if terms
        .iter()
        .flat_map(|x| x.chars())
        .all(|x| char::is_digit(x, 10) || x == '/')
    {
        terms
            .into_iter()
            .map(|x| Term::from_str(x))
            .collect::<Result<Vec<_>, _>>()
            .ok()
    } else {
        None
    }
}

/// Convert integer to character only if ascii alphabetic.
fn term_to_char(term: Term) -> Result<char, ()> {
    let term = term.to_integer();
    let z = u32::try_from(term).map_err(|_| ())?;
    let character = char::try_from(z).map_err(|_| ());
    match character {
        Ok(x) if x.is_ascii_alphabetic() => Ok(x),
        Ok(_) | Err(_) => Err(()),
    }
}

// fn parse_with_offset(terms: &[Term]) -> Result<Sequence, String> {
//     let seq = Sequence::try_from(terms).map_err(|_err| "Couldn't find pattern".to_owned())?;

//     let seq_iter = seq
//         .into_iter()
//         .skip(if args.beginning { 0 } else { terms.len() })
//         .map(|x| term_to_char(x))
//         .take(usize::from(args.n));

//     seq_iter.for_each(|x| println!("{x}"));
// }

fn parse_ratio_sequence2(terms: &[&str], args: &Args) -> Option<()> {
    // try to parse terms as sequence of ratios
    let terms = terms
        .into_iter()
        .map(|x| Term::from_str(x))
        .collect::<Result<Vec<_>, _>>()
        .ok()?;

    let seq = Sequence::try_from(terms.as_slice()).ok()?;

    seq.into_iter()
        .skip(if args.beginning { 0 } else { terms.len() })
        .take(usize::from(args.n))
        .for_each(|x| println!("{x}"));

    Some(())
}

fn try_str_to_char(value: &str) -> Result<char, ()> {
    let mut chars = value.chars();

    match (chars.next(), chars.next()) {
        (Some(_), Some(_)) => Err(()),
        (Some(first_char), None) => Ok(first_char),
        (None, _) => Err(()),
    }
}

fn parse_symbol_sequence2(terms: &[&str], args: &Args) -> Option<()> {
    // try to parse as sequence of characters
    let terms = terms
        .into_iter()
        .map(|x| try_str_to_char(*x))
        .collect::<Result<Vec<char>, _>>()
        .ok()?;

    // convert char to integer sequence
    for offset in &[97 - 1, 65 - 1, 65, 97] {
        // map char to integer with offset to align e.g. a=1, b=2, c=3, ...
        let terms = terms
            .iter()
            .map(|&x| Term::from(i128::from(u32::from(x)) - offset))
            .collect::<Vec<_>>();
        terms.iter().for_each(|x| print!("{},", x.to_integer()));
        println!();

        if let Ok(seq) = Sequence::try_from(terms.as_slice()) {
            println!("{}", seq);
            seq.into_iter()
                .skip(if args.beginning { 0 } else { terms.len() })
                .take(usize::from(args.n))
                .map(|x| term_to_char(x + offset))
                .take_while(|&x| x.is_ok())
                .for_each(|x| println!("{}", x.unwrap()));

            break;
        }
    }

    Some(())
}

// fn parse_the_thing(terms: &[&str]) -> Result<(), String> {
//     let (terms, is_symbol) = if let Some(terms) = parse_ratio_sequence(terms) {
//         (terms, false)
//     } else if let Some(terms) = parse_symbol_sequence(terms) {
//         (terms, true)
//     } else {
//         return Err("Couldn't parse".to_owned());
//     };

//     if is_symbol {
//         // shift the sequence so A maps to 0
//         for offset in &[0, 1, -1] {
//             let offset_terms: Vec<Ratio<Size>> = terms
//                 .into_iter()
//                 .map(|x| x - Term::new('a' as i128 + offset, 1))
//                 .collect();

//             let seq = Sequence::try_from(offset_terms.as_slice())
//                 .map_err(|_err| "Couldn't find pattern".to_owned())?;

//             seq.into_iter()
//                 .map(|x| x - offset)
//                 .take_while(|&x| x >= Ratio::from(65) && x <= Ratio::from(90));
//         }
//     } else {
//         let seq = Sequence::try_from(terms.as_slice())
//             .map_err(|_err| "Couldn't find pattern".to_owned())?;
//     };

//     Ok(())
// }

fn main() -> Result<(), String> {
    let args = Args::parse();

    let terms = tokenize_args(&args.terms);

    if let Some(()) = parse_ratio_sequence2(&terms, &args) {
        return Ok(());
    }

    if let Some(()) = parse_symbol_sequence2(&terms, &args) {
        return Ok(());
    }

    // if let Some(terms) = parse_ratio_sequence(&terms) {
    //     let seq = Sequence::try_from(terms.as_slice())
    //         .map_err(|_err| "Couldn't find pattern".to_owned())?;

    //     if args.info {
    //         println!("{seq}");
    //     }

    //     let seq_iter = seq
    //         .into_iter()
    //         .skip(if args.beginning { 0 } else { terms.len() })
    //         .take(usize::from(args.n));

    //     seq_iter.for_each(|x| println!("{x}"));
    // } else if let Some(terms) = parse_symbol_sequence(&terms) {
    //     let seq = Sequence::try_from(terms.as_slice())
    //         .map_err(|_err| "Couldn't find pattern".to_owned())?;

    //     let seq_iter = seq
    //         .into_iter()
    //         .skip(if args.beginning { 0 } else { terms.len() })
    //         .map(|x| term_to_char(x))
    //         .take(usize::from(args.n));

    //     seq_iter.for_each(|x| println!("{x}"));
    // } else {
    //     Err(format!("Failed to parse sequence: {:?}", args.terms))?
    // };

    Ok(())
}
