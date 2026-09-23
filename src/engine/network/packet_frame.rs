// Minecraft Java Varint21 Packet Framing
// Reference: net.minecraft.network.Varint21LengthFieldPrepender / Varint21FrameDecoder

use std::io::{Read, Result, Write};
use super::varint::{read_varint, write_varint, varint_size};

pub fn write_packet_frame<W: Write>(writer: &mut W, packet_id: i32, payload: &[u8]) -> Result<()> {
    let body_len = varint_size(packet_id) + payload.len();
    let mut frame = Vec::with_capacity(varint_size(body_len as i32) + body_len);
    write_varint(&mut frame, body_len as i32)?;
    write_varint(&mut frame, packet_id)?;
    frame.extend_from_slice(payload);
    writer.write_all(&frame)?;
    writer.flush()
}

pub fn read_packet_frame<R: Read>(reader: &mut R) -> Result<(i32, Vec<u8>)> {
    let frame_len = read_varint(reader)? as usize;
    if frame_len == 0 {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Frame length cannot be zero"));
    }
    let mut frame_buf = vec![0u8; frame_len];
    reader.read_exact(&mut frame_buf)?;

    let mut cursor = std::io::Cursor::new(&frame_buf[..]);
    let packet_id = read_varint(&mut cursor)?;
    let header_len = cursor.position() as usize;
    let payload = frame_buf[header_len..].to_vec();

    Ok((packet_id, payload))
}
