use std::io::prelude::*;
use flate2::Compression;
use flate2::write::ZlibEncoder;
pub fn zlib_encode(content_u8: &[u8]) -> Result<Vec<u8>, std::io::Error> {
    let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
    e.write_all(content_u8)?;
    let compressed = e.finish()?;
    Ok(compressed)
}

