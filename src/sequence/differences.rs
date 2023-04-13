#[derive(Debug, PartialEq)]
pub struct Differences {
    start: i32,
    step: i32,
}


impl TryFrom<&[i32]> for Differences {
    type Error = ();

    fn try_from(value: &[i32]) -> Result<Self, Self::Error> {
        if value.len() < 3 {
            return Err(());
        }
        
        Err(())
    }
}