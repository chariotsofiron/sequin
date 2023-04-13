/// Sequences where the next term is ax+b where x is the previous term and a and b are constants.
#[derive(Debug, PartialEq)]
pub struct PrevBinom {
    start: i32,
    a: i32,
    b: i32,
}


impl TryFrom<&[i32]> for PrevBinom {
    type Error = ();

    fn try_from(value: &[i32]) -> Result<Self, Self::Error> {
        if value.len() < 3 {
            return Err(());
        }
        
        Err(())
    }
}