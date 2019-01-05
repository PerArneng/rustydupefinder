
use digest::Digest;
use sha2::Sha256;
use std::io::{self};
use std::fs::{File};
use std::path::Path;
use itertools::Itertools;



pub fn create_hash(path:&Path) -> io::Result<String> {
    let mut digest = Sha256::new();
    let mut file:File = File::open(path)?;
    let _result = io::copy(&mut file, &mut digest)?;
    let result = digest.result();
    let hex_string = format!("{:02x}", result.as_slice().iter().format(""));
    return Ok(hex_string);
}


#[test]
fn create_hash_test() {
    let str = create_hash(Path::new("c:\\audio.log"));
    println!("hex {}", str.expect("a real string"));
}