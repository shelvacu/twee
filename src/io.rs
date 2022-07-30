use std::borrow::Cow;

pub trait SliceExt {
    type Item: Sized;

    fn halfconst_split_at<const N: usize>(&self) -> (&[Self::Item; N], &[Self::Item]);
}

impl<T: Sized> SliceExt for [T] {
    type Item = T;

    fn halfconst_split_at<const N: usize>(&self) -> (&[Self::Item; N], &[Self::Item]) {
        //TODO: make this fast
        let (a, b) = self.split_at(N);
        (a.try_into().unwrap(), b)
    }
}

pub trait ArrayExt {
    type Item: Sized;
    const SIZE: usize;

    fn const_split_at<const N:usize>(&self) -> (&[Self::Item; N], &[Self::Item; Self::SIZE - N]);
}

impl<T: Sized, const SIZE: usize> ArrayExt for [T; SIZE] {
    type Item = T;
    const SIZE:usize = SIZE;

    fn const_split_at<const N:usize>(&self) -> (&[Self::Item; N], &[Self::Item; Self::SIZE - N]) {
        //TODO: make this fast
        let (a,b) = self.as_slice().split_at(N);
        (
            a.try_into().unwrap(),
            b.try_into().unwrap(),
        )
    }
}


pub trait ByteWrite {
    fn write_byte(&mut self, data: u8);

    fn write_buf(&mut self, data: &[u8]) {
        for v in data {
            self.write_byte(*v);
        }
    }
}

pub trait ByteRead {
    fn read_byte(&mut self) -> u8 {
        self.read_buf(1)[0]
    }

    fn read_buf<'a>(&'a mut self, len: u64) -> Cow<'a, [u8]>;
}

impl ByteWrite for Vec<u8> {
    fn write_byte(&mut self, data: u8) {
        self.push(data);
    }

    fn write_buf(&mut self, data: &[u8]) {
        self.extend_from_slice(data);
    }
}

pub struct ByteCursor<'a> {
    idx: usize,
    inner: &'a [u8],
}

impl<'a> ByteCursor<'a> {
    pub fn new(inner: &'a [u8]) -> Self {
        Self{
            idx: 0,
            inner,
        }
    }

    pub fn at_end(&self) -> bool {
        self.idx == self.inner.len()
    }
}

impl<'a> ByteRead for ByteCursor<'a> {
    fn read_buf<'b>(&'b mut self, len: u64) -> Cow<'b, [u8]> {
        let len_us:usize = len.try_into().unwrap();
        let res = &self.inner[self.idx .. self.idx + len_us];
        self.idx += len_us;
        res.into()
    }

    fn read_byte(&mut self) -> u8 {
        let res = self.inner[self.idx];
        self.idx += 1;
        res
    }
}

#[derive(Debug,Copy,Clone,Default)]
pub struct ByteCounter {
    pub count: u64
}

impl ByteWrite for ByteCounter {
    fn write_byte(&mut self, _data: u8) {
        self.count += 1;
    }

    fn write_buf(&mut self, data: &[u8]) {
        let len:u64 = data.len().try_into().unwrap();
        self.count += len;
    }
}