use crate::Term;

/// Sequences where the next term can be expressed as
/// a(n) = a*A(n-2) + b*A(n-1) + c
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Fibonacci {
    pub s0: Term,
    pub s1: Term,
    pub a: Term,
    pub b: Term,
}

impl std::fmt::Display for Fibonacci {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            // "a(0) = {}; a(1) = {}; a(n)={}*A(n-2) + {}*A(n-1)",
            "Fib({}, {}, {}, {})",
            self.s0, self.s1, self.a, self.b
        )
    }
}

impl Iterator for Fibonacci {
    type Item = Term;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.a * self.s0 + self.b * self.s1;
        let tmp = self.s0;
        self.s0 = self.s1;
        self.s1 = next;
        Some(tmp)
    }
}

impl TryFrom<&[Term]> for Fibonacci {
    type Error = ();

    #[allow(clippy::suspicious_operation_groupings)]
    fn try_from(value: &[Term]) -> Result<Self, Self::Error> {
        if value.len() < 5 {
            return Err(());
        }

        // solve system of 2 equations
        // sequence: a, b, c, d
        // (1) x * a + y * b = c
        // (2) x * b + y * c = d

        let s0 = value[0];
        let s1 = value[1];
        let s2 = value[2];
        let s3 = value[3];

        if s1 * s1 == s0 * s2 {
            return Err(());
        }

        let x = (s2 * s2 - s1 * s3) / (s0 * s2 - s1 * s1);
        let y = (s1 * s2 - s0 * s3) / (s1 * s1 - s0 * s2);

        let seq = Self { s0, s1, a: x, b: y };

        if seq.clone().zip(value.iter()).all(|(a, b)| a == *b) {
            Ok(seq)
        } else {
            Err(())
        }
    }
}
