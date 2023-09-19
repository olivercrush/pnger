use std::error::Error;
use std::str::from_utf8;

pub struct Png {
    pub bytes: Vec<u8>,
    pub chunks: Vec<Chunk>
}

impl Png {
    pub fn build(bytes: Vec<u8>) -> Result<Png, ()> {
        let mut chunks = Vec::new();
        let mut chunk_cursor : usize = 8;
        let mut length = usize::try_from(Self::as_u32(&bytes[8..12])).unwrap();

        while (chunk_cursor + length + 13) < bytes.len() {
            length = usize::try_from(Self::as_u32(&bytes[chunk_cursor..chunk_cursor + 4])).unwrap();
            let chunk_type = from_utf8(&bytes[chunk_cursor + 4..chunk_cursor + 8]).unwrap().to_string();
            let data_cursor = chunk_cursor + 8;
            let crc = bytes[chunk_cursor + length..chunk_cursor + length + 4].try_into().unwrap();
            chunks.push(Chunk { length, chunk_type, data_cursor, crc });
            chunk_cursor += length + 12;
        }

        Ok(Png { bytes, chunks })
    }

    pub fn extract_size(&self) -> (u32, u32) {
        let ihdr = self.find_first_chunk_of_name("IHDR").unwrap();
        (Self::as_u32(&self.bytes[ihdr.data_cursor..ihdr.data_cursor + 4]), Self::as_u32(&self.bytes[ihdr.data_cursor + 4..ihdr.data_cursor + 8]))
    }

    pub fn extract_idat_data(&self) -> Vec<u8> {
        let mut data: Vec<u8> = Vec::new();

        for chunk in &self.chunks {
            if "IDAT".eq_ignore_ascii_case(&chunk.chunk_type) {
                data.extend_from_slice(self.bytes[chunk.data_cursor..chunk.data_cursor + chunk.length].as_ref());
            }
        }

        data
    }

    pub fn find_first_chunk_of_name(&self, name: &str) -> Result<&Chunk, ()> {
        for chunk in &self.chunks {
            if name.eq_ignore_ascii_case(&chunk.chunk_type) {
                return Ok(chunk);
            }
        }

        return Err(());
    }

    pub fn as_u32(bytes: &[u8]) -> u32 {
        ((bytes[0] as u32) << 24) + ((bytes[1] as u32) << 16) + ((bytes[2] as u32) << 8) + (bytes[3] as u32)
    }
}

pub struct Chunk {
    pub length: usize,
    pub chunk_type: String,
    pub data_cursor: usize,
    pub crc: [u8; 4]
}