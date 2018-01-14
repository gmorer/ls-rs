use std::env::args;
use std::path::PathBuf;
use std::convert::From;
use std::error::Error;

pub mod file;
pub mod option;

fn main() {
	let (options, option_len) = option::option(args());
    let mut paths: Vec<PathBuf> = args().skip(1 + option_len).map(From::from).collect();
    paths.sort();
	if paths.is_empty() {
		paths.push(PathBuf::from("./"));
	}
    for path in paths {
        match std::fs::read_dir(path.as_path()) {
            Ok(dir) => {
                let mut files: Vec<file::File> = std::vec::Vec::new();
                println!("{}:", path.display());
                for file in dir {
                    if let Ok(file) = file {
                        files.push(file::File::new(file));
                    }
                }
                files.sort_by(|a, b| a.cmp(b, options));
                for file in files {
					// TODO real print
                    println!("{}", file);
                }
            }
            Err(err) => {
                println!("ls-rs: cannot access {:?}: {}", path, err.description());

            }
        }
    }
}
