use image::*;
use hex::*;
use crate::dir::*;

//converts an image to a color scheme
pub fn convert(file:&String,scheme: &Vec<[u8;3]>, percent:&f64) {
    let mut image = image::open(file).unwrap().into_rgb8();

    // convert image
    for pixel in image.pixels_mut() {
        *pixel = Rgb(pixel_to_scheme(&scheme,&[pixel[0],pixel[1],pixel[2]],*percent))
    }
    let file_name = return_file_name(&file);

    // save image
    image.save(format!("{}_converted.{}",file_name,return_file_ext(file))).unwrap();
    println!("{} was converted successfully",return_short_file_name(&file));
}

// converts a pixel to the nearest color from a color scheme
pub fn pixel_to_scheme(scheme:&Vec<[u8;3]>,pixel:&[u8;3],percent:f64) -> [u8;3] {
    let mut closest = (scheme[0].clone(),distance(&scheme[0],&pixel));
    for color in scheme {
        let current_distance = distance(&color,&pixel);
        if current_distance<closest.1 {
            closest = (color.clone(),current_distance)
        }
    }
    return average(&closest.0,&pixel,percent)
}

// turns a vector of hex colors to rgb colors
pub fn hexes_to_scheme(hexes:Vec<String>) -> Vec<[u8;3]> {
    let mut res = Vec::new();
    let mut temp_arr = [0_u8;3];
    for hex in hexes {
        if !hex.is_empty() {
            decode_to_slice(&hex[1..],&mut temp_arr).unwrap();
            res.push(temp_arr.clone())
        }
    }
    return res
}

// calculates eucledean distance between two rgb values
pub fn distance(reference:&[u8;3],pixel:&[u8;3]) -> u8 {
    return ((reference[0] as f64 - pixel[0] as f64).powf(2.0)+
    (reference[1] as f64 - pixel[1] as f64).powf(2.0)+
    (reference[2] as f64 - pixel[2] as f64).powf(2.0))
    .sqrt() as u8
}

// performs a weighted average on the target color with the original color
pub fn average(pixel1:&[u8;3],pixel2:&[u8;3],percent:f64) -> [u8;3] {
    if percent == 1.0 {
        return [pixel1[0],pixel1[1],pixel1[2]]
    }
    return [(pixel1[0] as f64 * percent+pixel2[0] as f64 * (1.0-percent)) as u8,
            (pixel1[1] as f64 * percent+pixel2[1] as f64 * (1.0-percent)) as u8,
            (pixel1[2] as f64 * percent+pixel2[2] as f64 * (1.0-percent)) as u8]
}