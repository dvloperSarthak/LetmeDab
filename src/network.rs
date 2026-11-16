use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::io::{Cursor, Read, Write};
use thiserror::Error;
use uuid::Uuid;

pub const EMOTECRAFT_CHANNEL: &str = "emotecraft:main";
pub const PROTOCOL_VERSION: u16 = 8;

#[derive(Error, Debug)]
pub enum NetworkError {
    #[error("Buffer underflow or EOF")]
    Eof,
    #[error("VarInt is too large")]
    VarIntTooLarge,
    #[error("Invalid UTF-8 string: {0}")]
    InvalidUtf8(#[from] std::string::FromUtf8Error),
    #[error("Compression error: {0}")]
    Compression(#[from] std::io::Error),
    #[error("Unknown packet ID: {0:#x}")]
    UnknownPacket(u8),
    #[error("Protocol error: {0}")]
    Protocol(String),
}

/// Helper methods for Minecraft network buffer encoding and decoding.
pub struct McBuf;

impl McBuf {
    /// Read a Minecraft VarInt (up to 5 bytes).
    pub fn read_var_int<R: Read>(reader: &mut R) -> Result<i32, NetworkError> {
        let mut num_read = 0;
        let mut result = 0;
        loop {
            let byte = reader.read_u8().map_err(|_| NetworkError::Eof)?;
            let value = (byte & 0b0111_1111) as i32;
            result |= value << (7 * num_read);

            num_read += 1;
            if num_read > 5 {
                return Err(NetworkError::VarIntTooLarge);
            }
            if (byte & 0b1000_0000) == 0 {
                break;
            }
        }
        Ok(result)
    }

    /// Write a Minecraft VarInt.
    pub fn write_var_int<W: Write>(writer: &mut W, mut value: i32) -> Result<(), NetworkError> {
        loop {
            let mut byte = (value & 0b0111_1111) as u8;
            value = ((value as u32) >> 7) as i32;
            if value != 0 {
                byte |= 0b1000_0000;
            }
            writer.write_u8(byte).map_err(NetworkError::Compression)?;
            if value == 0 {
                break;
            }
        }
        Ok(())
    }

    /// Read a length-prefixed UTF-8 string.
    pub fn read_string<R: Read>(reader: &mut R) -> Result<String, NetworkError> {
        let len = Self::read_var_int(reader)? as usize;
        let mut buf = vec![0u8; len];
        reader.read_exact(&mut buf).map_err(|_| NetworkError::Eof)?;
        String::from_utf8(buf).map_err(NetworkError::InvalidUtf8)
    }

    /// Write a length-prefixed UTF-8 string.
    pub fn write_string<W: Write>(writer: &mut W, s: &str) -> Result<(), NetworkError> {
        let bytes = s.as_bytes();
        Self::write_var_int(writer, bytes.len() as i32)?;
        writer.write_all(bytes).map_err(NetworkError::Compression)?;
        Ok(())
    }

    /// Read a 128-bit UUID.
    pub fn read_uuid<R: Read>(reader: &mut R) -> Result<Uuid, NetworkError> {
        let mut bytes = [0u8; 16];
        reader.read_exact(&mut bytes).map_err(|_| NetworkError::Eof)?;
        Ok(Uuid::from_bytes(bytes))
    }

    /// Write a 128-bit UUID.
    pub fn write_uuid<W: Write>(writer: &mut W, uuid: &Uuid) -> Result<(), NetworkError> {
        writer.write_all(uuid.as_bytes()).map_err(NetworkError::Compression)?;
        Ok(())
    }
}

/// Packets used in Emotecraft plugin messaging.
#[derive(Debug, Clone, PartialEq)]
pub enum EmotePacket {
    /// Handshake between client and server.
    Handshake {
        protocol_version: u16,
        mod_version: String,
    },
    /// Play an emote on a specific player.
    Play {
        player_uuid: Uuid,
        emote_id: String,
        is_loop: bool,
        start_tick: u32,
        /// Optional raw emote json payload (if synchronized on demand)
        raw_emote_json: Option<String>,
    },
    /// Stop an active emote on a player.
    Stop {
        player_uuid: Uuid,
        fade_ticks: u32,
    },
    /// Server sending list of available emotes.
    SyncList {
        emotes: Vec<EmoteHeader>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct EmoteHeader {
    pub id: String,
    pub name: String,
    pub author: String,
    pub is_loop: bool,
    pub duration_ticks: u32,
}

impl EmotePacket {
    pub const ID_HANDSHAKE: u8 = 0x00;
    pub const ID_PLAY: u8 = 0x01;
    pub const ID_STOP: u8 = 0x02;
    pub const ID_SYNC_LIST: u8 = 0x03;

    /// Serialize packet into raw bytes for custom payload transport.
    pub fn encode(&self) -> Result<Vec<u8>, NetworkError> {
        let mut buf = Vec::new();
        match self {
            EmotePacket::Handshake {
                protocol_version,
                mod_version,
            } => {
                buf.write_u8(Self::ID_HANDSHAKE).map_err(NetworkError::Compression)?;
                buf.write_u16::<BigEndian>(*protocol_version).map_err(NetworkError::Compression)?;
                McBuf::write_string(&mut buf, mod_version)?;
            }
            EmotePacket::Play {
                player_uuid,
                emote_id,
                is_loop,
                start_tick,
                raw_emote_json,
            } => {
                buf.write_u8(Self::ID_PLAY).map_err(NetworkError::Compression)?;
                McBuf::write_uuid(&mut buf, player_uuid)?;
                McBuf::write_string(&mut buf, emote_id)?;
                buf.write_u8(if *is_loop { 1 } else { 0 }).map_err(NetworkError::Compression)?;
                McBuf::write_var_int(&mut buf, *start_tick as i32)?;

                if let Some(json) = raw_emote_json {
                    buf.write_u8(1).map_err(NetworkError::Compression)?;
                    // Gzip compress large emote json payload to minimize network bandwidth
                    let mut encoder = GzEncoder::new(Vec::new(), Compression::fast());
                    encoder.write_all(json.as_bytes()).map_err(NetworkError::Compression)?;
                    let compressed = encoder.finish().map_err(NetworkError::Compression)?;
                    McBuf::write_var_int(&mut buf, compressed.len() as i32)?;
                    buf.write_all(&compressed).map_err(NetworkError::Compression)?;
                } else {
                    buf.write_u8(0).map_err(NetworkError::Compression)?;
                }
            }
            EmotePacket::Stop {
                player_uuid,
                fade_ticks,
            } => {
                buf.write_u8(Self::ID_STOP).map_err(NetworkError::Compression)?;
                McBuf::write_uuid(&mut buf, player_uuid)?;
                McBuf::write_var_int(&mut buf, *fade_ticks as i32)?;
            }
            EmotePacket::SyncList { emotes } => {
                buf.write_u8(Self::ID_SYNC_LIST).map_err(NetworkError::Compression)?;
                McBuf::write_var_int(&mut buf, emotes.len() as i32)?;
                for e in emotes {
                    McBuf::write_string(&mut buf, &e.id)?;
                    McBuf::write_string(&mut buf, &e.name)?;
                    McBuf::write_string(&mut buf, &e.author)?;
                    buf.write_u8(if e.is_loop { 1 } else { 0 }).map_err(NetworkError::Compression)?;
                    McBuf::write_var_int(&mut buf, e.duration_ticks as i32)?;
                }
            }
        }
        Ok(buf)
    }

    /// Deserialize a packet from raw custom payload bytes.
    pub fn decode(bytes: &[u8]) -> Result<Self, NetworkError> {
        let mut cursor = Cursor::new(bytes);
        let packet_id = cursor.read_u8().map_err(|_| NetworkError::Eof)?;

        match packet_id {
            Self::ID_HANDSHAKE => {
                let protocol_version = cursor.read_u16::<BigEndian>().map_err(|_| NetworkError::Eof)?;
                let mod_version = McBuf::read_string(&mut cursor)?;
                Ok(EmotePacket::Handshake {
                    protocol_version,
                    mod_version,
                })
            }
            Self::ID_PLAY => {
                let player_uuid = McBuf::read_uuid(&mut cursor)?;
                let emote_id = McBuf::read_string(&mut cursor)?;
                let is_loop = cursor.read_u8().map_err(|_| NetworkError::Eof)? != 0;
                let start_tick = McBuf::read_var_int(&mut cursor)? as u32;

                let has_raw = cursor.read_u8().map_err(|_| NetworkError::Eof)? != 0;
                let raw_emote_json = if has_raw {
                    let comp_len = McBuf::read_var_int(&mut cursor)? as usize;
                    let mut comp_data = vec![0u8; comp_len];
                    cursor.read_exact(&mut comp_data).map_err(|_| NetworkError::Eof)?;
                    let mut decoder = GzDecoder::new(&comp_data[..]);
                    let mut decompressed = String::new();
                    decoder.read_to_string(&mut decompressed).map_err(NetworkError::Compression)?;
                    Some(decompressed)
                } else {
                    None
                };

                Ok(EmotePacket::Play {
                    player_uuid,
                    emote_id,
                    is_loop,
                    start_tick,
                    raw_emote_json,
                })
            }
            Self::ID_STOP => {
                let player_uuid = McBuf::read_uuid(&mut cursor)?;
                let fade_ticks = McBuf::read_var_int(&mut cursor)? as u32;
                Ok(EmotePacket::Stop {
                    player_uuid,
                    fade_ticks,
                })
            }
            Self::ID_SYNC_LIST => {
                let count = McBuf::read_var_int(&mut cursor)? as usize;
                let mut emotes = Vec::with_capacity(count);
                for _ in 0..count {
                    let id = McBuf::read_string(&mut cursor)?;
                    let name = McBuf::read_string(&mut cursor)?;
                    let author = McBuf::read_string(&mut cursor)?;
                    let is_loop = cursor.read_u8().map_err(|_| NetworkError::Eof)? != 0;
                    let duration_ticks = McBuf::read_var_int(&mut cursor)? as u32;
                    emotes.push(EmoteHeader {
                        id,
                        name,
                        author,
                        is_loop,
                        duration_ticks,
                    });
                }
                Ok(EmotePacket::SyncList { emotes })
            }
            other => Err(NetworkError::UnknownPacket(other)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_varint_roundtrip() {
        let values = [0, 1, 127, 128, 255, 256, 65535, 1000000, -1, -128];
        for v in values {
            let mut buf = Vec::new();
            McBuf::write_var_int(&mut buf, v).unwrap();
            let mut cursor = Cursor::new(&buf);
            let read = McBuf::read_var_int(&mut cursor).unwrap();
            assert_eq!(v, read);
        }
    }

    #[test]
    fn test_play_packet_roundtrip() {
        let uuid = Uuid::new_v4();
        let packet = EmotePacket::Play {
            player_uuid: uuid,
            emote_id: "dab".to_string(),
            is_loop: false,
            start_tick: 0,
            raw_emote_json: Some("{\"test\": 123}".to_string()),
        };

        let encoded = packet.encode().unwrap();
        let decoded = EmotePacket::decode(&encoded).unwrap();
        assert_eq!(packet, decoded);
    }

    #[test]
    fn test_stop_packet_roundtrip() {
        let uuid = Uuid::new_v4();
        let packet = EmotePacket::Stop {
            player_uuid: uuid,
            fade_ticks: 5,
        };

        let encoded = packet.encode().unwrap();
        let decoded = EmotePacket::decode(&encoded).unwrap();
        assert_eq!(packet, decoded);
    }
}
