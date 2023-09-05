use std::error::Error;

pub struct Png {
    pub chunks: Vec<Chunk>
}

impl Png {
    pub fn build(bytes: &Vec<u8>) -> Result<Png, ()> {
        let mut chunks = Vec::new();
        let mut chunk_cursor : usize = 8;
        let mut length = usize::try_from(Self::as_u32(&bytes[8..12])).unwrap();

        while (chunk_cursor + length + 13) < bytes.len() {
            length = usize::try_from(Self::as_u32(&bytes[chunk_cursor..chunk_cursor + 4])).unwrap();
            let chunk_type = bytes[chunk_cursor + 4..chunk_cursor + 8].try_into().unwrap();
            let chunk_data = bytes[chunk_cursor + 8..chunk_cursor + 8 + length].to_vec();
            let crc = bytes[chunk_cursor + length..chunk_cursor + length + 4].try_into().unwrap();
            chunks.push(Chunk { length, chunk_type, chunk_data, crc });
            chunk_cursor += length + 12;
        }

        Ok(Png { chunks })
    }

    pub fn as_u32(bytes: &[u8]) -> u32 {
        ((bytes[0] as u32) << 24) + ((bytes[1] as u32) << 16) + ((bytes[2] as u32) << 8) + (bytes[3] as u32)
    }
}

pub struct Chunk {
    pub length: usize,
    pub chunk_type: [u8; 4],
    pub chunk_data: Vec<u8>,
    pub crc: [u8; 4]
}