#![allow(incomplete_features)]
#![feature(generic_const_exprs, never_type, maybe_uninit_array_assume_init, maybe_uninit_uninit_array)]

pub mod io;
pub mod serde;
pub mod endians;
pub mod varint;
pub mod lists;
pub mod const_list;
pub mod string;

//mod cursed;
pub fn assert_serde_across_through<Ein, Eout, Tin, Tout>(item: &Tin)
where
    Ein: serde::ByteSerialize<Tin>,
    Eout: serde::ByteDeserialize<Tout>,
    Tin: ?Sized + std::fmt::Debug + PartialEq<Tout>,
    Tout: std::fmt::Debug,
    <Eout as serde::ByteDeserialize<Tout>>::ParseErr: std::fmt::Debug,
{
    assert_eq!(
        <Eout as serde::ByteTypeId<Tout>>::byte_type_id(),
        <Ein as serde::ByteTypeId<Tin>>::byte_type_id(),
    );
    let est_size = Ein::size(item);
    let mut buf:Vec<u8> = vec![];
    Ein::byte_serialize(item, &mut buf);
    let buf_len:u64 = buf.len().try_into().unwrap();
    assert_eq!(buf_len, est_size);
    let mut cur = io::ByteCursor::new(buf.as_slice());
    let other:Tout = Eout::byte_deserialize(&mut cur).unwrap();
    assert_eq!(
        item,
        &other,
    )
}

pub fn assert_serde_across<Ein, Eout, T>(item: &T)
where
    Ein: serde::ByteSerialize<T>,
    Eout: serde::ByteDeserialize<T>,
    T: std::fmt::Debug + PartialEq<T>,
    <Eout as serde::ByteDeserialize<T>>::ParseErr: std::fmt::Debug,
{
    assert_serde_across_through::<
        Ein,
        Eout,
        T,
        T,
    >(item)
}

pub fn assert_serde_through<E, Tin, Tout>(item: &Tin)
where
    E: serde::ByteSerialize<Tin> + serde::ByteDeserialize<Tout>,
    Tin: ?Sized + std::fmt::Debug + PartialEq<Tout>,
    Tout: std::fmt::Debug,
    <E as serde::ByteDeserialize<Tout>>::ParseErr: std::fmt::Debug,
{
    assert_serde_across_through::<
        E,
        E,
        Tin,
        Tout,
    >(item)
}

pub fn assert_serde<E, T>(item: &T)
where
    E: serde::ByteSerialize<T> + serde::ByteDeserialize<T>,
    T: PartialEq + std::fmt::Debug,
    <E as serde::ByteDeserialize<T>>::ParseErr: std::fmt::Debug,
{
    assert_serde_through::<E,T,T>(item);
}
