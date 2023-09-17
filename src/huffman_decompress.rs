pub fn inflate(bytes: &Vec<u8>) -> Vec<u8> {
    Vec::new()
}

fn huffman_fixed_decompress(bytes: &Vec<u8>, cursor: usize) -> (Vec<u8>, usize) {
    // start cursor
    let current_byte = bytes.get(cursor).unwrap();
    let last_block = (current_byte & 0x00000001) == 0;
    let compression_type = (current_byte & 0x00000110) == 0;

    // read header (3 bits)
    // read literals / length-distance code
    // return updated cursor with decompressed data

    (Vec::new(), 0)
}