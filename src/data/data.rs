use crate::data;
//山楂树下
use crate::decode;
use crate::encode;
use crate::obj_type;
use anyhow::Error;
use bstr::ByteSlice;
use hex::encode;
use sha1::{Digest, Sha1};
use std::clone;
use std::io::Read;
use std::time::{SystemTime, UNIX_EPOCH};
use std::{fmt, result};

#[derive(Clone)]
struct Data {
    content: Vec<u8>,
    mode: String,
    data_type: String,
    nodes: Option<Vec<Data>>,
    file_name: String,
    msg: Option<String>,
}
impl Data {
    fn new_blob(content: Vec<u8>, mode: String, data_type: String, file_name: String) -> Self {
        Data {
            content: content,
            mode: mode,
            data_type: data_type,
            nodes: None,
            file_name: file_name,
            msg: None,
        }
    }
    fn new_commit(msg: String, node: Data) -> Self {
        //commit
        let byte_type = "commit".as_bytes().to_vec();
        let space = " ".as_bytes().to_vec();
        let nil = "\x00".as_bytes().to_vec();
        let next_line = "\n".as_bytes().to_vec();
        let author = "author".as_bytes().to_vec();
        let author_val = "dorname".as_bytes().to_vec();
        let email_val = "<lgqfighting@163.com>".as_bytes().to_vec();
        let now = SystemTime::now();
        // let timestamp = now.duration_since(UNIX_EPOCH).unwrap().as_secs();
        let timestamp = "1699193914";
        let timezone_offset = "+0800";
        let node_copy = node.clone();
        let formatted_timestamp = format!("{} {}", timestamp, timezone_offset)
            .as_bytes()
            .to_vec();
        let commiter = "committer".as_bytes().to_vec();
        let node_byte_type = node.data_type.as_bytes().to_vec();
        let node_hash = encode::sha_1(node.content).as_bytes().to_vec();
        let message = msg.clone().as_bytes().to_vec();
        let mut v = vec![
            node_byte_type,
            space.clone(),
            node_hash,
            next_line.clone(),
            author,
            space.clone(),
            author_val.clone(),
            space.clone(),
            email_val.clone(),
            space.clone(),
            formatted_timestamp.clone(),
            next_line.clone(),
            commiter,
            space.clone(),
            author_val,
            space.clone(),
            email_val,
            space.clone(),
            formatted_timestamp.clone(),
            next_line.clone(),
            next_line.clone(),
            message,
            next_line,
        ];
        // let v = vec![byte_type,space,]
        let len = v.clone().concat().len().to_string().as_bytes().to_vec();
        v.insert(0, byte_type);
        v.insert(1, space);
        v.insert(2, len);
        v.insert(3, nil);
        let content = v.concat();
        Data {
            content: content,
            mode: "".to_string(),
            data_type: "commit".to_string(),
            nodes: Some(vec![node_copy]),
            file_name: "".to_string(),
            msg: Some(msg),
        }
    }
    fn new_tree(mode: String, data_type: String, nodes: Vec<Data>) -> Self {
        let copy_nodes = nodes.clone();
        let byte_type = data_type.as_bytes().to_vec();
        let space = " ".to_string().as_bytes().to_vec();
        let nil = b"\x00".to_vec();
        let mut v: Vec<Vec<u8>> = vec![byte_type, space, nil];
        let mut len = 0;
        for node in nodes {
            let mut data = node.get_tree_data();
            len += data.len();
            v.push(data);
        }
        let len = len.to_string().as_bytes().to_vec();
        v.insert(2, len);
        Data {
            mode: mode,
            data_type: data_type,
            nodes: Some(copy_nodes),
            file_name: " ".to_string(),
            content: v.concat(),
            msg: None,
        }
        // println!("{:?}",v.concat().as_bstr())
    }
    fn get_data(&self) -> String {
        format!(
            "{} {}\x00{}",
            self.data_type,
            self.content.len(),
            self.content.as_bstr()
        )
    }
    fn add_node(&mut self, subnode: Data) {
        if let Some(x) = &mut self.nodes {
            x.push(subnode);
        } else {
            self.nodes = Some(vec![subnode]);
        }
    }
    // 树对象的文件内容
    //#{content}
    //mode 40000代表树对象 100644代表blob对象
    // #{mode} #{dir1_name or file1_name}#{dir1 or file1's hash}#{mode} #{dir2_name or file2_name}#{dir2 or file2's hash}...
    // **注意：**`#{dir1 or file1's hash}`需要从`HEX`转换成字节序再作字符串拼接。
    fn get_tree_data(&self) -> Vec<u8> {
        let hash = encode::get_sha_1(self.get_data());
        // println!("{}:\n",hash);
        let mode = self.mode.as_bytes().to_vec();
        let queue = hex::decode(hash).expect("Invalid hex string");
        let file_name = self.file_name.as_bytes().to_vec();
        let nil = "\x00".as_bytes().to_vec();
        let space = " ".as_bytes().to_vec();
        [mode, space, file_name, nil, queue].concat()
    }
}
impl fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {}\x00{}",
            self.data_type,
            self.content.len(),
            self.content.as_bstr()
        )
    }
}
#[test]
fn test() {
    let content = b"what is up, doc?";
    // let content: String = String::from("test");
    let mode: String = String::from("100644");
    let data_type: String = obj_type::BLOB.to_string();
    let blob = Data::new_blob(content.to_vec(), mode, data_type, "demo.txt".to_string());
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
    let content = b"tree 72\0100644 demo.txt\00\xD7M%\x84B\xC7\xC6U\x12\xEA\xFA\xB4tV\x8D\xD7\x06\xC40100644 test.txt\0\xBD\x9D\xBFZ\xAE\x1A8b\xDD\x15&r2F\xB2\x02\x06\xE5\xFC7";
    let mut hasher = Sha1::new();
    hasher.update(content);
    let result = hasher.finalize();
    // format!("{:x}", result)
    println!("{:?}", content.as_bstr());

    println!("{:x}", result);
    // let result =  encode::zlib_encode(content.as_bytes());
    // println!("{:?}",result.unwrap().as_bstr());
    //rust中\a是1无效转义字符 其对应的响铃字符为\x07,在\a在ruby中是合法的转义字符
}

#[test]
fn new_tree_test() {
    let blob_1 = Data::new_blob(
        "what is up, doc?".as_bytes().to_vec(),
        "100644".to_string(),
        "blob".to_string(),
        "test.txt".to_string(),
    );
    let blob_2 = Data::new_blob(
        "test".as_bytes().to_vec(),
        "100644".to_string(),
        "blob".to_string(),
        "demo.txt".to_string(),
    );
    let tree = Data::new_tree(
        "40000".to_string(),
        "tree".to_string(),
        vec![blob_2, blob_1],
    );
    // println!("{:?}",blob_1.get_tree_data().as_bstr());
    // println!("{:?}",blob_2.get_tree_data().as_bstr());
    // println!("{:?}",tree.content.as_bstr());
    // println!("{:?}",encode::sha_1(tree.content)); //成功输出树对象的正确hash：dcc20f823c15ba6394596b475c03d08cdc4417a0
    assert_eq!(
        encode::sha_1(tree.content),
        "dcc20f823c15ba6394596b475c03d08cdc4417a0".to_string()
    );
}
#[test]
fn unleagal_test() {
    let hex_string = "1a3862";
    let umleagal = "\x1A8b".to_string();
    let bytes = hex::decode(hex_string).unwrap();
    // println!("{:?}:{:?}",bytes,bytes.as_bstr().to_string().replace("\u{1a}","\x1A"));
    println!("{:?}", "\\x1A");
    println!("{:?}", umleagal);
}
#[test]
fn commit_test() {
    let blob_1 = Data::new_blob(
        "what is up, doc?".as_bytes().to_vec(),
        "100644".to_string(),
        "blob".to_string(),
        "test.txt".to_string(),
    );
    let blob_2 = Data::new_blob(
        "test".as_bytes().to_vec(),
        "100644".to_string(),
        "blob".to_string(),
        "demo.txt".to_string(),
    );
    let tree = Data::new_tree(
        "40000".to_string(),
        "tree".to_string(),
        vec![blob_2, blob_1],
    );
    let commit = Data::new_commit("first commit".to_string(), tree);
    assert_eq!(
        encode::sha_1(commit.content),
        "ff11bc76cb7e488b83369a169e255fb4ca2ee328".to_string()
    );
}
#[test]
fn time_stamp_test() {
    use std::time::{SystemTime, UNIX_EPOCH};

    let now = SystemTime::now();
    let timestamp = now.duration_since(UNIX_EPOCH).unwrap().as_secs();
    let timezone_offset = "+0800";
    let formatted_timestamp = format!("{} {}", timestamp, timezone_offset);
    println!("{}", formatted_timestamp);
}
