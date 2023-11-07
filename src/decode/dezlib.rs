use bstr::ByteSlice;
use flate2::read::ZlibDecoder;
use std::io;
use std::io::prelude::*;

// Uncompresses a Zlib Encoded vector of bytes and returns a string or error
// Here &[u8] implements Read

pub fn decode_reader(bytes: &[u8]) -> io::Result<String> {
    let mut z = ZlibDecoder::new(bytes);
    let mut s = String::new();
    z.read_to_string(&mut s)?;
    Ok(s)
}

pub fn decode_tree(bytes: &[u8]) -> io::Result<String> {
    let mut z = ZlibDecoder::new(bytes);
    let mut vector = Vec::<u8>::new();
    z.read_to_end(&mut vector)?;
    let type_data_index = vector.find_byte(0x00).unwrap();

        let mut obj_data_arr: Vec<u8> = vector.drain(type_data_index + 1..).collect();
        let mut type_data = vector.as_bstr().to_string();
        type_data+="\n";
        while let Some(index) = obj_data_arr.find_byte(0x00) {
            type_data+=&format!("子对象：{} ",&obj_data_arr[..index].as_bstr());
            type_data+=&format!("Hash: {}\n",hex::encode(&obj_data_arr[index + 1..index + 21]));
            obj_data_arr.drain(..index + 21);
        }
    Ok(type_data)
}
#[test]
fn blob_test(){
    use std::fs;
    use std::fs::File;
    let tree_object_file: Vec<u8> =
        fs::read("/project/git_lab/src/testfile/30/d74d258442c7c65512eafab474568dd706c430")
            .expect("文件读取成功");
    let content_byte = decode_reader(&tree_object_file[..]).unwrap();
    println!("{}", content_byte);
}

#[test]
fn tree_test() {
    use std::fs;
    use std::fs::File;
    let tree_object_file: Vec<u8> =
        fs::read("/project/git_lab/src/testfile/dc/c20f823c15ba6394596b475c03d08cdc4417a0")
            .expect("文件读取成功");
    let content_byte = decode_tree(&tree_object_file[..]).unwrap();
    println!("{}", content_byte);
}

#[test]
fn commit_test(){
    use std::fs;
    use std::fs::File;
    let tree_object_file: Vec<u8> =
        fs::read("/project/git_lab/src/testfile/ff/11bc76cb7e488b83369a169e255fb4ca2ee328")
            .expect("文件读取成功");
    let content_byte = decode_reader(&tree_object_file[..]).unwrap();
    println!("{}", content_byte);
}
#[test]
fn hex_test() {
    println!(
        "{:?}",
        hex::decode("dcc20f823c15ba6394596b475c03d08cdc4417a0").unwrap()
    );
    println!(
        "{:?}",
        hex::encode([
            220, 194, 15, 130, 60, 21, 186, 99, 148, 89, 107, 71, 92, 3, 208, 140, 220, 68, 23, 160
        ])
    );
    println!("{:?}",hex::encode(b"\xBD\x9D\xBFZ\xAE\x1A8b\xDD\x15&r2F\xB2\x02\x06\xE5\xFC7"));
    if let Ok(bytes) = hex::decode("bd9dbf5aae1a3862dd1526723246b20206e5fc37") {
        println!("{:?}", bytes.iter().map(|b| format!("{:02x}", b)).collect::<String>());
    }
    assert_eq!(hex::decode("bd9dbf5aae1a3862dd1526723246b20206e5fc37").unwrap(), b"\xBD\x9D\xBFZ\xAE\x1A8b\xDD\x15&r2F\xB2\x02\x06\xE5\xFC7");
}



#[test]
fn decode_tree_test() {
    use bstr::ByteSlice;
    use std::fs;
    use std::fs::File;
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
    fn decode_tree_object() {
        let tree_object_file: Vec<u8> =
            fs::read("/project/git_lab/src/testfile/28/7991643b4e88317f0f6980828e89e449208f6e")
                .expect("文件读取成功");

        let mut decoder = ZlibDecoder::new(&tree_object_file[..]);
        let mut tree_object_bytes: Vec<u8> = Vec::new();
        decoder
            .read_to_end(&mut tree_object_bytes)
            .expect("提取成功");
        let type_data_index = tree_object_bytes.find_byte(0x00).unwrap();

        let mut obj_data_arr: Vec<u8> = tree_object_bytes.drain(type_data_index + 1..).collect();
        let type_data = String::from_utf8_lossy(&tree_object_bytes);
        println!("{:?}", type_data.to_string());
        while let Some(index) = obj_data_arr.find_byte(0x00) {
            println!(
                "子对象：{:?}",
                String::from_utf8_lossy(&obj_data_arr[..index])
            );
            println!(
                "Hash：{:?}",
                bytes_to_hex_string(&obj_data_arr[index + 1..index + 21])
            );
            obj_data_arr.drain(..index + 21);
        }
    }
    println!("{:?}", decode_tree_object());
}

#[test]
fn test_hex(){
    use hex::FromHex;
    use rustc_hex::ToHex;
       // 十六进制字符串
       let hex_string = "30d74d258442c7c65512eafab474568dd706c430";

       // 将十六进制字符串转换为字节序列
       let byte_sequence = hex::decode(hex_string).expect("Invalid hex string");
   
       // 输出字节序列
       println!("Byte Sequence: {:?}", byte_sequence.as_bstr().to_os_str().unwrap());
   
       // 原始字符串
       let original_string = String::from("Hello, ");

       // 不可变字节字符串
       let byte_string: &[u8] = b"world";
   
       // 使用 bstr::B 类型拼接字节字符串和字符串
        let result = [bstr::B(original_string.as_bytes()),byte_string].concat();
   
       // 输出拼接后的结果
       println!("{}", result.as_bstr().to_string());

}