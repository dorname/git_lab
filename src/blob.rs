#[test]
fn test() {
    use bstr::ByteSlice;
    use crypto::digest::Digest;
    use crypto::sha1::Sha1;
    use encoding_rs::ISO_8859_10;
    use flate2::read::ZlibDecoder;
    use flate2::write::ZlibEncoder;
    use flate2::Compression;
    use std::fs;
    use std::fs::File;
    use std::io::prelude::*;
    use std::str::{Bytes, FromStr};
    // let text = b"test";
    // let content =
    //     fs::read("src\\testfile\\0a\\d5eae631a7935e42ae9268a6eeeda49df34b01").expect("成功取出");
    // let content_obj = fs::read_to_string("src\\testfile\\demo.js").expect("成功取出");
    // println!("文本内容:{:?}", content);

    // let mut decoder = ZlibDecoder::new(content.as_slice());

    // // 解压缩数据
    // let mut decompressed_data = Vec::new();
    // decoder
    //     .read_to_end(&mut decompressed_data)
    //     .expect("Failed to read from decoder.");

    // println!("Decompressed data: {:?}", decompressed_data);
    // let git_object = format!("blob {}\u{0}{}", content.len(), content);
    // // 计算 SHA-1 哈希值
    // let mut sha = Sha1::new();
    // sha.input(git_object.as_bytes());
    // //30d74d258442c7c65512eafab474568dd706c430
    // //a94a8fe5ccb19ba61c4c0873d391e987982fbbd3
    // let result = sha.result_str();
    // println!("SHA-1 Hash: {}", result);
    // //bd9dbf5aae1a3862dd1526723246b20206e5fc37
    // //bd9dbf5aae1a3862dd1526723246b20206e5fc37
    // 读取经过 zlib 压缩的文件内容
    //提交对象
    let compressed_content = fs::read("src\\testfile\\28\\7991643b4e88317f0f6980828e89e449208f6e")
        .expect("Failed to read file");

    // 创建一个解压缩器
    let mut decoder = ZlibDecoder::new(&compressed_content[..]);

    // 创建一个缓冲区用于存储解压后的数据
    let mut decompressed_content = Vec::new();

    // 解压数据
    decoder
        .read_to_end(&mut decompressed_content)
        .expect("Failed to decompress data");
    println!("{:?}",decompressed_content.len());

    let type_index = decompressed_content.find_byte(0x20).unwrap();
    let type_object = &decompressed_content[0..type_index];

    let size_index = decompressed_content.find_byte(0x00).unwrap();
    let data = decompressed_content[size_index + 1..].to_vec();
    // println!("{:?}",data);
    // println!("type_index:{:?},type_object:{:?},size_index:{:?},data:{:?}",type_index,type_object,size_index,data);
    // // 现在，decompressed_content 包含解压后的数据
    // // 你可以将其转换为字符串或处理它的其他操作
    // let result_str = String::from_utf8_lossy(&decompressed_content);
    // println!("Decompressed data: {}", result_str);
    // 假设这是你的 decompressed_content

    // 创建一个向量，用于存储最终结果
    let mut final_segments: Vec<Vec<Vec<u8>>> = Vec::new();

    // 创建一个起始索引，用于追踪分割位置
    let mut start_index = 0;

    // 遍历 decompressed_content 中的字节
    for (index, &byte) in data.iter().enumerate() {
        if byte == 0x00 {
            // 如果遇到字节值为 0x00，将从 start_index 到当前索引的部分添加到 segments
            let segment = data[start_index..index].to_vec();

            // 创建一个向量，用于存储第二次分割后的结果
            let mut sub_segments: Vec<Vec<u8>> = Vec::new();

            // 创建一个起始索引，用于追踪第二次分割位置
            let mut sub_start_index = 0;

            // 遍历 segment 中的字节
            for (sub_index, &sub_byte) in segment.iter().enumerate() {
                if sub_byte == 0x20 {
                    // 如果遇到字节值为 0x20，将从 sub_start_index 到当前索引的部分添加到 sub_segments
                    sub_segments.push(segment[sub_start_index..sub_index].to_vec());

                    // 更新起始索引，以便下一个分割
                    sub_start_index = sub_index + 1;
                }
            }

            // 处理最后一个分割后的部分
            if sub_start_index < segment.len() {
                sub_segments.push(segment[sub_start_index..].to_vec());
            }

            // 将第二次分割后的结果添加到 final_segments
            final_segments.push(sub_segments);

            // 更新起始索引，以便下一个分割
            start_index = index + 1;
        }
    }
    fn bytes_to_hex_string(bytes: &[u8]) -> String {
        bytes.iter()
            .map(|byte| format!("{:02X}", byte))
            .collect::<Vec<String>>()
            .join("")
    }
    // 打印最终结果
    for (i, segments) in final_segments.iter().enumerate() {
        println!("Segment {}:{:?}", i,segments);
        for (j, sub_segment) in segments.iter().enumerate() {
            // println!(" Sub-segment {}:{:?}:",j,sub_segment);
            // println!("hash{:?}",bytes_to_hex_string(sub_segment));
            // if segments.len()==1 {

            // }
            if sub_segment.len() < 20{
                println!(" filename {:?}", String::from_utf8_lossy(sub_segment));
            }else{
                println!("hash{:?}",bytes_to_hex_string(&sub_segment[..20]));
                println!(" mode {:?}", String::from_utf8_lossy(&sub_segment[20..]));
            }
         

            
        }
    }
}
