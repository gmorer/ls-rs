use std::env::args;
use std::path::PathBuf;
use std::convert::From;
use std::error::Error;

pub mod file;
pub mod option;

fn read_path(path: PathBuf, options: u8) -> () {
	println!("BUFF: {:?}", path);
	match std::fs::read_dir(path.as_path()) {
		Ok(dir) => {
			let mut files: Vec<file::File> = std::vec::Vec::new();
			println!("{}:", path.display());
			for file in dir {
				if let Ok(file) = file {
					if file.file_name().to_string_lossy().into_owned().starts_with(".") {
						if option::option_a(options) {
							files.push(file::File::new(file));
						}
					} else {
						files.push(file::File::new(file));
					}
				}
			}
			files.sort_by(|a, b| a.cmp(b, options));
			for file in files {
				if option::option_R(options) && file.is_directory() {
					let mut newpath = path.clone();
					newpath.push(file.name());
					println!("");
					//TODO read path a te end of the for
					read_path(newpath, options);
				}
				// TODO real print
				println!("{}", file);
			}
		}
		Err(err) => {
			println!("ls-rs: cannot access {:?}: {}", path, err.description());
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
	for path in paths {
		read_path(path, options);
	}
}
