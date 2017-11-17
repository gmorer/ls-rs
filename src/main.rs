use std::env::args;
use std::path::PathBuf;
use std::convert::From;
use std::error::Error;

pub mod file;

fn main() {

	let mut paths: Vec<PathBuf> = args().skip(1).map(From::from).collect();

	paths.sort();

	for path in paths {
		match std::fs::read_dir(path.as_path()){
			Ok(dir) => {
				let mut files: Vec<file::File> = std::vec::Vec::new();
				println!("{}:", path.display());
				for file in dir {
					if let Ok(file) = file {
						files.push(file::File::new(file));
					}
				}
				for file in files {
					println!("{}", file);
				}
			}
			Err(err) => {
				println!("ls-rs: cannot access '{:?}': {}", path, err.description());
			}
		}
	}
}
