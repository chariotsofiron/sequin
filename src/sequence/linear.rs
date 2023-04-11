use super::sequence::Sequence;

#[derive(Debug, PartialEq)]
pub struct Linear {
    start: i32,
    step: i32,
}

impl Linear {
    pub fn new(start: i32, step: i32) -> Self {
        Linear { start, step }
    }
}

impl Sequence<'_> for Linear {
    fn iter(&self) -> Box<dyn Iterator<Item = i32> + '_> {
        Box::new((0..).map(|x| x * self.step + self.start))
    }
}

impl TryFrom<&[i32]> for Linear {
    type Error = ();

    fn try_from(value: &[i32]) -> Result<Self, Self::Error> {
        if value.len() < 3 {
            return Err(());
        }
        let diff = value[1] - value[0];
        for window in value.windows(2) {
            if window[1] - window[0] != diff {
                return Err(());
            }
        }
        Ok(Linear {
            start: value[0],
            step: diff,
        })
    }
}

impl Iterator for Linear {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        Some(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear() {
        let linear = Linear::new(1, 2);
        let mut iter = linear.iter();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(5));
        assert_eq!(iter.next(), Some(7));
        assert_eq!(iter.next(), Some(9));
    }
}
