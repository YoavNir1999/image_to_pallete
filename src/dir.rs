use std::env::current_dir;
use std::fs::{read_dir};
use std::fs::File;
use crate::kmeans::rgb_to_hex;
use std::io::prelude::*;

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

pub fn scheme_to_file(name:String,scheme:Vec<[u8;3]>) {
    let mut f = File::create(format!("{}.txt",name)).unwrap();
    let mut content = String::new();
    for color in scheme {
        content += &format!("{}\n",rgb_to_hex(&color))
    }
    f.write_all(content.as_bytes()).unwrap();
    println!("success converting {}",return_short_file_name(&name));
}