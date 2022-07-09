use crate::serde::{ByteSerialize, ByteDeserialize};
use crate::io;

pub enum UVarInt {}

#[derive(Debug,PartialEq,Eq,Clone,Copy,Hash)]
pub struct VarIntTooBig;

impl ByteSerialize<u64> for UVarInt {
    fn byte_serialize<W: io::ByteWrite>(item: &u64, io: &mut W) {
        let mut val:u64 = *item;
        loop {
            let mut byte = (val & 0x7f) as u8;
            if val > 127 { byte |= 0x80 }
            io.write_byte(byte);
            val >>= 7;
            if val == 0 { break; }
        }
    }

    fn size(item: &u64) -> usize {
        // 0.upto(9){ |n| puts "#{2**(7*n)}..=#{2**(7*(n+1))-1} => #{n+1}," }
        // and then some manual editing for 0 and u64::MAX...
        match *item {
            0..=127 => 1,
            128..=16383 => 2,
            16384..=2097151 => 3,
            2097152..=268435455 => 4,
            268435456..=34359738367 => 5,
            34359738368..=4398046511103 => 6,
            4398046511104..=562949953421311 => 7,
            562949953421312..=72057594037927935 => 8,
            72057594037927936..=9223372036854775807 => 9,
            9223372036854775808..=18446744073709551615 => 10,
        }
    }
}

impl ByteDeserialize<u64> for UVarInt {
    type ParseErr = VarIntTooBig;

    fn byte_deserialize<R: io::ByteRead>(io: &mut R) -> Result<u64, Self::ParseErr> {
        let mut val = 0u64;
        let mut cnt = 1;
        loop {
            let byte = io.read_byte();
            if cnt == 10 {
                // 7 bits * 9 bytes = 63 bits
                // So on the tenth byte, we can only "take in" 1 bit of data: 1 or 0
                // And it has to be the last one, so the top bit must be zero
                if byte > 1 {
                    return Err(VarIntTooBig)
                }
            }
            val |= ((byte & 0x7f) as u64) << ((cnt-1)*7);
            if byte <= 127 {
                break;
            }
            cnt += 1;
        }
        Ok(val.into())
    }
}

/// Uses the same encoding as protocol buffers' signed integers
/// https://developers.google.com/protocol-buffers/docs/encoding#signed-ints
pub enum SVarInt {}

// Encoding a signed varint is just taking the number and encoding `(n << 1) ^ (n >> un::BITS-1)`

fn encode_svarint(n: i64) -> u64 {
    ((n << 1) ^ (n >> 63)) as u64
}

fn decode_svarint(n: u64) -> i64 {
    (n >> 1) as i64 ^ if n&1 > 0 { -1 } else { 0 }
}

impl ByteSerialize<i64> for SVarInt {
    fn byte_serialize<W: io::ByteWrite>(item: &i64, io: &mut W) {
        UVarInt::byte_serialize(&encode_svarint(*item), io)
    }

    fn size(item: &i64) -> usize {
        UVarInt::size(&encode_svarint(*item))
    }
}

impl ByteDeserialize<i64> for SVarInt {
    type ParseErr = VarIntTooBig;

    fn byte_deserialize<R: io::ByteRead>(io: &mut R) -> Result<i64, Self::ParseErr> {
        UVarInt::byte_deserialize(io).map(decode_svarint)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn assert_svarint_codes(n: i64) {
        assert_eq!(
            decode_svarint(encode_svarint(n)),
            n
        );
        crate::assert_serde::<SVarInt, _>(&n)
    }

    fn assert_uvarint_serde(n: u64) {
        crate::assert_serde::<UVarInt, u64>(&n);
    }

    #[test]
    fn uvarint_codes() {
        assert_uvarint_serde(0);
        assert_uvarint_serde(1);
        assert_uvarint_serde(69);
        assert_uvarint_serde(127);
        assert_uvarint_serde(128);
        assert_uvarint_serde(420);
        assert_uvarint_serde(16383);
        assert_uvarint_serde(16384);
        assert_uvarint_serde(2097151);
        assert_uvarint_serde(2097152);
        assert_uvarint_serde(268435455);
        assert_uvarint_serde(268435456);
        assert_uvarint_serde(34359738367);
        assert_uvarint_serde(34359738368);
        assert_uvarint_serde(4398046511103);
        assert_uvarint_serde(4398046511104);
        assert_uvarint_serde(562949953421311);
        assert_uvarint_serde(562949953421312);
        assert_uvarint_serde(72057594037927935);
        assert_uvarint_serde(72057594037927936);
        assert_uvarint_serde(9223372036854775807);
        assert_uvarint_serde(9223372036854775808);
        assert_uvarint_serde(18446744073709551614);
        assert_uvarint_serde(18446744073709551615);
    }

    #[test]
    fn protobuf_examples() {
        assert_eq!(encode_svarint(0), 0);
        assert_eq!(encode_svarint(-1), 1);
        assert_eq!(encode_svarint(1), 2);
        assert_eq!(encode_svarint(-2), 3);
        assert_eq!(encode_svarint(2147483647), 4294967294);
        assert_eq!(encode_svarint(-2147483648), 4294967295);
    }

    #[test]
    fn blarg() {
        assert_svarint_codes(0);
        assert_svarint_codes(1);
        assert_svarint_codes(-1);
        assert_svarint_codes(2);
        assert_svarint_codes(-40);
        assert_svarint_codes(126);
        assert_svarint_codes(127);
        assert_svarint_codes(128);
        assert_svarint_codes(-126);
        assert_svarint_codes(-127);
        assert_svarint_codes(-128);
        assert_svarint_codes(256);
        assert_svarint_codes(2147483647);
        assert_svarint_codes(-2147483648);
        assert_svarint_codes(i64::MAX);
    }
}