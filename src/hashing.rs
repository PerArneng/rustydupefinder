use digest::Digest;
use sha2::Sha256;
use std::io::{self};
use std::fs::{File};
use std::path::Path;
use itertools::Itertools;


pub fn read_hash(read:&mut io::Read) -> io::Result<String> {
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
fn test_read_hash() {
    let str = read_hash(&mut io::BufReader::new("Potato!".as_bytes()));
    assert_eq!("bf82efa58491263cf5c3c8866f7f20ec18d25f2c0894b323184a7625ed529084",
               str.expect("a real string"));
}