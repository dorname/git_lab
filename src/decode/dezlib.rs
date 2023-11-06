use std::io::prelude::*;
use std::io;
use flate2::read::ZlibDecoder;

// Uncompresses a Zlib Encoded vector of bytes and returns a string or error
// Here &[u8] implements Read

pub fn decode_reader(bytes: &[u8]) -> io::Result<String> {
    let mut z = ZlibDecoder::new(bytes);
    let mut s = String::new();
    z.read_to_string(&mut s)?;
    Ok(s)
}
pub fn decode_tree(bytes: &[u8]) -> io::Result<Vec<u8>> {
    let mut z = ZlibDecoder::new(bytes);
    let mut vector = Vec::<u8>::new();
    z.read_to_end(&mut vector)?;
    Ok(vector)
}