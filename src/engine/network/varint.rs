use std::io::{Read, Write, Result, Error, ErrorKind};

pub const MAX_VARINT_SIZE: usize = 5;

pub fn read_varint<R: Read>(reader: &mut R) -> Result<i32> {
    let mut out: i32 = 0;
    let mut bytes: usize = 0;
    let mut buf = [0u8; 1];

    loop {
        reader.read_exact(&mut buf)?;
        let byte = buf[0];
        out |= ((byte & 0x7F) as i32) << (bytes * 7);
        bytes += 1;
        if bytes > MAX_VARINT_SIZE {
            return Err(Error::new(ErrorKind::InvalidData, "VarInt too big"));
        }
        if (byte & 0x80) == 0 {
            break;
        }
    }
    Ok(out)
}

pub fn write_varint<W: Write>(writer: &mut W, mut value: i32) -> Result<usize> {
    let mut written = 0;
    loop {
        if (value as u32 & !0x7F) == 0 {
            writer.write_all(&[value as u8])?;
            written += 1;
            break;
        } else {
            writer.write_all(&[((value & 0x7F) | 0x80) as u8])?;
            written += 1;
            value = ((value as u32) >> 7) as i32;
        }
    }
    Ok(written)
}

pub fn varint_size(mut value: i32) -> usize {
    let mut size = 0;
    loop {
        size += 1;
        if (value as u32 & !0x7F) == 0 {
            break;
        }
        value = ((value as u32) >> 7) as i32;
    }
    size
}
