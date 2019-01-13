
use std::path::PathBuf;
use std::fs;
use std::path::Path;
use std::io;
use std::collections::HashMap;

pub fn file_name_cmp(x1:&PathBuf, x2:&PathBuf) -> std::cmp::Ordering {
    x1.file_name().cmp(&x2.file_name())
}

// canonicalize returns a strange unc path on windows
// see https://github.com/rust-lang/rust/issues/42869
pub fn fix_unc_path_bug(path_buf:PathBuf) -> Result<PathBuf, io::Error> {
    let new_path_buf = if path_buf.to_string_lossy().starts_with("\\\\?\\") {
        PathBuf::from(path_buf.to_str().unwrap().replace("\\\\?\\",""))
    } else {
        path_buf.clone()
    };
    return Ok(new_path_buf)
}

pub fn canonicalize(path:&str) -> Result<PathBuf, io::Error>  {
    if cfg!(windows) {
        fs::canonicalize(Path::new(path))
            .and_then(fix_unc_path_bug)
    } else {
        fs::canonicalize(Path::new(path))
    }
}


pub fn get_file_size(path:&PathBuf) -> io::Result<u64> {
    path.metadata()
        .and_then(|metadata| Ok(metadata.len()))
}

