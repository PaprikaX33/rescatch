use std::io::BufRead;
use std::marker::Sized;
pub trait FromBuf: Sized {
    type Err;

    // Required method
    fn from_buf<T>(s: &mut T) -> Result<Self, Self::Err>
    where
        T: BufRead;
}
