use std::io::{Cursor, Read, Result};
use super::block_pos_codec::{pack_block_pos, unpack_block_pos};
use super::varint::{read_varint, write_varint};

pub struct FriendlyByteBuf {
    pub data: Vec<u8>,
}

impl FriendlyByteBuf {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn from_slice(slice: &[u8]) -> Self {
        Self { data: slice.to_vec() }
    }

    pub fn write_varint(&mut self, val: i32) {
        let _ = write_varint(&mut self.data, val);
    }

    pub fn write_utf(&mut self, s: &str) {
        let bytes = s.as_bytes();
        self.write_varint(bytes.len() as i32);
        self.data.extend_from_slice(bytes);
    }

    pub fn write_block_pos(&mut self, x: i32, y: i32, z: i32) {
        let packed = pack_block_pos(x, y, z);
        self.data.extend_from_slice(&packed.to_be_bytes());
    }

    pub fn write_double(&mut self, val: f64) {
        self.data.extend_from_slice(&val.to_be_bytes());
    }

    pub fn write_float(&mut self, val: f32) {
        self.data.extend_from_slice(&val.to_be_bytes());
    }

    pub fn write_byte(&mut self, val: u8) {
        self.data.push(val);
    }

    pub fn write_bool(&mut self, val: bool) {
        self.data.push(if val { 1 } else { 0 });
    }
}

pub struct FriendlyByteBufReader<'a> {
    cursor: Cursor<&'a [u8]>,
}

impl<'a> FriendlyByteBufReader<'a> {
    pub fn new(slice: &'a [u8]) -> Self {
        Self { cursor: Cursor::new(slice) }
    }

    pub fn read_varint(&mut self) -> Result<i32> {
        read_varint(&mut self.cursor)
    }

    pub fn read_utf(&mut self) -> Result<String> {
        let len = self.read_varint()? as usize;
        let mut buf = vec![0u8; len];
        self.cursor.read_exact(&mut buf)?;
        String::from_utf8(buf).map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
    }

    pub fn read_block_pos(&mut self) -> Result<(i32, i32, i32)> {
        let mut b = [0u8; 8];
        self.cursor.read_exact(&mut b)?;
        Ok(unpack_block_pos(u64::from_be_bytes(b)))
    }

    pub fn read_double(&mut self) -> Result<f64> {
        let mut b = [0u8; 8];
        self.cursor.read_exact(&mut b)?;
        Ok(f64::from_be_bytes(b))
    }

    pub fn read_float(&mut self) -> Result<f32> {
        let mut b = [0u8; 4];
        self.cursor.read_exact(&mut b)?;
        Ok(f32::from_be_bytes(b))
    }

    pub fn read_byte(&mut self) -> Result<u8> {
        let mut b = [0u8; 1];
        self.cursor.read_exact(&mut b)?;
        Ok(b[0])
    }

    pub fn read_bool(&mut self) -> Result<bool> {
        Ok(self.read_byte()? != 0)
    }
}
