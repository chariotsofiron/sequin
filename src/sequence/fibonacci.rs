use crate::Term;

/// Sequences where the next term can be expressed as
/// a(n) = a*A(n-2) + b*A(n-1) + c
#[derive(Debug, PartialEq, Clone)]
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
            // "a(0) = {}\na(1) = {}\na(n) = {}*A(n-2) + {}*A(n-1) + {}",
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

    fn try_from(value: &[Term]) -> Result<Self, Self::Error> {
        if value.len() < 4 {
            return Err(());
        }
        for a in (-2..=2).into_iter().map(|x| Term::from(x)) {
            for b in (-2..=2).into_iter().map(|x| Term::from(x)) {
                let mut ok = true;
                for w in value.windows(3) {
                    if a * w[0] + b * w[1] != w[2] {
                        ok = false;
                        break;
                    }
                }
                if ok {
                    return Ok(Fibonacci {
                        s0: Term::from(value[0]),
                        s1: Term::from(value[1]),
                        a,
                        b,
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

    // #[test]
    // fn test() {
    //     let nums = [18, 6, 24, 8, 32];
    //     let nums = nums.into_iter().map(|n| Term::from(n)).collect::<Vec<_>>();
    //     let seq = Fibonacci::try_from(nums.as_slice()).unwrap();
    //     println!("{}", seq);
    // }
}