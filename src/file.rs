extern crate time;

use std::os::unix::fs::PermissionsExt;
use std::os::unix::fs::MetadataExt;
use std::ffi::OsStr;
use std::fmt;
use std;

#[derive(Debug)]
pub struct File {
    name: String,
    permissions: String,
    block: u64,
    nlink: u64,
    size: u64,
    uid: u32,
    gid: u32,
    modified: i64,
    time : String
}

impl File {
    pub fn new(file: std::fs::DirEntry) -> File {
        let mut permissions = String::from("");
        let mut bit = 256;

        if let Ok(data) = file.metadata() {
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
            return File {
                name: file.path()
                    .file_name()
                    .unwrap_or(OsStr::new("??????????"))
                    .to_string_lossy()
                    .into_owned(),
                permissions: permissions,
                time: time::strftime("%b %d %R", &time::at_utc(time::Timespec::new(data.mtime(), 0))).unwrap(),
                block: data.blocks(),
                nlink: data.nlink(),
                size: data.len(),
                uid: data.uid(),
                gid: data.gid(),
                modified: data.mtime(),
            };
        } else {
            println!("Couldn't read metadata for {}", file.path().display());
            permissions.push_str("?????????");
            return File {
                name: file.path()
                    .file_name()
                    .unwrap_or(OsStr::new("??????????"))
                    .to_string_lossy()
                    .into_owned(),
                permissions: permissions,
                nlink: 0,
                time: "?????????????".to_string(),
                block: 0,
                size: 0,
                uid: 0,
                gid: 0,
                modified: 0,
            };
        }
    }
    pub fn cmp(&self, f: &File) -> std::cmp::Ordering {
        self.name.cmp(&f.name)
    }
}


impl fmt::Display for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {} {} {} {} {} {}",
            self.permissions,
            self.nlink,
            self.uid,
            self.gid,
            self.size,
            self.time,
            self.name
        )
    }
}
