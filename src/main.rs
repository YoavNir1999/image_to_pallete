mod parse_text;
use parse_text::*;
mod convert_to_scheme;
use convert_to_scheme::*;
mod dir;
use dir::*;
use std::io::prelude::*;
use rayon::iter::*;
mod cli_args_parse;
use cli_args_parse::*;
mod kmeans;
use kmeans::*;

fn main() {
    //parse config
    let dir = curr_dir();
    let config_file = file_to_iter(parse_text::open(&format!("{dir}/files/config.txt")));
    let settings : Vec<String> = config_file.lines().map(|x| x.unwrap()).collect();
    println!("{:?}",parse_args());
    
    //exctacting settings
    let percent : f64 = settings[1].clone().parse().unwrap();
    let scheme : Vec<[u8;3]> = hexes_to_scheme(settings[4..].to_vec());
    let files = return_files(&format!("{dir}/files/"));
    let files : Vec<&String> = files.iter().filter(|x| (return_file_ext(x)=="jpg" || return_file_ext(x)=="png" || return_file_ext(x)=="jpeg")).collect();

    // convert images in parallel
    files.par_iter().for_each(|x| convert(x,&scheme,&percent));

    println!("{} files were converted",files.len());
    
}