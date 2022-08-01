use std::fmt::{self, Display};
use std::error::Error;

use super::io;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ParseOrIOError<P, I> {
    Parse(P),
    IO(I),
}

impl<P, I> Display for ParseOrIOError<P, I>
where
    P: Display,
    I: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Parse(e) => write!(f, "parse error: {}", e),
            Self::IO(e) => write!(f, "io error: {}", e),
        }
    }
}

impl<P, I> Error for ParseOrIOError<P, I>
where
    P: Error + 'static,
    I: Error + 'static,
{
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            Self::Parse(ref e) => Some(e),
            Self::IO(ref e) => Some(e),
        }
    }
}

impl<P, I> ParseOrIOError<P, I> {
    pub fn map_parse<Func, NewParse>(self, op: Func) -> ParseOrIOError<NewParse, I>
    where
        Func: FnOnce(P) -> NewParse,
    {
        match self {
            Self::Parse(p) => ParseOrIOError::Parse(op(p)),
            Self::IO(i) => ParseOrIOError::IO(i),
        }
    }
    
    pub fn map_io<Func, NewIO>(self, op: Func) -> ParseOrIOError<P, NewIO>
    where
        Func: FnOnce(I) -> NewIO,
    {
        match self {
            Self::Parse(p) => ParseOrIOError::Parse(p),
            Self::IO(i) => ParseOrIOError::IO(op(i)),
        }
    }
}

/// Types that implement this trait should be unit structs or structs with only [`PhantomData`][`std::marker::PhantomData`]-type members. This is encouraged through the `Default` and `Copy` requirements
pub trait ByteTypeId<T: ?Sized>: Default + Copy {
    /// This *must* always give the same value; This is not an associated const because of rust limitations.
    fn byte_type_id() -> Vec<&'static str>;
}

pub trait ByteDeserialize<T> : ByteTypeId<T> {
    type ParseErr;

    fn byte_deserialize<R: io::ByteRead>(io: &mut R) -> Result<T, ParseOrIOError<Self::ParseErr, R::Err>>;

    fn guess_size() -> Option<usize> { None }
}

pub trait ByteSerialize<T: ?Sized> : ByteTypeId<T> {
    fn byte_serialize<W: io::ByteWrite>(item: &T, io: &mut W) -> Result<(), W::Err>;

    fn size(item: &T) -> u64 {
        let mut io = io::ByteCounter::default();
        Self::byte_serialize(item, &mut io).unwrap();
        io.count
    }
}

pub trait ByteConstSize<T> : ByteTypeId<T> {
    const BYTE_SIZE:usize;
}

pub trait ByteConstDeserialize<T> : ByteConstSize<T> {
    type ParseErr;

    fn byte_const_deserialize(io: &[u8; Self::BYTE_SIZE]) -> Result<T, Self::ParseErr>;
}

impl<P, T> ByteDeserialize<T> for P
where P: ByteConstDeserialize<T>, [(); Self::BYTE_SIZE]: {
    type ParseErr = <Self as ByteConstDeserialize<T>>::ParseErr;

    fn byte_deserialize<R: io::ByteRead>(io: &mut R) -> Result<T, ParseOrIOError<Self::ParseErr, R::Err>> {
        let buf_cow = io.read_buf(Self::BYTE_SIZE.try_into().unwrap()).map_err(ParseOrIOError::IO)?;
        let buf_slice:&[u8] = buf_cow.as_ref();
        let buf_arr:&[u8; Self::BYTE_SIZE] = buf_slice.try_into().unwrap();
        Self::byte_const_deserialize(buf_arr).map_err(ParseOrIOError::Parse)
    }

    fn guess_size() -> Option<usize> {
        Some(Self::BYTE_SIZE)
    }
} 

pub trait ByteConstSerialize<T> : ByteConstSize<T> {
    fn byte_const_serialize(item: &T, io: &mut [u8; Self::BYTE_SIZE]);
}

impl<P, T> ByteSerialize<T> for P
where P: ByteConstSerialize<T>, [(); Self::BYTE_SIZE]: {
    fn byte_serialize<W: io::ByteWrite>(item: &T, io: &mut W) -> Result<(), W::Err> {
        let mut buf = [0u8; Self::BYTE_SIZE];
        Self::byte_const_serialize(item, &mut buf);
        io.write_buf(&buf)
    }

    fn size(_item: &T) -> u64 {
        Self::BYTE_SIZE.try_into().unwrap()
    }
}