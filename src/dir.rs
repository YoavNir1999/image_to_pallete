use std::env::current_dir;
use std::fs::{read_dir};

pub fn curr_dir() -> String {
    let path = current_dir().unwrap();
    return path.display().to_string();
}

pub fn return_files(path:&String) -> Vec<String> {
    let files = read_dir(path).unwrap();
    let mut res: Vec<String> = Vec::new();
    for file in files {
        let path = file.unwrap().path();
        res.push(path.as_path().display().to_string());
    }
    return res;
}

pub fn return_file_ext(file:&String) -> String {
    let vec = file.split(".").collect::<Vec<&str>>();
    return vec[1].to_owned()
}

pub fn return_file_name(file:&String) -> String {
    let vec = file.split(".").collect::<Vec<&str>>();
    return vec[0].to_owned()
}

pub fn return_short_file_name(file:&String) -> String {
    let mut vec = file.split("/").collect::<Vec<&str>>();
    return vec.pop().unwrap_or("error in parsing file name").to_owned()
}