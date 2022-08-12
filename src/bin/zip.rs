#![feature(generic_const_exprs)]
use twee::{
    magic_bytes::magic_bytes_type,
    serde::{ByteSerialize, ByteDeserialize, self},
    self,
};

// Just making sure this compiles
magic_bytes_type!{
    struct CoolBytes[0x69, 0x04, 0x20];
    pub struct NeatBytes[69, 4, 20];
}

fn main() {
    let mut write_to_me:Vec<u8> = vec![];
    CoolBytes::byte_serialize(&(), &mut write_to_me).unwrap();
    NeatBytes::byte_serialize(&(), &mut write_to_me).unwrap();
    dbg!(write_to_me);
    //twee::assert_serde::<CoolBytes,_>(&());

    // assert_eq!(
    //     <Eout as serde::ByteTypeId<Tout>>::byte_type_id(),
    //     <Ein as serde::ByteTypeId<Tin>>::byte_type_id(),
    // );
    // let est_size = Ein::size(item);
    // let mut buf:Vec<u8> = vec![];
    // Ein::byte_serialize(item, &mut buf).unwrap();
    // let buf_len:u64 = buf.len().try_into().unwrap();
    // assert_eq!(buf_len, est_size);
    // let mut cur = io::ByteCursor::new(buf.as_slice());
    // let other:Tout = Eout::byte_deserialize(&mut cur).unwrap();
    // assert_eq!(
    //     item,
    //     &other,
    // )
    let item = &();
    assert_eq!(
        <CoolBytes as serde::ByteTypeId<()>>::byte_type_id(),
        <CoolBytes as serde::ByteTypeId<()>>::byte_type_id(),
    );
    let est_size = CoolBytes::size(item);
    let mut buf:Vec<u8> = vec![];
    CoolBytes::byte_serialize(item, &mut buf).unwrap();
    let buf_len:u64 = buf.len().try_into().unwrap();
    assert_eq!(buf_len, est_size);
    let mut cur = twee::io::ByteCursor::new(buf.as_slice());
    let other:() = CoolBytes::byte_deserialize(&mut cur).unwrap();
}