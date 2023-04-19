use serde::{Deserialize, Serialize};

use crate::Term;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Root {
    pub results: Option<Vec<Response>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Response {
    pub number: u64,
    pub data: String,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Oeis {
    pub numbers: Vec<Term>,
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
        let numbers: Vec<i64> = value
            .into_iter()
            .map(|&x| {
                if x.denom() == Some(&1) {
                    Ok(*x.numer().unwrap())
                } else {
                    Err(())
                }
            })
            .collect::<Result<Vec<i64>, _>>()?;

        // query oeis.org
        let nums: String = numbers.iter().map(|x| format!("{},", x)).collect();
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
                .map(|x| x.parse::<i64>())
                .take_while(|x| x.is_ok())
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
