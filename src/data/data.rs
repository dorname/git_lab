use crate::data;
//山楂树下
use crate::decode;
use crate::encode;
use crate::obj_type;
use anyhow::Error;
use bstr::ByteSlice;
use sha1::{Digest, Sha1};
use std::{fmt, result};
use rand::{Rng, distributions::Alphanumeric};

struct Data {
    content: String,
    code: String,
    data_type: String,
    nodes:Option<Vec<Data>>,
    file_name:String
}
impl Data {
    fn new(content: String, code: String, data_type: String) -> Self {
        Data {
            content: content,
            code: code,
            data_type: data_type,
            nodes:None,
            file_name:rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(10)
            .map(char::from)
            .collect()
        }
    }
    fn new_tree(code: String, data_type: String,nodes:Vec<Data>)->(){
        let mut content = String::from("");
        for node in nodes{

            content+=&format!("{} {}\x00{}",&node.data_type.clone(),&node.file_name,hex::decode(&node.get_sha_1()).unwrap());
        }
        println!("{:?}",format!("tree {}\x00{}",content.len(),content));
        // Data {
        //     content: content,
        //     code: code,
        //     data_type: data_type,
        //     nodes:Some(nodes)
        // }
    }
    fn get_data(&self) -> String {
        format!(
            "{} {}\x00{}",
            self.data_type,
            self.content.len(),
            self.content
        )
    }
    fn get_sha_1(&self) -> String {
        let mut hasher = Sha1::new();
        hasher.update(self.get_data().as_bytes());
        let result = hasher.finalize();
        format!("{:x}", result)
    }
    fn add_node(&mut self,subnode:Data){
        if let Some(x) = &mut self.nodes{
            x.push(subnode);
        }else{
            self.nodes = Some(vec![subnode]);
        }
    }
    // 树对象的文件内容
    //#{content}
    //mode 40000代表树对象 100644代表blob对象
    // #{mode} #{dir1_name or file1_name}#{dir1 or file1's hash}#{mode} #{dir2_name or file2_name}#{dir2 or file2's hash}...
    // **注意：**`#{dir1 or file1's hash}`需要从`HEX`转换成字节序再作字符串拼接。
    fn get_tree_data(&self) -> String {
        fn base_arc(data_type: String,content: String)->String{
            format!("{} {}\x00{}",data_type,content.len(),content)
        }
        String::new()
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
    let blob = Data::new(content, code, data_type);
    println!("{}", blob);
    println!("{}", encode::get_sha_1(blob.get_data()));
    //rust中\a是无效转义字符 其对应的响铃字符为\x07,在\a在ruby中是合法的转义字符
    let expected =
        b"x\x9CK\xCA\xC9OR04c(\xCFH,Q\xC8,V(-\xD0QH\xC9O\xB6\x07\0_\x1C\x07\x9D".as_bstr();
    let binding = encode::zlib_encode(blob.get_data().as_bytes()).unwrap();
    let result = binding.as_bstr();
    println!(
        "{:?}",
        encode::zlib_encode(blob.get_data().as_bytes())
            .unwrap()
            .as_bstr()
    );
    assert_eq!(expected, result);
    assert_eq!(
        "blob 16\0what is up, doc?",
        decode::decode_reader(
            b"x\x9CK\xCA\xC9OR04c(\xCFH,Q\xC8,V(-\xD0QH\xC9O\xB6\x07\0_\x1c\x07\x9D".as_bytes()
        )
        .unwrap()
    );
}

#[test]
fn tree_test() {
    let content = b"tree 72\x00100644 demo.txt\x000\xD7M%\x84B\xC7\xC6U\x12\xEA\xFA\xB4tV\x8D\xD7\x06\xC40100644 test.txt\x00\xBD\x9D\xBFZ\xAE\x1A8b\xDD\x15&r2F\xB2\x02\x06\xE5\xFC7";
    let mut hasher = Sha1::new();
    hasher.update(content);
    let result = hasher.finalize();
    // format!("{:x}", result)
    println!("{:x}", result);
    // let result =  encode::zlib_encode(content.as_bytes());
    // println!("{:?}",result.unwrap().as_bstr());
    //rust中\a是1无效转义字符 其对应的响铃字符为\x07,在\a在ruby中是合法的转义字符
}

#[test]
fn new_tree_test(){
    let blob_1 = Data::new("test_one".to_string(),"100064".to_string(),"blob".to_string());
    let blob_2 = Data::new("test_two".to_string(),"100064".to_string(),"blob".to_string());
    let nodes = vec![blob_1,blob_2];
    let tree = Data::new_tree("040000".to_string(),"tree".to_string(),nodes);
}
