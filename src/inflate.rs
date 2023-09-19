/* Pass the concatenated IDAT chunks data to inflate algorithm */

pub fn inflate(bytes: &Vec<u8>) -> Vec<u8> {
    Vec::new()
}

fn huffman_fixed_decompress(bytes: &Vec<u8>) -> (Vec<u8>, usize) {
    // init decompress
    let mut cursor = 0;
    let mut current_byte = bytes.get(cursor).unwrap().clone();
    let mut remaining_bits = 8;

    // read header (3 bits)
    let last_block = (current_byte >> remaining_bits - 1 & 1) == 1;
    remaining_bits -= 1;

    current_byte = current_byte >> 7;
    let compression_type = (current_byte >> 5 & 3);
    remaining_bits -= 2;

    // read literals / length-distance code
    // return updated cursor with decompressed data

    (Vec::new(), 0)
}