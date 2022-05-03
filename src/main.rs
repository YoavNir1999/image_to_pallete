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
    //parse args
    let args = parse_args();
    match args.2 {
        Operation::Extract => extract_colors(),
        Operation::ExtractAndMatch => extract_convert(),
        Operation::MatchFromScheme => from_scheme()
    }
    
}

fn from_scheme() {
    //parse config
    let dir = curr_dir();
    let config_file = file_to_iter(parse_text::open(&format!("{dir}/config/config.txt")));
    let settings : Vec<String> = config_file.lines().map(|x| x.unwrap()).collect();
    
    //exctacting settings
    let percent : f64 = settings[1].clone().parse().unwrap();
    let scheme : Vec<[u8;3]> = hexes_to_scheme(settings[4..].to_vec());
    let files = return_files(&format!("{dir}/files_for_conversion/"));
    let files : Vec<&String> = files.iter().filter(|x| (return_file_ext(x)=="jpg" || return_file_ext(x)=="png" || return_file_ext(x)=="jpeg")).collect();

    // convert images in parallel
    files.par_iter().for_each(|x| convert(x,&scheme,&percent));

    println!("{} files were converted",files.len());
}

fn extract_colors() {
    let dir = curr_dir();
    let files = return_files(&format!("{dir}/files_for_extraction/"));
    let files : Vec<&String> = files.iter().filter(|x| (return_file_ext(x)=="jpg" || return_file_ext(x)=="png" || return_file_ext(x)=="jpeg")).collect();
    files.par_iter().for_each(|x| scheme_to_file(x.clone().to_owned(),image_to_hex(x.clone().to_owned(),8)));
}

fn extract_convert() {
    let dir = curr_dir();
    let config_file = file_to_iter(parse_text::open(&format!("{dir}/config/config.txt")));
    let settings : Vec<String> = config_file.lines().map(|x| x.unwrap()).collect();
    let percent : f64 = settings[1].clone().parse().unwrap();
    let files = return_files(&format!("{dir}/files_for_conversion/"));
    let files : Vec<&String> = files.iter().filter(|x| (return_file_ext(x)=="jpg" || return_file_ext(x)=="png" || return_file_ext(x)=="jpeg")).collect();
    let scheme = return_files(&format!("{dir}/files_for_extraction/"));
    let scheme : Vec<&String> = scheme.iter().filter(|x| (return_file_ext(x)=="jpg" || return_file_ext(x)=="png" || return_file_ext(x)=="jpeg")).collect();
    files.par_iter().for_each(|x| convert(x,&image_to_hex(scheme[0].clone().to_owned(),8),&percent));
}