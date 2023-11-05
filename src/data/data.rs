//山楂树下
use crate::obj_type;
use std::{fmt, result};
use bstr::ByteSlice;
use sha1::{Sha1, Digest};
use crate::encode;
use crate::decode;

struct Data {
    content: String,
    code: String,
    data_type: String,
}
impl Data {
    fn new(content: String, code: String, data_type: String) -> Self {
        Data {
            content: content,
            code: code,
            data_type: data_type,
        }
    }
    fn get_data(&self) -> String {
        format!( "{} {}\x00{}",
        self.data_type,
        self.content.len(),
        self.content)
    }
    fn get_sha_1(&self)->String{
        let mut hasher = Sha1::new();
        hasher.update(self.get_data().as_bytes());
        let result = hasher.finalize();
        format!("{:x}", result)
    }
}
impl fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {}\x00{}",
            self.data_type,
            self.content.len(),
            self.content
        )
    }
}
#[test]
fn test() {
    let content: String = String::from("what is up, doc?");
    // let content: String = String::from("test");
    let code: String = String::from("100644");
    let data_type: String = obj_type::BLOB.to_string();
    let blob =  Data::new(content, code, data_type);
    println!("{}", blob);
    println!("{}",encode::get_sha_1(blob.get_data()));
    //rust中\a是无效转义字符 其对应的响铃字符为\x07,在\a在ruby中是合法的转义字符
    let expected = b"x\x9CK\xCA\xC9OR04c(\xCFH,Q\xC8,V(-\xD0QH\xC9O\xB6\x07\0_\x1C\x07\x9D".as_bstr();
    let binding = encode::zlib_encode(blob.get_data().as_bytes()).unwrap();
    let result = binding.as_bstr();
    println!("{:?}",encode::zlib_encode(blob.get_data().as_bytes()).unwrap().as_bstr());
    assert_eq!(expected, result);
    assert_eq!("blob 16\0what is up, doc?",decode::decode_reader(b"x\x9CK\xCA\xC9OR04c(\xCFH,Q\xC8,V(-\xD0QH\xC9O\xB6\x07\0_\x1c\x07\x9D".as_bytes()).unwrap());
}
