use crate::serde::*;

#[derive(Debug, Default, Copy, Clone)]
pub struct NumberEncodingSingleByte;

impl ByteTypeId<u8> for NumberEncodingSingleByte {
    fn byte_type_id() -> Vec<&'static str> {
        vec!["twee::SingleByteUnsigned"]
    }
}

impl ByteConstSize<u8> for NumberEncodingSingleByte {
    const BYTE_SIZE:usize = 1;
}

impl ByteConstDeserialize<u8> for NumberEncodingSingleByte {
    type ParseErr = !;

    fn byte_const_deserialize(io: &[u8; 1]) -> Result<u8, Self::ParseErr> {
        Ok(io[0])
    }
}

impl ByteConstSerialize<u8> for NumberEncodingSingleByte {
    fn byte_const_serialize(item: &u8, io: &mut [u8; 1]) {
        io[0] = *item
    }
}

impl ByteTypeId<i8> for NumberEncodingSingleByte {
    fn byte_type_id() -> Vec<&'static str> {
        vec!["twee::SingleByteSigned"]
    }
}

impl ByteConstSize<i8> for NumberEncodingSingleByte {
    const BYTE_SIZE:usize = 1;
}

impl ByteConstDeserialize<i8> for NumberEncodingSingleByte {
    type ParseErr = !;

    fn byte_const_deserialize(io: &[u8; 1]) -> Result<i8, Self::ParseErr> {
        Ok(io[0] as i8)
    }
}

impl ByteConstSerialize<i8> for NumberEncodingSingleByte {
    fn byte_const_serialize(item: &i8, io: &mut [u8; 1]) {
        io[0] = *item as u8
    }
}

#[derive(Debug, Default, Copy, Clone)]
pub struct NumberEncodingLE;
#[derive(Debug, Default, Copy, Clone)]
pub struct NumberEncodingBE;

macro_rules! impl_single_byte_encoding {
    ($($t:ty,)+) => {
        $(
            impl ByteTypeId<$t> for NumberEncodingLE {
                fn byte_type_id() -> Vec<&'static str> {
                    <NumberEncodingSingleByte as ByteTypeId<$t>>::byte_type_id()
                }
            }

            impl ByteConstSize<$t> for NumberEncodingLE {
                const BYTE_SIZE:usize = 1;
            }

            impl ByteConstDeserialize<$t> for NumberEncodingLE {
                type ParseErr = !;
            
                fn byte_const_deserialize(io: &[u8; 1]) -> Result<$t, Self::ParseErr> {
                    NumberEncodingSingleByte::byte_const_deserialize(io)
                }
            }
            
            impl ByteConstSerialize<$t> for NumberEncodingLE {
                fn byte_const_serialize(item: &$t, io: &mut [u8; 1]) {
                    NumberEncodingSingleByte::byte_const_serialize(item, io)
                }
            }

            
            impl ByteTypeId<$t> for NumberEncodingBE {
                fn byte_type_id() -> Vec<&'static str> {
                    <NumberEncodingSingleByte as ByteTypeId<$t>>::byte_type_id()
                }
            }

            impl ByteConstSize<$t> for NumberEncodingBE {
                const BYTE_SIZE:usize = 1;
            }

            impl ByteConstDeserialize<$t> for NumberEncodingBE {
                type ParseErr = !;
            
                fn byte_const_deserialize(io: &[u8; 1]) -> Result<$t, Self::ParseErr> {
                    NumberEncodingSingleByte::byte_const_deserialize(io)
                }
            }
            
            impl ByteConstSerialize<$t> for NumberEncodingBE {
                fn byte_const_serialize(item: &$t, io: &mut [u8; 1]) {
                    NumberEncodingSingleByte::byte_const_serialize(item, io)
                }
            }
        )+
    }
}

impl_single_byte_encoding! {u8, i8,}

macro_rules! impl_encoding {
    ($($t:ty,)*) => {
        $(
            impl ByteTypeId<$t> for NumberEncodingLE {
                fn byte_type_id() -> Vec<&'static str> {
                    vec![concat!("twee::LE<", stringify!($t), ">")]
                }
            }

            impl ByteConstSize<$t> for NumberEncodingLE {
                const BYTE_SIZE:usize = (<$t>::BITS/8) as usize;
            }

            impl ByteConstDeserialize<$t> for NumberEncodingLE {
                type ParseErr = !;

                fn byte_const_deserialize(io: &[u8; (<$t>::BITS/8) as usize]) -> Result<$t, Self::ParseErr> {
                    Ok(<$t>::from_le_bytes(*io))
                }
            }

            impl ByteConstSerialize<$t> for NumberEncodingLE {
                fn byte_const_serialize(item: &$t, io: &mut [u8; (<$t>::BITS/8) as usize]) {
                    *io = item.to_le_bytes()
                }
            }

            impl ByteTypeId<$t> for NumberEncodingBE {
                fn byte_type_id() -> Vec<&'static str> {
                    vec![concat!("twee::BE<", stringify!($t), ">")]
                }
            }

            impl ByteConstSize<$t> for NumberEncodingBE {
                const BYTE_SIZE:usize = (<$t>::BITS/8) as usize;
            }

            impl ByteConstDeserialize<$t> for NumberEncodingBE {
                type ParseErr = !;

                fn byte_const_deserialize(io: &[u8; (<$t>::BITS/8) as usize]) -> Result<$t, Self::ParseErr> {
                    Ok(<$t>::from_be_bytes(*io))
                }
            }

            impl ByteConstSerialize<$t> for NumberEncodingBE {
                fn byte_const_serialize(item: &$t, io: &mut [u8; (<$t>::BITS/8) as usize]) {
                    *io = item.to_be_bytes()
                }
            }
        )*
    };
}

impl_encoding! {
    u16, u32, u64, u128,
    i16, i32, i64, i128,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_serde;

    #[test]
    fn single_byteness() {
        use crate::assert_serde_across;
        for v in [0, 1, 127, 128, 255] {
            assert_serde_across::<
                NumberEncodingSingleByte,
                NumberEncodingLE,
                u8,
            >(&v);
            assert_serde_across::<
                NumberEncodingSingleByte,
                NumberEncodingBE,
                u8,
            >(&v);
            assert_serde_across::<
                NumberEncodingBE,
                NumberEncodingLE,
                u8,
            >(&v);
        }
        for v in [-128, 0, 1, 127] {
            assert_serde_across::<
                NumberEncodingSingleByte,
                NumberEncodingLE,
                i8,
            >(&v);
            assert_serde_across::<
                NumberEncodingSingleByte,
                NumberEncodingBE,
                i8,
            >(&v);
            assert_serde_across::<
                NumberEncodingBE,
                NumberEncodingLE,
                i8,
            >(&v);
        }
    }

    #[test]
    fn zero_one_max() {
        macro_rules! do_tests {
            ($($t:ty,)*) => {
                $(
                    assert_serde::<NumberEncodingLE, $t>(&<$t>::MIN);
                    assert_serde::<NumberEncodingLE, $t>(&0);
                    assert_serde::<NumberEncodingLE, $t>(&1);
                    assert_serde::<NumberEncodingLE, $t>(&<$t>::MAX);

                    assert_serde::<NumberEncodingBE, $t>(&<$t>::MIN);
                    assert_serde::<NumberEncodingBE, $t>(&0);
                    assert_serde::<NumberEncodingBE, $t>(&1);
                    assert_serde::<NumberEncodingBE, $t>(&<$t>::MAX);
                )*
            }
        }
        do_tests!(
            u8, u16, u32, u64, u128,
            i8, i16, i32, i64, i128,
        );
    }

    #[test]
    fn endianness() {
        use crate::serde::{ByteConstSerialize, ByteConstDeserialize};

        let n = 1u32;
        let expected_le = [1, 0, 0, 0];
        let expected_be = [0, 0, 0, 1];

        let mut actual_le = [0; 4];
        NumberEncodingLE::byte_const_serialize(&n, &mut actual_le);
        assert_eq!(
            expected_le,
            actual_le,
        );

        let mut actual_be = [0; 4];
        NumberEncodingBE::byte_const_serialize(&n, &mut actual_be);
        assert_eq!(
            expected_be,
            actual_be,
        );

        let n_le:u32 = NumberEncodingLE::byte_const_deserialize(&expected_le).unwrap();
        assert_eq!(
            n_le,
            n
        );
        let n_be:u32 = NumberEncodingBE::byte_const_deserialize(&expected_be).unwrap();
        assert_eq!(
            n_be,
            n
        );
    }
}