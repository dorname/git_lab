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