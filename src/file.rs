use std::os::unix::fs::PermissionsExt;
use std::os::linux::fs::MetadataExt;
use std::time::SystemTime;
use std::fmt;
use std;

pub struct File {
name: String,
		  permissions: String,
		  size: u64,
		  uid: u32,
		  gid: u32,
		  modified: SystemTime
}

impl File {
	pub fn new(file: std::fs::DirEntry) -> File
	{
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
				if (bit == 256 || bit == 32 || bit == 4) && data.permissions().mode()
					& bit != 0 {
						permissions.push('r');
					} else if (bit == 128 || bit == 16 || bit == 2) && data.permissions().mode()
						& bit != 0 {
							permissions.push('w');
						} else if (bit == 64 || bit == 8 || bit == 1) && data.permissions().mode()
							& bit != 0 {
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
name: match file.path().file_name() {
		  None => "????????".to_string(),
			   Some(name) => match name.to_str() {
				   None => "????????".to_string(),
				   Some(name) => name.to_string()
			   }
	  },
permissions: permissions,
			 size: data.len(),
			 uid: data.st_uid(),
			 gid: data.st_gid(),
			 modified: SystemTime::now()
			}
		}
		else {
			println!("Couldn't read metadata for {}", file.path().display());
			permissions.push_str("?????????");
			return File {
name: match file.path().file_name() {
		  None => "????????".to_string(),
			   Some(name) => match name.to_str() {
				   None => "????????".to_string(),
				   Some(name) => name.to_string()
			   }
	  },
permissions: permissions,
			 size: 0,
			 uid: 0,
			 gid: 0,
			 modified: SystemTime::now()
			}
		}
	}
	pub fn cmp(&self, f: &File) -> std::cmp::Ordering {
		self.name.cmp(&f.name)
	}
}

impl fmt::Display for File {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{} {} {} {} time {}", self.permissions, self.uid, self.gid, self.size,
				/*self.modified,*/ self.name)
	}
}
