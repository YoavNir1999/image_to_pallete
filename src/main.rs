use image::*;
use hex::*;
mod parse_text;
use parse_text::*;
mod dir;
use dir::*;
use std::io::prelude::*;
use rayon::iter::*;

fn main() {
    //parse config
    let dir = curr_dir();
    let config_file = file_to_iter(parse_text::open(&format!("{dir}/files/config.txt")));
    let settings : Vec<String> = config_file.lines().map(|x| x.unwrap()).collect();
    //let path = settings[0].clone();
    let percent : f64 = settings[1].clone().parse().unwrap();
    let scheme : Vec<[u8;3]> = hexes_to_scheme(settings[4..].to_vec());
    let files = return_files(&format!("{dir}/files/"));
    let files : Vec<&String> = files.iter().filter(|x| (return_file_ext(x)=="jpg" || return_file_ext(x)=="png" || return_file_ext(x)=="jpeg")).collect();

    // open image
    files.par_iter().for_each(|x| convert(x,&scheme,&percent));
    
}

fn convert(file:&String,scheme: &Vec<[u8;3]>, percent:&f64) {
    let mut image = image::open(file).unwrap().into_rgb8();

        // convert image
        for pixel in image.pixels_mut() {
            *pixel = Rgb(pixel_to_scheme(&scheme,&[pixel[0],pixel[1],pixel[2]],*percent))
        }
        image.save(format!("{}_converted.png",return_file_name(&file))).unwrap();
}

fn pixel_to_scheme(scheme:&Vec<[u8;3]>,pixel:&[u8;3],percent:f64) -> [u8;3] {
    let mut closest = (scheme[0].clone(),distance(&scheme[0],&pixel));
    for color in scheme {
        let current_distance = distance(&color,&pixel);
        if current_distance<closest.1 {
            closest = (color.clone(),current_distance)
        }
    }
    return average(&closest.0,&pixel,percent)
}

fn hexes_to_scheme(hexes:Vec<String>) -> Vec<[u8;3]> {
    let mut res = Vec::new();
    let mut temp_arr = [0_u8;3];
    for hex in hexes {
        decode_to_slice(&hex[1..],&mut temp_arr).unwrap();
        res.push(temp_arr.clone())
    }
    return res
}

fn distance(reference:&[u8;3],pixel:&[u8;3]) -> u8 {
    return ((reference[0] as f64 - pixel[0] as f64).powf(2.0)+
    (reference[1] as f64 - pixel[1] as f64).powf(2.0)+
    (reference[2] as f64 - pixel[2] as f64).powf(2.0))
    .sqrt() as u8
}

fn average(pixel1:&[u8;3],pixel2:&[u8;3],percent:f64) -> [u8;3] {
    return [(pixel1[0] as f64 * percent+pixel2[0] as f64 * (1.0-percent)) as u8,
            (pixel1[1] as f64 * percent+pixel2[1] as f64 * (1.0-percent)) as u8,
            (pixel1[2] as f64 * percent+pixel2[2] as f64 * (1.0-percent)) as u8]
}