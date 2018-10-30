extern crate clap;
use clap::{Arg, App};
use std::fs;
use std::path::Path;
use std::path::PathBuf;

// canonicalize returns a strange unc path on windows
// see https://github.com/rust-lang/rust/issues/42869
fn fix_unc_path_bug(path_buf:&PathBuf) -> PathBuf {
    let new_path_buf = if path_buf.to_string_lossy().starts_with("\\\\?\\") {
        PathBuf::from(path_buf.to_string_lossy().replace("\\\\?\\",""))
    } else {
        PathBuf::from(path_buf)
    };
    return new_path_buf;
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


    let path:PathBuf =
        if cfg!(windows) {
            let path_buf = fs::canonicalize(Path::new(config))?;
            fix_unc_path_bug(&path_buf)
        } else {
            let path_buf = fs::canonicalize(Path::new(config))?;
            path_buf
        };

    println!("Hello, world! {}", path.display());

    Ok(())
}
