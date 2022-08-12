use std::marker::PhantomData;
use std::borrow::Cow;

use crate::io;
use crate::serde::{ByteTypeId, ByteDeserialize, ByteSerialize, ParseOrIOError};

#[derive(Default, Copy, Clone)]
pub struct LengthPrefixString<LE>
where
    LE: ByteTypeId<u64>,
{
    length_encoder: PhantomData<LE>,
}

#[derive(Debug)]
pub enum StringParseError<L> {
    LengthParseError(L),
    InvalidUtf8Error(std::str::Utf8Error),
    InvalidUtf8OwnedError(std::string::FromUtf8Error),
}

impl<L> From<L> for StringParseError<L> {
    fn from(l: L) -> Self {
        StringParseError::LengthParseError(l)
    }
}

impl<LE> ByteTypeId<str> for LengthPrefixString<LE>
where
    LE: ByteTypeId<u64>,
{
    fn byte_type_id() -> Vec<&'static str> {
        let mut res = Vec::new();
        res.push("twee::LengthPrefixedString<");
        res.extend_from_slice(LE::byte_type_id().as_slice());
        res.push(">");
        res
    }
}

impl<LE> ByteTypeId<String> for LengthPrefixString<LE>
where
    LE: ByteTypeId<u64>,
{
    fn byte_type_id() -> Vec<&'static str> {
        <Self as ByteTypeId<str>>::byte_type_id()
    }
}

impl<LE> ByteTypeId<Cow<'_, str>> for LengthPrefixString<LE>
where
    LE: ByteTypeId<u64>,
{
    fn byte_type_id() -> Vec<&'static str> {
        <Self as ByteTypeId<str>>::byte_type_id()
    }
}

impl<LE> ByteSerialize<str> for LengthPrefixString<LE>
where
    LE: ByteSerialize<u64>,
{
    fn byte_serialize<W: io::ByteWrite>(item: &str, io: &mut W) -> Result<(), W::Err> {
        let len:u64 = item.len().try_into().unwrap();
        LE::byte_serialize(&len, io)?;
        io.write_buf(item.as_bytes())
    }

    fn size(item: &str) -> u64 {
        let len:u64 = item.len().try_into().unwrap();
        LE::size(&len) + len
    }
}

impl<LE> ByteDeserialize<String> for LengthPrefixString<LE>
where
    LE: ByteDeserialize<u64>,
{
    type ParseErr = StringParseError<LE::ParseErr>;

    fn byte_deserialize<R: io::ByteRead>(io: &mut R) -> Result<String, ParseOrIOError<Self::ParseErr, R::Err>> {
        let len:u64 = LE::byte_deserialize(io).map_err(|e| e.map_parse(StringParseError::LengthParseError))?;
        let mut horror = Vec::with_capacity(len.try_into().unwrap());
        for _ in 0..len {
            horror.push(io.read_byte().map_err(ParseOrIOError::IO)?);
        }
        String::from_utf8(horror).map_err(StringParseError::InvalidUtf8OwnedError).map_err(ParseOrIOError::Parse)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn blarg() {
        use crate::endians::NumberEncodingBE as BE;
        let a = "123abc\u{0}<- Thats a null char";
        crate::assert_serde_through::<
            LengthPrefixString<BE>,
            str,
            String,
        >(a);
    }
}