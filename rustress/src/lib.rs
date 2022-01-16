use std::{fs, io};
use std::collections::HashMap;
use std::ffi::OsString;
use std::path::PathBuf;

pub mod threadpool;
pub mod server;
pub mod client;
mod configparser;


// There isn't an enum like this in the standard library. Since file type concept depends on OS
//  rust decided to generalize the implementation. So FileType in std library only consists of a
//  mode field containing necessary flags and uses it to extract the type
enum FileType {
    File,
    Dir,
    SymLink,
}

impl FileType {
    fn from_meta(meta: fs::Metadata) -> FileType {
        if meta.is_file() {
            FileType::File
        } else if meta.is_dir() {
            FileType::Dir
        } else {
            FileType::SymLink
        }
    }
}


pub fn load(path: String) -> HashMap<OsString, String> {
    let path = PathBuf::from(path);
    let mut content_map = HashMap::new();
    load_into_map(&path, &mut content_map);
    content_map
}

fn load_into_map(parent_path: &PathBuf, content_map: &mut HashMap<OsString, String>) -> io::Result<()> {
    let files_in_current_dir = fs::read_dir(&parent_path)?;
    for file_entry in files_in_current_dir {
        let file_path = file_entry?.path();
        let file_meta = fs::metadata(&file_path)?;
        match FileType::from_meta(file_meta) {
            FileType::File => {
                match fs::read_to_string(&file_path) {
                    Ok(file_content) => {
                        content_map.insert(file_path.into_os_string(), file_content);
                    }
                    Err(err) => {
                        println!("Error {} during opening {:?}", err, &file_path);
                    }
                }
            }
            FileType::Dir => {
                load_into_map(&file_path, content_map);
            }
            FileType::SymLink => {
                unimplemented!("Can not handle symbolic links yet");
            }
        }
    }
    Ok(())
}