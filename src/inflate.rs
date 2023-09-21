/* Pass the concatenated IDAT chunks data to inflate algorithm */

pub fn inflate(bytes: &Vec<u8>) -> Vec<u8> {
    Vec::new()
}

// TODO : test this
fn huffman_fixed_decompress(bytes: &Vec<u8>) -> (Vec<u8>, usize) {
    let mut decompressed_bytes : Vec<u8> = Vec::new();

    // init decompress
    let mut byte_cursor = 0;
    let mut bit_cursor = 0;
    let mut final_block = false;

    // process blocks
    while !final_block {
        let mut current_byte = bytes.get(byte_cursor).unwrap().clone();

        // read header (3 bits)
        final_block = process_bits(1, &bytes, &mut bit_cursor, &mut byte_cursor) == 1;
        let compression_type = process_bits(2, &bytes, &mut bit_cursor, &mut byte_cursor);

        // handle decompress type

        // read literals / length-distance code
        loop {
            decompressed_bytes.push(process_bits(8, &bytes, &mut bit_cursor, &mut byte_cursor))
        }

        // return updated cursor with decompressed data
    }

    (Vec::new(), 0)
}

// TODO : test this thing
fn process_bits(processed_bits: usize, bytes: &Vec<u8>, bit_cursor: &mut usize, byte_cursor: &mut usize) -> u8 {
    let mut result = 0;

    // calculate processed_bits then if it's 8 reset and increment byte_cursor
    let mut i = 0;
    while i < processed_bits {
        result += (bytes[byte_cursor] >> (8 - bit_cursor) & 1) << i;

        if (*bit_cursor + 1) >= 8 {
            *bit_cursor = 0;
            *byte_cursor += 1;
        }
        *bit_cursor += 1;
        i += 1;
    }

    result
}