extern crate libc;

use std::path::PathBuf;
use std::os::raw::{c_ulong, c_ushort};

use file;
use std;
use option;

#[cfg(target_os="macos")]
const TIOCGWINSZ: c_ulong  = 0x40087468;
#[cfg(all(target_env="musl", not(target_os="macos")))]
const TIOCGWINSZ: c_int = 0x00005413;
#[cfg(all(not(target_env="musl"), not(target_os="macos")))]
const TIOCGWINSZ: c_ulong = 0x00005413;

//#[warn(dead_code)]
struct WinSize {
    _ws_row: c_ushort,
    ws_col: c_ushort,
    _ws_xpixel: c_ushort,
    _ws_ypixel: c_ushort
}

fn get_term_size() -> Option<usize> {
    if unsafe{libc::isatty(libc::STDOUT_FILENO) != 1} {
        return None
    }
    Some ( unsafe {
        let mut winsize = WinSize{_ws_row: 0, ws_col: 0, _ws_xpixel: 0, _ws_ypixel: 0};
        libc::ioctl(libc::STDOUT_FILENO, libc::TIOCGWINSZ, &mut winsize);
        let cols = if winsize.ws_col > 0 { winsize.ws_col } else { 0 };
        cols as usize
    })
}

fn max_size(files: &Vec<file::File>) -> usize {
    let mut max: usize = 0;

    for file in files {
        if file.name().len() > max {
            max = file.name().len();
        }
    }
    max
}

fn print_small(files: Vec<file::File>, old_path: PathBuf, options: u8) -> Vec<PathBuf> {
    lazy_static!{static ref TERM_SIZE: Option<usize> = get_term_size(); }
    let mut new_paths = std::vec::Vec::new();
    let max_size = max_size(&files);
    for (index, file) in files.iter().enumerate() {
        if option::option_rr(options) && file.is_directory() {
            let mut new_path = old_path.clone();
            new_path.push(file.name());
            new_paths.push(new_path);
        }
        match *TERM_SIZE {
            Some(size) => {
                let col = size / (max_size + 10);
                if (index + 1) % col == 0 {
                    println!("{}", file.name());
                } else {
                    print!("{:<lenght$}    ", file.name(), lenght=max_size);
                }
            },
            None => println!("{}", file.name())
        }
    }
    println!("");
    new_paths
}

fn print_big(files: Vec<file::File>, old_path: PathBuf, options: u8) -> Vec<PathBuf> {
    let mut new_paths = std::vec::Vec::new();

    for file in files {
        if option::option_rr(options) && file.is_directory() {
            let mut new_path = old_path.clone();
            new_path.push(file.name());
            new_paths.push(new_path);
        }
        println!("{}", file);

    }
    new_paths
}

pub fn print_file(files: Vec<file::File>, old_path: PathBuf, options: u8) -> Vec<PathBuf> {

    if option::option_l(options) {
        print_big(files, old_path, options)
    } else {
        print_small(files, old_path, options)
    }
    //println!("");
}
