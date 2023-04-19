use crate::Term;

#[derive(Debug, PartialEq, Clone)]
pub struct Differences {
    pub terms: Vec<Term>,
}

impl Iterator for Differences {
    type Item = Term;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.terms[0];
        for i in 1..self.terms.len() {
            self.terms[i - 1] = self.terms[i - 1] + self.terms[i];
        }
        Some(next)
    }
}

impl TryFrom<&[Term]> for Differences {
    type Error = ();

    fn try_from(value: &[Term]) -> Result<Self, Self::Error> {
        if value.len() < 3 {
            return Err(());
        }
        let mut terms = vec![value[0]];
        let mut differences: Vec<Term> = value.to_vec();
        for _ in 0..value.len() - 2 {
            differences = differences.windows(2).map(|w| w[1] - w[0]).collect();
            terms.push(differences[0]);
            if differences.windows(2).all(|w| w[0] == w[1]) {
                return Ok(Differences { terms });
            }
        }
        Err(())
    }
}
