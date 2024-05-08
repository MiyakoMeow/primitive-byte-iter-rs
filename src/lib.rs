#![no_std]

pub mod impl_be;
pub mod impl_default;
pub mod impl_le;
pub mod impl_ne;

pub struct ByteIter<'a, T: Iterator<Item = &'a u8>> {
    inner: T,
}

impl<'a, T: Iterator<Item = &'a u8>> ByteIter<'a, T> {
    pub fn new(iter: T) -> Self {
        Self { inner: iter }
    }
    pub fn next_u8(&mut self) -> Option<u8> {
        let mut values: [u8; core::mem::size_of::<u8>()] = Default::default();
        for val in values.iter_mut() {
            *val = self.next().copied()?;
        }
        Some(u8::from_be_bytes(values))
    }
}

impl<'a, T: Iterator<Item = &'a u8>> ByteIter<'a, T> {
    pub fn next_i8(&mut self) -> Option<i8> {
        let mut values: [u8; core::mem::size_of::<i8>()] = Default::default();
        for val in values.iter_mut() {
            *val = self.next_u8()?;
        }
        Some(i8::from_be_bytes(values))
    }
    pub fn next_bool(&mut self) -> Option<bool> {
        let mut values: [u8; core::mem::size_of::<bool>()] = Default::default();
        for val in values.iter_mut() {
            *val = self.next_u8()?;
        }
        Some(values[0] != u8::from(false))
    }
}

impl<'a, T: Iterator<Item = &'a u8>> Iterator for ByteIter<'a, T> {
    type Item = &'a u8;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

#[cfg(test)]
mod tests {
    use super::ByteIter;

    #[test]
    fn consume_slice() {
        let slice = [0x45, 0xf6, 0xde, 0x34, 0x98, 0x77];
        let mut reader = ByteIter::new(slice.iter());
        assert_eq!(reader.next_u8(), Some(0x45));
        assert_eq!(reader.next_u32_le(), Some(0x98_34_de_f6));
        assert_eq!(reader.next_u16_ne(), None);
    }
}
