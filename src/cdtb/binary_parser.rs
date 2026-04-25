use std::io::{Seek, SeekFrom};

/// Helper class to read from binary data in memory
pub struct BinaryParser {
    data: Vec<u8>,
    pos: usize,
}

impl BinaryParser {
    pub(crate) fn new(data: Vec<u8>) -> BinaryParser {
        BinaryParser { data, pos: 0 }
    }

    /// Moves the cursor to an absolute position
    pub fn seek(&mut self, position: u64) {
        self.pos = position as usize;
    }

    /// Copies N bytes from the current position into a fixed-size array.
    #[inline(always)]
    pub(crate) fn read_bytes<const N: usize>(&mut self) -> [u8; N] {
        // Single bounds check, then a direct copy — no trait dispatch, no loop
        let slice = &self.data[self.pos..self.pos + N];
        self.pos += N;
        slice.try_into().expect("slice length mismatch")
    }

    /// Reads a dynamic number of bytes into a Vec
    pub(crate) fn read(&mut self, length: u32) -> Vec<u8> {
        let length = length as usize;
        let slice = &self.data[self.pos..self.pos + length];
        self.pos += length;
        slice.to_vec()
    }

    /// Reads 1 byte as u8
    pub(crate) fn read_u8(&mut self) -> u8 {
        self.read_bytes::<1>()[0]
    }

    pub(crate) fn read_u8_le(&mut self) -> u8 {
        self.read_u8()
    }

    /// Reads 2 bytes as u16 (little-endian)
    pub(crate) fn read_u16_le(&mut self) -> u16 {
        u16::from_le_bytes(self.read_bytes::<2>())
    }

    /// Reads 4 bytes as u32 (little-endian)
    pub fn read_u32_le(&mut self) -> u32 {
        u32::from_le_bytes(self.read_bytes::<4>())
    }

    /// Reads 8 bytes as u64 (little-endian)
    pub fn read_u64_le(&mut self) -> u64 {
        u64::from_le_bytes(self.read_bytes::<8>())
    }

    /// Reads 1 byte as bool
    pub(crate) fn read_bool(&mut self) -> bool {
        self.read_u8() != 0
    }
}