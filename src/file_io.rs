use std::fs;
use std::fs::File;
use std::io::prelude::*;
use crate::decode;
use bstr::ByteSlice;

#[test]
fn test(){
    let tree_object_file:Vec<u8> = fs::read("/project/git_lab/src/testfile/28/7991643b4e88317f0f6980828e89e449208f6e").expect("文件读取成功");
    // let d = String::from_utf8_lossy(&tree_object_file[..]).replace("a", "x07");
    let d = tree_object_file.as_bstr().replace("a", "x07");

    println!("{:?}",decode::decode_reader(d.as_bytes()));
        // let t = b"x\\x9CK\\xCA\\xC9OR04c(\\xCFH,Q\\xC8,V(-\\xD0QH\\xC9O\\xB6\\a\\x00_\\x1C\\a\\x9D";
        // x\x9CK\xCA\xC9OR04c(\xCFH,Q\xC8,V(-\xD0QH\xC9O\xB6\x07\0_\x1c\x07\x9D

    // println!("{:?}",d.as_bytes());
    // let c = vec![120, 92, 120, 57, 67, 75, 92, 120, 67, 65, 92, 120, 67, 57, 79, 82, 48, 52, 99, 40, 92, 120, 67, 70, 72, 44, 81, 92, 120, 67, 56, 44, 86, 40, 45, 92, 120, 68, 48, 81, 72, 92, 120, 67, 57, 79, 92, 120, 66, 54, 92, 97, 92, 120, 48, 48, 95, 92, 120, 49, 67, 92, 97, 92, 120, 57, 68];
    // println!("{}",String::from_utf8_lossy(&tree_object_file[..]).replace('a', "x07"));
    
    // assert_eq!(t.as_bytes(),&tree_object_file[..]);
}