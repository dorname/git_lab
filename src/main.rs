use std::vec;
use bstr::ByteSlice;
mod blob;
mod data;
mod encode;
mod obj_type;
mod decode;
mod file_io;
fn main() {
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
    fn bytes_to_hex_string(bytes: &[u8]) -> String {
        bytes
            .iter()
            .map(|byte| format!("{:02x}", byte))
            .collect::<Vec<String>>()
            .join("")
    }
    let a: Vec<u8> = vec![
        52, 48, 48, 48, 48, 32, 98, 97, 107, 0, 230, 222, 130, 57, 119, 44, 44, 99, 88, 93, 209,
        27, 40, 48, 80, 99, 27, 127, 56, 1,
    ];
    let b: Vec<u8> = vec![
        49, 48, 48, 54, 52, 52, 32, 103, 105, 116, 95, 116, 101, 115, 116, 46, 116, 120, 116, 0,
        55, 41, 145, 114, 142, 51, 72, 178, 195, 43, 141, 213, 55, 120, 133, 242, 247, 113, 115,
        255,
    ];
    let c: Vec<u8> = vec![
        49, 48, 48, 54, 52, 52, 32, 103, 105, 116, 95, 116, 101, 115, 116, 46, 116, 120, 116, 0,
        55, 41, 145, 114, 142, 51, 72, 178, 195, 43, 141, 213, 55, 120, 133, 242, 247, 113, 115,
        255,
    ];
    let d: Vec<u8> = vec![
        49, 48, 48, 54, 52, 52, 32, 110, 101, 119, 46, 116, 120, 116, 0, 39, 119, 121, 29, 109,
        231, 170, 192, 63, 56, 177, 184, 64, 58, 208, 250, 232, 46, 40, 172,
    ];
    let e: Vec<u8> = vec![
        52, 48, 48, 48, 48, 32, 116, 114, 101, 101, 84, 101, 115, 116, 0, 129, 58, 109, 94, 77, 72,
        129, 92, 139, 243, 79, 1, 252, 84, 14, 67, 234, 44, 219, 9,
    ];
    println!(
        "a:{},b:{},c:{},d:{},e:{}",
        a.len(),
        b.len(),
        c.len(),
        d.len(),
        e.len()
    );
    fn getSlipt(arr:&Vec<u8>) -> usize{
        let split_index = arr.find_byte(0x00).unwrap();
        split_index
    }
    let s_a = getSlipt(&a);
    println!(
        "Hash:{:?},left:{:?}",
        bytes_to_hex_string(&a[s_a+1..]),
        String::from_utf8_lossy(&a[..s_a])
    );

    let s_b = getSlipt(&b);
    println!(
        "Hash:{:?},left:{:?}",
        bytes_to_hex_string(&b[s_b+1..]),
        String::from_utf8_lossy(&b[..s_b])
    );

    let s_c = getSlipt(&c);
    println!(
        "Hash:{:?},left:{:?}",
        bytes_to_hex_string(&c[s_c+1..]),
        String::from_utf8_lossy(&c[..s_c])
    );

    let s_d = getSlipt(&d);
    println!(
        "Hash:{:?},left:{:?}",
        bytes_to_hex_string(&d[s_d+1..]),
        String::from_utf8_lossy(&d[..s_d])
    );

    let s_e = getSlipt(&e);

    println!(
        "Hash:{:?},left:{:?}",
        bytes_to_hex_string(&e[s_e+1..]),
        String::from_utf8_lossy(&e[..s_e])
    );
    let head:Vec<u8>= vec![116, 114, 101, 101, 32, 49, 56, 53, 0];
    println!("{:?}",String::from_utf8_lossy(&head));
}
