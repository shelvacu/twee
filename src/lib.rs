#![allow(incomplete_features)]
#![feature(generic_const_exprs, never_type)]

pub mod io;
pub mod serde;
pub mod endians;
pub mod varint;

pub fn assert_serde<E, T>(item: &T)
where
    E: serde::ByteSerialize<T> + serde::ByteDeserialize<T>,
    T: PartialEq + std::fmt::Debug,
    <E as serde::ByteDeserialize<T>>::ParseErr: std::fmt::Debug,
{
    let est_size = E::size(item);
    let mut buf:Vec<u8> = vec![];
    E::byte_serialize(item, &mut buf);
    assert_eq!(buf.len(), est_size);
    let mut cur = io::ByteCursor::new(buf.as_slice());
    let other:T = E::byte_deserialize(&mut cur).unwrap();
    assert_eq!(
        item,
        &other,
    )
}

// #[cfg(test)]
// mod tests {
//     use super::*;

// }
