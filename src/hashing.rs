
use digest::Digest;
use sha2::Sha256;
use std::io::{self, Read};
use std::fs::{File};
use std::path::Path;
use itertools::Itertools;


pub fn read_hash(read:&mut Read) -> io::Result<String> {
    let mut digest = Sha256::new();
    let _result = io::copy(read, &mut digest)?;
    let result = digest.result();
    let hex_string = format!("{:02x}", result.as_slice().iter().format(""));
    return Ok(hex_string);
}

pub fn path_hash(path:&Path) -> io::Result<String> {
    let mut file:File = File::open(path)?;
    return read_hash(&mut file);
}

#[test]
fn create_hash_test() {
    let str = read_hash(&mut BufReader::new("Potato!".as_bytes()));
    assert_eq!("hex {}", str.expect("a real string"));
}