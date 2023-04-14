use crate::Fraction;

/// Sequences where the next term is ax+b where x
/// is the previous term and a and b are constants.
/// Covers the linear sequence case.
#[derive(Debug, PartialEq)]
pub struct Binom {
    pub start: Fraction,
    pub a: Fraction,
    pub b: Fraction,
}

impl TryFrom<&[Fraction]> for Binom {
    type Error = ();

    fn try_from(value: &[Fraction]) -> Result<Self, Self::Error> {
        if value.len() < 3 {
            return Err(());
        }

        for num in -10..10 {
            for denom in 1..10 {
                let a = Fraction::from(num) / Fraction::from(denom);
                for offset in -10..10 {
                    let b = Fraction::from(offset);
                    let mut prev = value[0];
                    let mut ok = true;
                    for &next in value.iter().skip(1) {
                        if a * prev + b != next {
                            ok = false;
                            break;
                        }
                        prev = next;
                    }
                    if ok {
                        return Ok(Binom {
                            start: Fraction::from(value[0]),
                            a,
                            b,
                        });
                    }
                }
            }
        }
        Err(())
    }
}

impl Iterator for Binom {
    type Item = Fraction;

    fn next(&mut self) -> Option<Self::Item> {
        let ans = self.start;
        self.start = self.a * self.start + self.b;
        Some(ans)
    }
}