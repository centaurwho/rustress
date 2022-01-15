use std::collections::HashMap;
use std::ffi::OsString;
use std::{fs, io};
use std::fmt::{Display, Formatter};
use std::fs::FileType;
use std::io::Error;
use std::ops::Add;
use std::path::PathBuf;

pub mod threadpool;
pub mod server;
pub mod client;
mod configparser;

#[derive(Debug)]
pub enum PathContent {
    FileContent(String),
    FolderContent(HashMap<String, PathContent>),
}

fn make_path(parent_path: String, child_path: String) -> PathBuf {
    let mut path_buf = PathBuf::from(parent_path);
    path_buf.push(child_path);
    path_buf
}

// TODO: Cleanup
pub fn load_content_from_folder(folder_path: String) -> io::Result<HashMap<String, PathContent>> {
    let paths = fs::read_dir(folder_path.clone())?;
    let mut file_content_by_name = HashMap::new();
    for file_path in paths {
        let file_name = file_path?.file_name().into_string().unwrap();
        let full_path = make_path(folder_path.clone(), file_name.clone());
        let file_meta = fs::metadata(&full_path)?;

        if file_meta.is_file() {
            match fs::read_to_string(&full_path) {
                Ok(file_content) => {
                    file_content_by_name.insert(String::from(full_path.to_str().unwrap()), PathContent::FileContent(String::new()));
                }
                Err(err) => {
                    println!("Error {} during opening {:?}", err, file_name);
                }
            }
        } else if file_meta.is_dir() {
            let internal_map = load_content_from_folder(String::from(full_path.to_str().unwrap()))?;
            file_content_by_name.insert(String::from(full_path.to_str().unwrap()), PathContent::FolderContent(internal_map));
        } else {
            // file_meta.is_symlink is true
            // TODO
            unimplemented!("Can not handle symbolic links yet.")
        }
    }
    Ok(file_content_by_name)
}