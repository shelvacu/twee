use crate::serde::*;

pub enum NumberEncodingLE {}
pub enum NumberEncodingBE {}

macro_rules! impl_encoding {
    ($($t:ty,)*) => {
        $(
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

impl_encoding! {u8, u16, u32, u64, u128,}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_serde;

    #[test]
    fn zero_one_max() {
        macro_rules! do_tests {
            ($($t:ty,)*) => {
                $(
                    assert_serde::<NumberEncodingLE, $t>(&0);
                    assert_serde::<NumberEncodingLE, $t>(&1);
                    assert_serde::<NumberEncodingLE, $t>(&<$t>::MAX);
                    assert_serde::<NumberEncodingBE, $t>(&0);
                    assert_serde::<NumberEncodingBE, $t>(&1);
                    assert_serde::<NumberEncodingBE, $t>(&<$t>::MAX);
                )*
            }
        }
        do_tests!(u8, u16, u32, u64, u128,);
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