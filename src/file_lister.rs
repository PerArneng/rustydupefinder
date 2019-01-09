
use std::{io};
use std::path::{Path, PathBuf};

pub fn list_files_recursively<P>(dir:P) -> io::Result<Vec<PathBuf>>
    where P: AsRef<Path>
{

    fn file_name_cmp(x1:&walkdir::DirEntry, x2:&walkdir::DirEntry) -> std::cmp::Ordering {
        x1.file_name().cmp(&x2.file_name())
    }

    let mut file_entries:Vec<PathBuf> = Vec::new();
    walkdir::WalkDir::new(dir).into_iter()
        .filter(|e| e.is_ok())
        .map(|x|x.unwrap().into_path())
        .for_each(
            |path_buf| file_entries.push(path_buf)
        );

    return Ok(file_entries);
}