use serde::{Deserialize, Serialize};

use crate::Term;

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Root {
    pub results: Option<Vec<Response>>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Response {
    pub number: u64,
    pub data: String,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Oeis {
    pub numbers: Vec<Term>,
}

impl std::fmt::Display for Oeis {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let nums: String = self
            .numbers
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<String>>()
            .join(", ");
        write!(f, "Oeis({nums})")
    }
}

impl Iterator for Oeis {
    type Item = Term;

    fn next(&mut self) -> Option<Self::Item> {
        if self.numbers.is_empty() {
            return None;
        }
        Some(self.numbers.remove(0))
    }
}

impl TryFrom<&[Term]> for Oeis {
    type Error = ();

    fn try_from(value: &[Term]) -> Result<Self, Self::Error> {
        // convert sequence to integers, error if not integers
        let numbers: Vec<i128> = value
            .iter()
            .map(|&x| {
                if x.denom() == Some(&1) {
                    Ok(*x.numer().unwrap())
                } else {
                    Err(())
                }
            })
            .collect::<Result<Vec<_>, _>>()?;

        // query oeis.org
        let nums: String = numbers.iter().map(|x| format!("{x},")).collect();
        let url = format!("https://oeis.org/search?q={nums}&fmt=json");
        let response: Root = ureq::get(&url)
            .call()
            .map_err(|_| ())?
            .into_json()
            .map_err(|_| ())?;

        if let Some(results) = response.results {
            let nums: Vec<Term> = results[0]
                .data
                .split(',')
                .map(str::parse::<i64>)
                .take_while(Result::is_ok)
                .map(|f| Term::from(f.unwrap()))
                .collect();

            // match subsequence
            for (i, w) in nums.windows(numbers.len()).enumerate() {
                if w == value {
                    return Ok(Self {
                        numbers: nums[i..].to_vec(),
                    });
                }
            }
        }
        Err(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_one() {
        let seq = vec![1, 4, 7, 8, 9, 6];
        let seq: Vec<Term> = seq.iter().map(|&x| Term::from(x)).collect();
        let oeis = Oeis::try_from(seq.as_slice()).unwrap();
        println!("{:?}", oeis.numbers)
    }
}
