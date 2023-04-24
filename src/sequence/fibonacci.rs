use crate::Term;

/// Sequences where the next term can be expressed as
/// a(n) = a*A(n-2) + b*A(n-1) + c
#[derive(Debug, PartialEq, Clone)]
pub struct Fibonacci {
    pub s0: Term,
    pub s1: Term,
    pub a: Term,
    pub b: Term,
    pub c: Term,
}

impl std::fmt::Display for Fibonacci {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            // "a(0) = {}\na(1) = {}\na(n) = {}*A(n-2) + {}*A(n-1) + {}",
            "Fib({}, {}, {}, {}, {})",
            self.s0, self.s1, self.a, self.b, self.c
        )
    }
}

impl Iterator for Fibonacci {
    type Item = Term;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.a * self.s0 + self.b * self.s1 + self.c;
        let tmp = self.s0;
        self.s0 = self.s1;
        self.s1 = next;
        Some(tmp)
    }
}

impl TryFrom<&[Term]> for Fibonacci {
    type Error = ();

    fn try_from(value: &[Term]) -> Result<Self, Self::Error> {
        if value.len() < 5 {
            return Err(());
        }

        // solve system of 3 equations
        // sequence: a, b, c, d, e

        // (1) x * a + y * b + z = c
        // (2) x * b + y * c + z = d
        // (3) x * c + y * d + z = e

        // (2-1 A): x * (b - a) + y * (c - b) = d - c
        // (3-2 B): x * (c - b) + y * (d - c) = e - d

        // A': x * (b-a)(c-b) + y * (c-b)(c-b) = (d-c)(c-b)
        // B': x * (c-b)(b-a) + y * (d-c)(b-a) = (e-d)(b-a)

        // y * (d-c)(b-a) - y * (c-b)^2 = (e-d)(b-a) - (d-c)(c-b)
        // y [(d-c)(b-a) - (c-b)^2] = (e-d)(b-a) - (d-c)(c-b)
        // y = [(e-d)(b-a) - (d-c)(c-b)] / [(d-c)(b-a) - (c-b)^2]

        let a = value[0];
        let b = value[1];
        let c = value[2];
        let d = value[3];
        let e = value[4];

        let y = ((e - d) * (b - a) - (d - c) * (c - b)) / ((d - c) * (b - a) - (c - b) * (c - b));

        let x = ((d - c)
            - (c - b) * ((e - d) * (b - a) - (d - c) * (c - b))
                / ((d - c) * (b - a) - (c - b) * (c - b)))
            / (b - a);

        let z = c - x * a - y * b;

        let seq = Fibonacci {
            s0: Term::from(a),
            s1: Term::from(b),
            a: x,
            b: y,
            c: z,
        };

        if seq
            .clone()
            .into_iter()
            .zip(value.iter())
            .all(|(a, b)| a == *b)
        {
            Ok(seq)
        } else {
            Err(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = [13, -21, 34, -55, 89, -144];
        // let nums = [34, -21, 13, -8, 5, -3];
        let nums = [1, 3, 7, 17, 41, 99, 239];
        let nums = [18, 6, 24, 8, 32];
        let nums = [54, 18, 72, 24, 96, 32];
        let nums = [-3, 3, 27, 69, 129, 207];
        let nums = nums.into_iter().map(|n| Term::from(n)).collect::<Vec<_>>();
        let seq = Fibonacci::try_from(nums.as_slice()).unwrap();
        println!("{}", seq);
    }
}
