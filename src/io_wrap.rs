use std::io as stdio;
use std::borrow::Cow;

use crate::io::{ByteRead, ByteWrite};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct StdWrapper<V> (pub V);


impl<V: stdio::Write> ByteWrite for StdWrapper<V> {
    type Err = stdio::Error;

    fn write_byte(&mut self, data: u8) -> Result<(), Self::Err> {
        self.0.write_all(&[data])
    }

    fn write_buf(&mut self, data: &[u8]) -> Result<(), Self::Err> {
        self.0.write_all(data)
    }
}

impl<V: stdio::Read> ByteRead for StdWrapper<V> {
    type Err = stdio::Error;

    fn read_buf<'a>(&'a mut self, len: u64) -> Result<Cow<'a, [u8]>, Self::Err> {
        let mut buf:Vec<u8> = (0..len).map(|_| 0).collect();
        self.0.read_exact(buf.as_mut_slice())?;
        Ok(buf.into())
    }
}