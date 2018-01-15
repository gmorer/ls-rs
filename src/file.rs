extern crate time;
extern crate libc;

//pub mod options;

use option;

use std::os::unix::fs::PermissionsExt;
use std::os::unix::fs::MetadataExt;
use std::ffi::CStr;
use std::fmt;
use std::mem;
use std::ptr;
use std;

#[derive(Debug)]
pub struct File {
    name: String,
    permissions: String,
    block: u64,
    nlink: u64,
    size: u64,
    owner: String,
    group: String,
    modified: i64,
    time : String
}

fn read_permission(data: &std::fs::Metadata) -> String {
	let mut permissions = String::from("");
    let mut bit = 256;

        if data.file_type().is_dir() {
            permissions.push('d');
        } else if data.file_type().is_symlink() {
            permissions.push('l');
        } else if data.file_type().is_file() {
            permissions.push('-');
        } else {
           permissions.push('?');
        }
        loop {
            if (bit == 256 || bit == 32 || bit == 4) &&
				data.permissions().mode() & bit != 0 {
                permissions.push('r');
            } else if (bit == 128 || bit == 16 || bit == 2) &&
                       data.permissions().mode() & bit != 0
            {
                permissions.push('w');
            } else if (bit == 64 || bit == 8 || bit == 1) &&
                       data.permissions().mode() & bit != 0
            {
                permissions.push('x');
            } else {
                permissions.push('-');
            }
            if bit == 1 {
                break;
            }
            bit = bit / 2;
        }
		permissions
}

fn get_groupename(gid :u32) -> String {
	unsafe{
		let mut rslt = ptr::null_mut();
		let amt = match libc::sysconf(libc::_SC_GETPW_R_SIZE_MAX) {
			n if n < 0 => 512 as usize,
			n => n as usize,
		};
		let mut buf = Vec::with_capacity(amt);
		let mut passwd: libc::group = mem::zeroed();
		match libc::getgrgid_r(gid, &mut passwd, buf.as_mut_ptr(),
			buf.capacity() as libc::size_t, &mut rslt) {
			0 if !rslt.is_null() => {
				CStr::from_ptr(passwd.gr_name).to_str().unwrap().to_owned()
			}
			_=> "????".to_string()
		}
	}
}

fn get_username(uid :u32) -> String {
	unsafe{
		let mut rslt = ptr::null_mut();
		let amt = match libc::sysconf(libc::_SC_GETPW_R_SIZE_MAX) {
			n if n < 0 => 512 as usize,
			n => n as usize,
		};
		let mut buf = Vec::with_capacity(amt);
		let mut passwd: libc::passwd = mem::zeroed();
		match libc::getpwuid_r(uid, &mut passwd, buf.as_mut_ptr(),
			buf.capacity() as libc::size_t, &mut rslt) {
			0 if !rslt.is_null() => {
				CStr::from_ptr(passwd.pw_name).to_str().unwrap().to_owned()
			}
			_=> "????".to_string()
		}
	}
}

impl File {
    pub fn new(file: std::fs::DirEntry) -> File {
        if let Ok(data) = file.metadata() {
            return File {
                name: file.file_name()
                    .to_string_lossy()
                    .into_owned(),
                permissions: read_permission(&data),
                time: time::strftime("%b %d %R", &time::at_utc(time::Timespec::new(data.mtime(), 0))).unwrap(),
                block: data.blocks(),
                nlink: data.nlink(),
                size: data.len(),
                owner: get_username(data.uid()),
                group: get_groupename(data.gid()),
                modified: data.mtime(),
            };
        } else {
            println!("Couldn't read metadata for {}", file.path().display());
            return File {
                name: file.file_name()
                    .to_string_lossy()
                    .into_owned(),
                permissions: "??????????".to_string(),
                nlink: 0,
                time: "?????????????".to_string(),
                block: 0,
                size: 0,
                owner: "????".to_string(),
                group: "????".to_string(),
                modified: 0,
            };
        }
    }
	pub fn is_directory(&self) -> bool {self.permissions.starts_with("d")}
	pub fn name(&self) -> &String {&self.name}
    pub fn cmp(&self, f: &File, options: u8) -> std::cmp::Ordering {
		let mut rslt: std::cmp::Ordering = std::cmp::Ordering::Equal;
		if option::option_t(options)  {
			rslt = f.modified.cmp(&self.modified);
		} else if option::option_S(options)  {
			rslt = f.size.cmp(&self.size);
		}
		if rslt == std::cmp::Ordering::Equal {
        	rslt = self.name.cmp(&f.name);
		}
		if option::option_r(options) {
			return rslt.reverse()
		}
		rslt
    }
}


impl fmt::Display for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {} {} {} {} {} {}",
            self.permissions,
            self.nlink,
            self.owner,
            self.group,
            self.size,
            self.time,
            self.name
        )
    }
}
