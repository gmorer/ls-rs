use std;

/*
 * avaible option : -l, -R, -a, -t, -r
 * option are store in a u8
 * 00000000
*/

fn	valid_option(array: &[u8]) -> bool {
	let mut i = 1;
	while i < array.len() {
		match array[i] as char {
			'l' | 'R' | 'a' | 't' | 'r' => {} ,
			_ => {
				println!("Unknow option: {}", i);
				return false;
			}
		};
		i += 1;
	}
	true
}

pub fn option(args: std::env::Args) -> (usize, usize) {
	let mut index = 0;

	let args = args.skip(1);
	for arg in args
	{
		index += 1;
		if &arg == "--" {
			return (0, index);
		}
		if arg.starts_with("-") == false || &arg == "-" {
			return (0, index - 1);
		}
		let array = arg.as_bytes();
		valid_option(array);
	}
	// TODO real option handler
	(index, 0)
}
