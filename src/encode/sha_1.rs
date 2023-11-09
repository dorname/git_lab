use sha1::{Sha1, Digest};

pub fn get_sha_1(s:String)-> String{
    let mut hasher = Sha1::new();
    hasher.update(s.as_bytes());
    let result = hasher.finalize();
    format!("{:x}", result)
}

pub fn sha_1(s:Vec<u8>)->String{
    let mut hasher = Sha1::new();
    hasher.update(s);
    let result = hasher.finalize();
    format!("{:x}", result)
}