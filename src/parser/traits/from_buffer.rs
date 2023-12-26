use std::io::BufRead;
use std::marker::Sized;
pub trait FromBuf: Sized {
    type Err;

    // Required method
    fn from_buf<T>(s: &str) -> Result<Self, Self::Err>
    where
        T: BufRead;
}
