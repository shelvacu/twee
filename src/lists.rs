use std::marker::PhantomData;

use crate::io;
use crate::serde::{ByteTypeId, ByteDeserialize, ByteSerialize};

#[derive(Default, Debug, Clone, Copy)]
pub struct LengthPrefixList<LE, VE>
where
    LE: ByteTypeId<u64>,
{
    length_encoder: PhantomData<LE>,
    value_encoder: PhantomData<VE>,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ListParseError<L, V> {
    LengthParseError(L),
    ItemParseError{idx: u64, error: V},
}

impl<L, V> From<L> for ListParseError<L, V> {
    fn from(l: L) -> Self {
        ListParseError::LengthParseError(l)
    }
}

macro_rules! impl_byte_type {
    ($v:ident, $t:ty) => {
        impl<LE, VE, $v> ByteTypeId<$t> for LengthPrefixList<LE, VE>
        where
            LE: ByteTypeId<u64>,
            VE: ByteTypeId<$v>,
        {
            fn byte_type_id() -> Vec<&'static str> {
                let mut res = Vec::new();
                res.push("twee::LengthPrefixedList<");
                res.extend_from_slice(LE::byte_type_id().as_slice());
                res.push(",");
                res.extend_from_slice(VE::byte_type_id().as_slice());
                res.push(">");
                res
            }
        }
    }
}

impl_byte_type!{V, Vec<V>}
impl_byte_type!{V, [V]}

impl<LE, VE, V> ByteDeserialize<Vec<V>> for LengthPrefixList<LE, VE>
where
    LE: ByteDeserialize<u64>,
    VE: ByteDeserialize<V>,
{
    type ParseErr = ListParseError<LE::ParseErr, VE::ParseErr>;

    fn byte_deserialize<R: io::ByteRead>(io: &mut R) -> Result<Vec<V>, Self::ParseErr> {
        let length:u64 = LE::byte_deserialize(io)?;

        let mut res = Vec::with_capacity(length.try_into().unwrap());

        for idx in 0..length {
            res.push(VE::byte_deserialize(io).map_err(|e| ListParseError::ItemParseError{idx, error: e})?);
        }
        Ok(res)
    }
}

impl<LE, VE, V> ByteSerialize<[V]> for LengthPrefixList<LE, VE>
where
    LE: ByteSerialize<u64>,
    VE: ByteSerialize<V>,
{ 
    fn byte_serialize<W: io::ByteWrite>(item: &[V], io: &mut W) {
        let length:u64 = item.len().try_into().unwrap();
        LE::byte_serialize(&length, io);

        for el in item.iter() {
            VE::byte_serialize(&el, io);
        }
    }

    fn size(item: &[V]) -> u64 {
        let length:u64 = item.len().try_into().unwrap();
        LE::size(&length) + item.iter().map(VE::size).sum::<u64>()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn blarg() {
        use crate::endians::NumberEncodingBE as BE;
        let a:[u64; 3] = [1, 2, 5];
        crate::assert_serde_through::<
            LengthPrefixList<BE, BE>,
            [_],
            Vec<_>,
        >(a.as_slice());
    }
}

// impl<LE, VE, V> ByteSerialize<Vec<V>> for LengthPrefixList<LE, VE, V>
// where
//     LE: ByteSerialize<u64>,
//     VE: ByteSerialize<V>,
// { 
//     fn byte_serialize<W: io::ByteWrite>(item: &Vec<V>, io: &mut W) {
//         Self::byte_serialize(item.as_slice(), io);
//     }

//     fn size(item: &Vec<V>) -> u64 {
//         Self::size(item.as_slice())
//     }
// }

// impl<LE, VE, V, T> ByteTypeId<T> for LengthPrefixList<LE, VE, V>
// where
//     LE: ByteTypeId<u64>,
//     VE: ByteTypeId<V>,
//     T: Iterator<Item = V> + std::iter::ExactSizeIterator,
// {
//     fn byte_type_id() -> Vec<&'static str> {
//         let mut res = Vec::new();
//         res.push("twee::LengthPrefixedList<");
//         res.extend_from_slice(LE::byte_type_id().as_slice());
//         res.push(",");
//         res.extend_from_slice(VE::byte_type_id().as_slice());
//         res.push(">");
//         res
//     }
// }

// impl<LE, VE, V, T> ByteSerialize<T> for LengthPrefixList<LE, VE, V>
// where
//     LE: ByteSerialize<u64>,
//     VE: ByteSerialize<V>,
//     T: Iterator<Item = V> + std::iter::ExactSizeIterator,
// {
//     fn byte_serialize<W: io::ByteWrite>(item: &T, io: &mut W) {
//         todo!()
//     }

//     fn size(item: &T) -> u64 {
//         todo!()
//     }
// }