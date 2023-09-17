pub fn inflate(bytes: &Vec<u8>) -> Vec<u8> {

}

fn huffman_fixed_decompress(bytes: &Vec<u8>, cursor: &usize) -> (Vec<u8>, usize) {
    // start cursor
    // read header (3 bits)
    // read literals / length-distance code
    // return updated cursor with decompressed data
}