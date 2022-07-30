use std::marker::PhantomData;
use std::mem::MaybeUninit;

use crate::io;
use crate::serde::{ByteTypeId, ByteDeserialize, ByteSerialize};

#[derive(Debug, Default, Copy, Clone)]
pub struct ConstListEncoder<VE>
{
    value_encoder: PhantomData<VE>,
}

#[derive(Debug, Clone, Copy)]
pub struct ParseConstListError<E> {
    pub index: usize,
    pub error: E,
}

impl<VE, V, const N: usize> ByteTypeId<[V; N]> for ConstListEncoder<VE>
where
    VE: ByteTypeId<V>,
    [V; N]: ,
{
    fn byte_type_id() -> Vec<&'static str> {
        let mut res = Vec::new();
        res.push("twee::ConstSizeList<");
        res.extend_from_slice(VE::byte_type_id().as_slice());
        res.push(",");
        res.push(const_stringify_ints::const_str_usize::<N>());
        res.push(">");
        res
    }
}

impl<VE, V, const N: usize> ByteDeserialize<[V; N]> for ConstListEncoder<VE>
where
    VE: ByteDeserialize<V>,
    [V; N]: ,
{
    type ParseErr = ParseConstListError<VE::ParseErr>;

    fn byte_deserialize<R: io::ByteRead>(io: &mut R) -> Result<[V; N], Self::ParseErr> {
        let mut data:[MaybeUninit<V>; N] = MaybeUninit::uninit_array();

        let mut index = 0;
        while index < data.len() {
            let res = VE::byte_deserialize(io).map_err(|error| ParseConstListError{index, error});
            match res {
                Ok(v) => {
                    data[index].write(v);
                },
                Err(e) => {
                    // To avoid a memory leak, we must manually drop any elements we already "filled in"
                    for i in (0..index).into_iter().rev() {
                        unsafe { data[i].assume_init_drop() }
                    }
                    return Err(e)
                },
            }
            index += 1;
        }

        Ok(unsafe { MaybeUninit::array_assume_init(data) })   
    }

    fn guess_size() -> Option<usize> {
        VE::guess_size().map(|el_size| el_size * N)
    }
}

impl<VE, V, const N: usize> ByteSerialize<[V; N]> for ConstListEncoder<VE>
where
    VE: ByteSerialize<V>,
    [V; N]: ,
{
    fn byte_serialize<W: io::ByteWrite>(item: &[V; N], io: &mut W) {
        for i in 0..N {
            VE::byte_serialize(&item[i], io)
        }
    }

    fn size(item: &[V; N]) -> u64 {
        item.iter().map(VE::size).sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn blarg() {
        use crate::endians::NumberEncodingBE as BE;
        let arr:[u64; 4] = [0,1,12345,99];
        crate::assert_serde::<ConstListEncoder<BE>,_>(&arr);
    }
}