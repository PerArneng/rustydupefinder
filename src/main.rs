extern crate clap;
extern crate walkdir;
extern crate itertools;
extern crate digest;
extern crate sha2;

use clap::{Arg, App};
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::io;
use itertools::Itertools;


mod hashing;
mod file_lister;

// canonicalize returns a strange unc path on windows
// see https://github.com/rust-lang/rust/issues/42869
fn fix_unc_path_bug(path_buf:PathBuf) -> Result<PathBuf, io::Error> {
    let new_path_buf = if path_buf.to_string_lossy().starts_with("\\\\?\\") {
        PathBuf::from(path_buf.to_str().unwrap().replace("\\\\?\\",""))
    } else {
        path_buf.clone()
    };
    return Ok(new_path_buf)
}

fn canonicalize(path:&str) -> Result<PathBuf, io::Error>  {
    if cfg!(windows) {
        fs::canonicalize(Path::new(path))
               .and_then(fix_unc_path_bug)
    } else {
        fs::canonicalize(Path::new(path))
    }
}

fn main() -> std::io::Result<()> {

    let matches = App::new("Rusty Dupe Finder (Duplicate file finder)")
                                    .version("1.0")
                                    .author("Per Arneng")
                                    .about("Finds duplicate files on a give path")
                                    .arg(Arg::with_name("path")
                                        .short("p")
                                        .long("path")
                                        .value_name("PATH")
                                        .required(true)
                                        .help("The path to search")
                                        .takes_value(true))
                                    .get_matches();

    let config = matches.value_of("path").unwrap_or(".");

    let path:PathBuf = canonicalize(config)?;

    println!("scanning path: '{}'", path.display());


    let mut files = file_lister::list_files_recursively(path.as_path())?;

    fn print_entry(entry:walkdir::DirEntry) {
        println!("{} - {} - {}", entry.file_name().to_str().unwrap(),
                    entry.path().display(), hashing::path_hash(entry.path()).unwrap());
    }

    fn file_name_cmp(x1:&walkdir::DirEntry, x2:&walkdir::DirEntry) -> std::cmp::Ordering {
        x1.file_name().cmp(&x2.file_name())
    }


    files.sort_by(file_name_cmp);

    fn name_and_size(e:&walkdir::DirEntry) -> String {
        let len = e.metadata()
                            .map(|m| m.len())
                            .map(|len| len.to_string())
                            .unwrap();
        e.file_name()
            .to_str()
            .map(|s| s.to_string())
            .map(|s| format!("{} - {}", s, len) )
            .unwrap()
    }

    fn is_file(e:&walkdir::DirEntry) -> bool { e.file_type().is_file() }

    /*file_entries
        .into_iter()
        .filter(is_file)
        .unique_by(name_and_size)
        .for_each(print_entry);
*/
    Ok(())
}
