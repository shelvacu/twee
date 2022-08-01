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
    type Err;

    fn write_byte(&mut self, data: u8) -> Result<(), Self::Err>;

    fn write_buf(&mut self, data: &[u8]) -> Result<(), Self::Err> {
        for v in data {
            self.write_byte(*v)?;
        }
        Ok(())
    }
}

pub trait ByteRead {
    type Err;

    fn read_byte(&mut self) -> Result<u8, Self::Err> {
        Ok(self.read_buf(1)?[0])
    }

    fn read_buf<'a>(&'a mut self, len: u64) -> Result<Cow<'a, [u8]>, Self::Err>;
}

impl ByteWrite for Vec<u8> {
    type Err = !;

    fn write_byte(&mut self, data: u8) -> Result<(), !> {
        self.push(data);
        Ok(())
    }

    fn write_buf(&mut self, data: &[u8]) -> Result<(), !> {
        self.extend_from_slice(data);
        Ok(())
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

#[derive(Debug, Copy, Clone, Default)]
pub struct EndOfBufferError;

impl std::fmt::Display for EndOfBufferError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "attempted to read past end of buffer")
    }
}

impl std::error::Error for EndOfBufferError {}

impl<'a> ByteRead for ByteCursor<'a> {
    type Err = EndOfBufferError;

    fn read_buf<'b>(&'b mut self, len: u64) -> Result<Cow<'b, [u8]>, Self::Err> {
        let len_us:usize = len.try_into().unwrap();
        if len_us > (self.inner.len() - self.idx) {
            return Err(EndOfBufferError);
        }
        let res = &self.inner[self.idx .. self.idx + len_us];
        self.idx += len_us;
        Ok(res.into())
    }

    fn read_byte(&mut self) -> Result<u8, Self::Err> {
        let res = self.inner.get(self.idx).ok_or(EndOfBufferError)?;
        self.idx += 1;
        Ok(*res)
    }
}

#[derive(Debug,Copy,Clone,Default)]
pub struct ByteCounter {
    pub count: u64
}

impl ByteWrite for ByteCounter {
    type Err = !;

    fn write_byte(&mut self, _data: u8) -> Result<(), !> {
        self.count += 1;
        Ok(())
    }

    fn write_buf(&mut self, data: &[u8]) -> Result<(), !> {
        let len:u64 = data.len().try_into().unwrap();
        self.count += len;
        Ok(())
    }
}