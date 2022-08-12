use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct MagicMismatch<const N:usize> {
    pub expected:[u8; N],
    pub found:[u8; N],
}

impl<const N:usize> fmt::Display for MagicMismatch<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "unexpected magic bytes {:?}, expecting {:?}", self.found, self.expected)
    }
}

impl<const N:usize> std::error::Error for MagicMismatch<N> {}

#[macro_export]
macro_rules! magic_bytes_type {
    (
        $(
            $(#[$struct_meta:meta])*
            $sv:vis struct $name:ident [$($n:literal),*];
        )+
    ) => {
        $(
            #[derive(Debug, Default, Copy, Clone)]
            $(#[$struct_meta])*
            $sv struct $name;

            impl $crate::serde::ByteTypeId<()> for $name {
                fn byte_type_id() -> ::std::vec::Vec<&'static str> {
                    vec![
                        concat!(
                            "MagicBytes<", $( stringify!($n), ",", )* ">"
                        )
                    ]
                }
            }
            
            impl $crate::serde::ByteConstSize<()> for $name {
                const BYTE_SIZE:usize = 0 $( + 1 + ($n - $n) as usize)*;
            }

            impl $crate::serde::ByteConstDeserialize<()> for $name {
                type ParseErr = $crate::magic_bytes::MagicMismatch<{0 $( + 1 + ($n - $n) as usize)*}>;

                fn byte_const_deserialize(io: &[u8; 0 $( + 1 + ($n - $n) as usize)*]) -> ::std::result::Result<(), Self::ParseErr> {
                    if io == &[$($n,)*] {
                        ::std::result::Result::Ok(())
                    } else {
                        ::std::result::Result::Err($crate::magic_bytes::MagicMismatch{
                            expected: [$($n,)*],
                            found: <[u8; 0 $( + 1 + ($n - $n) as usize)*] as ::std::clone::Clone>::clone(io),
                        })
                    }
                }
            }

            impl $crate::serde::ByteConstSerialize<()> for $name {
                fn byte_const_serialize(_item: &(), io: &mut [u8; 0 $( + 1 + ($n - $n) as usize)*]) {
                    *io = [$($n,)*];
                }
            }
        )+
    };
}

// Just making sure this compiles
magic_bytes_type!{
    struct AwesomeBytes[0x69, 0x04, 0x20];
    struct CoolBytes[b'a', b'b', b'C', 0x00];
}

pub use magic_bytes_type;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn blarg() {
        crate::assert_serde::<AwesomeBytes,_>(&());
        crate::assert_serde::<CoolBytes,_>(&());
    }
}