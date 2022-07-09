use super::io;
pub trait ByteDeserialize<T> {
    type ParseErr;

    fn byte_deserialize<R: io::ByteRead>(io: &mut R) -> Result<T, Self::ParseErr>;

    fn guess_size() -> Option<usize> { None }
}

pub trait ByteSerialize<T> {
    fn byte_serialize<W: io::ByteWrite>(item: &T, io: &mut W);

    fn size(item: &T) -> usize {
        let mut io = io::ByteCounter::default();
        Self::byte_serialize(item, &mut io);
        io.count
    }
}

pub trait ByteConstSize<T> {
    const BYTE_SIZE:usize;
}

pub trait ByteConstDeserialize<T> : ByteConstSize<T> {
    type ParseErr;

    fn byte_const_deserialize(io: &[u8; Self::BYTE_SIZE]) -> Result<T, Self::ParseErr>;
}

impl<P, T> ByteDeserialize<T> for P
where P: ByteConstDeserialize<T>, [(); Self::BYTE_SIZE]: {
    type ParseErr = <Self as ByteConstDeserialize<T>>::ParseErr;

    fn byte_deserialize<R: io::ByteRead>(io: &mut R) -> Result<T, Self::ParseErr> {
        let mut buf = [0u8; Self::BYTE_SIZE];
        io.read_buf(&mut buf[..]);
        Self::byte_const_deserialize(&mut buf)
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
    fn byte_serialize<W: io::ByteWrite>(item: &T, io: &mut W) {
        let mut buf = [0u8; Self::BYTE_SIZE];
        Self::byte_const_serialize(item, &mut buf);
        io.write_buf(&buf)
    }

    fn size(_item: &T) -> usize {
        Self::BYTE_SIZE
    }
}