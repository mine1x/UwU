use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use super::varint::{varint_size, write_varint, MAX_VARINT_SIZE};
use std::io::{Error, ErrorKind, Result};

pub async fn read_varint_async<R: AsyncRead + Unpin>(reader: &mut R) -> Result<i32> {
    let mut out: i32 = 0;
    let mut bytes: usize = 0;
    let mut buf = [0u8; 1];

    loop {
        reader.read_exact(&mut buf).await?;
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

pub async fn write_packet_frame_async<W: AsyncWrite + Unpin>(
    writer: &mut W,
    packet_id: i32,
    payload: &[u8],
) -> Result<()> {
    let body_len = varint_size(packet_id) + payload.len();
    let mut frame = Vec::with_capacity(varint_size(body_len as i32) + body_len);
    write_varint(&mut frame, body_len as i32)?;
    write_varint(&mut frame, packet_id)?;
    frame.extend_from_slice(payload);
    writer.write_all(&frame).await?;
    writer.flush().await
}

pub async fn read_packet_frame_async<R: AsyncRead + Unpin>(reader: &mut R) -> Result<(i32, Vec<u8>)> {
    let frame_len = read_varint_async(reader).await? as usize;
    if frame_len == 0 {
        return Err(Error::new(ErrorKind::InvalidData, "Frame length cannot be zero"));
    }
    let mut frame_buf = vec![0u8; frame_len];
    reader.read_exact(&mut frame_buf).await?;

    let mut cursor = std::io::Cursor::new(&frame_buf[..]);
    let packet_id = super::varint::read_varint(&mut cursor)?;
    let header_len = cursor.position() as usize;
    let payload = frame_buf[header_len..].to_vec();

    Ok((packet_id, payload))
}
