/* Pass the concatenated IDAT chunks data to inflate algorithm */

pub fn inflate(bytes: &Vec<u8>) -> Vec<u8> {
    Vec::new()
}

fn huffman_fixed_decompress(bytes: &Vec<u8>) -> (Vec<u8>, usize) {
    // init decompress
    let mut byte_cursor = 0;
    let mut bit_cursor = 0;
    let mut final_block = false;

    // process blocks
    while !final_block {
        let mut current_byte = bytes.get(byte_cursor).unwrap().clone();

        // read header (3 bits)
        calculate_bits_processed(1, &mut bit_cursor, &mut byte_cursor);
        final_block = (current_byte >> (8 - bit_cursor) & 1) == 1;

        calculate_bits_processed(1, &mut bit_cursor, &mut byte_cursor);
        let mut compression_type = (current_byte >> (8 - bit_cursor) & 1);
        calculate_bits_processed(1, &mut bit_cursor, &mut byte_cursor);
        compression_type = compression_type + (current_byte >> (8 - bit_cursor) & 1);

        // read literals / length-distance code

        // return updated cursor with decompressed data
    }

    (Vec::new(), 0)
}

fn calculate_bits_processed(processed_bits: usize, bit_cursor: &mut usize, byte_cursor: &mut usize) {
    // calculate processed_bits (should be always 1 ?), then if it's 8 reset and increment byte_cursor
}