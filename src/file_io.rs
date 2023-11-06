use std::fs;
use std::fs::File;
use std::io::prelude::*;
use crate::decode;
use bstr::ByteSlice;
use hex;
use flate2::read::ZlibDecoder;


#[test]
fn test(){
    // let tree_object_file:Vec<u8> = fs::read("/project/git_lab/src/testfile/28/7991643b4e88317f0f6980828e89e449208f6e").expect("文件读取成功");
    let tree_object_file:Vec<u8> = fs::read("src//testfile//28//7991643b4e88317f0f6980828e89e449208f6e").expect("文件读取成功");
    let mut decoder = ZlibDecoder::new(&tree_object_file[..]);
    let mut decompressed_data = Vec::new();
    decoder.read_to_end(&mut decompressed_data).expect("Failed to decompress data");

    // let hex_string = hex::encode(&decompressed_data);
    // println!("{}", hex_string);
    println!("{:?}", String::from_utf8_lossy(&decompressed_data[..]));
}