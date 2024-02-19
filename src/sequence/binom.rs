use crate::Term;
use fraction::Zero;

/// Sequences where the next term is ax+b where x
/// is the previous term and a and b are constants.
/// Covers the linear sequence case.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Binomial {
    start: Term,
    a: Term,
    b: Term,
}

impl Binomial {
    pub fn new<X, Y, Z>(start: X, a: Y, b: Z) -> Self
    where
        X: Into<Term>,
        Y: Into<Term>,
        Z: Into<Term>,
    {
        Self {
            start: start.into(),
            a: a.into(),
            b: b.into(),
        }
    }
}

impl std::fmt::Display for Binomial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Binom({}, {}, {})", self.start, self.a, self.b)
    }
}

// impl Binomial {
//     fn closed_form(&self) -> String {
//         // https://math.stackexchange.com/a/2194232
//         format!("a(n) = {}*{}^n + {}*{}^n", self.a, self.a, self.b, self.b)
//     }
// }

impl TryFrom<&[Term]> for Binomial {
    type Error = ();

    fn try_from(value: &[Term]) -> Result<Self, Self::Error> {
        if value.len() < 3 {
            return Err(());
        }
        if value[0] == Term::zero() && value[0] != value[1] && value.len() < 4 {
            return Err(());
        }

        // x, y, z
        // a * x + b = y
        // a * y + b = z
        // a * (x - y) = y - z
        // a = (y - z) / (x - y)

        let seq = if value[0] == value[1] && value[1] == value[2] {
            Self::new(value[0], 0, value[0])
        } else if value[0] == value[1] {
            Err(())?
        } else {
            let a = (value[1] - value[2]) / (value[0] - value[1]);
            let b = value[1] - a * value[0];
            // b should be an integer
            if b.denom() != &1 {
                Err(())?;
            }
            Self::new(value[0], a, b)
        };

        if seq.clone().zip(value.iter()).skip(2).all(|(a, b)| a == *b) {
            Ok(seq)
        } else {
            Err(())
        }
    }
}

impl Iterator for Binomial {
    type Item = Term;

    fn next(&mut self) -> Option<Self::Item> {
        let ans = self.start;
        self.start = self.a * self.start + self.b;
        Some(ans)
    }
}
