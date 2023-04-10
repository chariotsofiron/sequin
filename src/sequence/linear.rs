use super::sequence::Sequence;

#[derive(Debug)]
pub struct Linear {
    start: i32,
    increment: i32,
}

impl Sequence<'_> for Linear {
    fn iter(&self) -> _ {
        (self.start..).map(|x| x * self.increment)
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
            increment: diff,
        })
    }
}

impl Iterator for Linear {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        Some(0)
    }
}
