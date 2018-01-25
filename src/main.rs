#[macro_use]
extern crate lazy_static;

use std::env::args;
use std::path::PathBuf;
use std::convert::From;
use std::error::Error;

pub mod file;
pub mod option;
pub mod print;

fn read_path(path: PathBuf, options: u8) -> () {
	let new_paths;

	match std::fs::read_dir(path.as_path()) {
		Ok(dir) => {
			let mut files: Vec<file::File> = Vec::new();
			for file in dir {
				if let Ok(file) = file {
					if file.file_name().to_string_lossy().into_owned().starts_with(".") {
						if option::option_a(options) {
							files.push(file::File::new(file, options));
						}
					} else {
						files.push(file::File::new(file, options));
					}
				}
			}
			files.sort_by(|a, b| a.cmp(b, options));
			new_paths = print::print_file(files, path, options);
			if option::option_rr(options) {
				let len = new_paths.len();
				for (index, path) in new_paths.iter().enumerate() {
					println!("{}:", path.to_str().unwrap_or("???"));
					read_path(path.to_path_buf(), options);
					if index < len - 1 {
						println!("");
					}
				}
			}
		}
		Err(err) => {
			eprintln!("ls-rs: cannot access {:?}: {}", path, err.description());
		}
	};
}

fn main() {
	let (options, option_len) = option::option(args());
	let mut paths: Vec<PathBuf> = args().skip(1 + option_len)
		.map(From::from).collect();
	paths.sort();
	if paths.is_empty() {
		paths.push(PathBuf::from("./"));
	}
	let len = paths.len();
	for (index, path) in paths.iter().enumerate() {
		if len > 1 {
			println!("{}:", path.to_str().unwrap_or("???"));
		}
		read_path(path.to_path_buf(), options);
		if index + 1 < len {
			println!("");
		}
	}
}
