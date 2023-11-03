use bstr::ByteSlice;
use crypto::digest::Digest;
use crypto::sha1::Sha1;
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::str::{Bytes, FromStr};
use std::io::{Read, Cursor};
use byteorder::{ReadBytesExt, BigEndian};
#[test]
fn test() {
    // 字节值转换成十六进制字符串
    fn bytes_to_hex_string(bytes: &[u8]) -> String {
        bytes
            .iter()
            .map(|byte| format!("{:02x}", byte))
            .collect::<Vec<String>>()
            .join("")
    }
    /**
     * 树对象的解析策略：
     * 
     * 全量数据第一次分割：找到第一个0x00,得到前半部分为树对象的特征信息 tree 185--size;后半部分则为存储数据data
     * 对data进行足步分割：
     * Step1 找到第一个0x00的位置i，往后i+20得到第一个树对象存储的子对象数据的结束索引；
     *       以i+21为起点找第一个0x00的位置j,往后j+20得到第二子对象数据的结束索引...以此类推，直到起始位置n已经超过数据data的长度
     * Step2 解析子对象数据
     *       按照0x00进行分割，前者可之间使用String::from_utf8_lossy,后面使用bytes_to_hex_string计算对应的hash值
     * 
     */
    fn decode_tree_object(){
        let tree_object_file:Vec<u8> = fs::read("src\\testfile\\28\\7991643b4e88317f0f6980828e89e449208f6e").expect("文件读取成功");
        let mut decoder =  ZlibDecoder::new(&tree_object_file[..]);
        let mut tree_object_bytes:Vec<u8> = Vec::new();
        decoder.read_to_end(&mut tree_object_bytes).expect("提取成功");
        let type_data_index = tree_object_bytes.find_byte(0x00).unwrap();
        
      
        let mut obj_data_arr:Vec<u8> = tree_object_bytes.drain(type_data_index+1..).collect();
        let type_data = String::from_utf8_lossy(&tree_object_bytes);
        println!("{:?}",type_data.to_string());
        while let Some(index) = obj_data_arr.find_byte(0x00) {
            println!("子对象：{:?}",String::from_utf8_lossy(&obj_data_arr[..index]));
            println!("Hash：{:?}",bytes_to_hex_string(&obj_data_arr[index+1..index+21]));
            obj_data_arr.drain(..index+21);
        }
    }
    fn encode_tree_object(){
        
    }
    println!("{:?}",decode_tree_object());

}