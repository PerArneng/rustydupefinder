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
mod utils;



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

    let path:PathBuf = utils::canonicalize(config)?;

    println!("scanning path: '{}'", path.display());

    let mut files = file_lister::list_files_recursively(path.as_path())?;
    files.sort_by(utils::file_name_cmp);


    let sized_groups =
        files.into_iter().group_by(|x| utils::get_file_size(x).unwrap_or(std::u64::MAX));

    //sized_groups.into_iter().for_each(|(x,y)| println!("{:?} {:?}", x , y));

    Ok(())
}
