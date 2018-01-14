use std;

/*
 * avaible option : -l, -R, -a, -t, -r
 * option are store in a u8
 * bit:		0  0  0  0  0  0  0  0
 * option :	     -S -r -t -R -a -l
*/

fn	valid_option(array: &[u8], mut options: u8) -> u8 {
	let mut i = 1;

	while i < array.len() {
		match array[i] as char {
			'l' => options = options | 0b1,
			'a' => options = options | 0b10,
			'R' => options = options | 0b100,
			't' => {options = options | 0b1000;
			options = options & 0b11011111;},
			'r' => options = options | 0b10000,
			'S' => {options = options | 0b100000;
			options = options & 0b11110111;},
			_ => {
				println!("Unknow option: -{}", array[i] as char);
				std::process::exit(2);
			}
		};
		i += 1;
	}
	options
}

pub fn option(args: std::env::Args) -> (u8, usize) {
	let mut index = 0;
	let mut options: u8 = 0;

	for arg in args.skip(1)
	{
		index += 1;
		if &arg == "--" {
			return (options, index);
		}
		if arg.starts_with("-") == false || &arg == "-" {
			return (options, index - 1);
		}
		options = valid_option(arg.as_bytes(), options);
	}
	(options, index)
}

pub fn option_l(options: u8) -> bool { options & (1 << 0) > 0 }
pub fn option_a(options: u8) -> bool { options & (1 << 1) > 0 }
pub fn option_R(options: u8) -> bool { options & (1 << 2) > 0 }
pub fn option_t(options: u8) -> bool { options & (1 << 3) > 0 }
pub fn option_r(options: u8) -> bool { options & (1 << 4) > 0 }
pub fn option_S(options: u8) -> bool { options & (1 << 5) > 0 }
