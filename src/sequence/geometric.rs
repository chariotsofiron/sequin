use super::sequence::Sequence;

#[derive(Debug, PartialEq)]
pub struct Geometric {
    start: i32,
    factor: i32,
}

impl Geometric {
    pub fn new(start: i32, factor: i32) -> Self {
        Geometric { start, factor }
    }
}

impl Sequence for Geometric {
    fn iter(&self) -> Box<dyn Iterator<Item = i32>> {
        let factor = self.factor;
        let start = self.start;
        Box::new((0..).map(move |x| factor.pow(x as u32) * start))
    }
}

impl TryFrom<&[i32]> for Geometric {
    type Error = ();

    fn try_from(value: &[i32]) -> Result<Self, Self::Error> {
        if value.len() < 3 {
            return Err(());
        }
        let factor = value[1] / value[0];
        for window in value.windows(2) {
            if window[1] / window[0] != factor {
                return Err(());
            }
        }
        Ok(Geometric {
            start: value[0],
            factor,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_geometric() {
        let geometric = Geometric::new(1, 2);
        let mut iter = geometric.iter();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(4));
        assert_eq!(iter.next(), Some(8));

        assert_eq!(
            Geometric::try_from([1, 2, 4, 8, 16].as_slice()),
            Ok(Geometric::new(1, 2))
        );
    }
}
