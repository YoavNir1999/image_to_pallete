use hex::encode;
use image::*;
use crate::dir::{return_short_file_name};

// rgb values struct
#[derive(Copy,Clone,Debug)]
pub struct Point {
    pub r:f64,
    pub g:f64,
    pub b:f64
}

impl Point {
    // calculates eucledean distance between two points
    fn distance(&self,other:&Point) -> f64 {
        return (
            (self.r-other.r).powf(2.0)+
            (self.g-other.b).powf(2.0)+
            (self.b-other.b).powf(2.0)
        ).sqrt()
    }

    // converts a point to a pixel accepted by the image crate
    pub fn point_to_pixel(&self) -> [u8;3] {
        return [self.r as u8, self.g as u8, self.b as u8]
    }
}

// struct for all the values of a k-means clusters
#[derive(Debug)]
pub struct Kmeans {
    k:usize,
    data:Vec<Point>,
    pub means:Vec<Point>
}

// implements a way to equate two points
impl PartialEq for Point {
    fn eq(&self,other:&Point) -> bool {
        return self.r==other.r &&
        self.g==other.g &&
        self.b==other.b
    }
}

impl Kmeans {
    // return a new k-means struct
    pub fn new(data:Vec<Point>,k:usize) -> Kmeans {
        let mut means = Vec::new();
        for i in 0..k {
            means.push(Point {
                r : i as f64 *(255.0/k as f64),
                g : i as f64 *(255.0/k as f64),
                b : i as f64 *(255.0/k as f64)
            });
                    
        }
        return Kmeans {
            k:k,
            data:data,
            means:means
        }
    }

    // calculates a single iteration of new means from current clusters
    pub fn new_means(&mut self) -> f64 {
        let mut diff = 0.0;
        let mut partitions = vec!([0.0,0.0,0.0,0.0];self.k);

        // clustering the points to the means
        for p_idx in 0..self.data.len() {
            let mut closest_idx = 0;
            let mut closest_distance = self.data[p_idx].distance(&self.means[0]);
            for m_idx in 1..self.means.len() {
                let new_dist = self.data[p_idx].distance(&self.means[m_idx]);
                if new_dist<closest_distance {
                    closest_distance=new_dist;
                    closest_idx=m_idx;
                };
            }
            partitions[closest_idx][0]+=1.0;
            partitions[closest_idx][1]+=self.data[p_idx].r;
            partitions[closest_idx][2]+=self.data[p_idx].g;
            partitions[closest_idx][3]+=self.data[p_idx].b;
        }

        // calculating new means
        for idx in 0..self.k {
            let new_mean = Point {
                r:partitions[idx][1]/partitions[idx][0],
                g:partitions[idx][2]/partitions[idx][0],
                b:partitions[idx][3]/partitions[idx][0]
            };
            diff += new_mean.distance(&self.means[idx]);
            self.means[idx] = new_mean;
        }

        return diff
    }


}

// converts rgb values to hex values
pub fn rgb_to_hex(point:&[u8]) -> String {
    return format!("#{}",encode(point))
}

// converts a color scheme of rgb to an image representing all the colors
pub fn rgb_to_image(colors:&Vec<[u8;3]>,file_name:String) {
    let mut buffer : image::RgbImage = image::ImageBuffer::new(colors.len() as u32 * 100,500);
    let mut inc = 0;
    for idx in 0..colors.len() {
        for y in 0..500 {
            for x in 0..100 {
                buffer.put_pixel(x+inc, y, image::Rgb(colors[idx]))
            }
        }
        inc+=100;
    }
    buffer.save(&format!("files_for_extraction/{}_colors.jpg",file_name)).unwrap()
}

// converts an image to a color scheme and returns it while creating an image representation of the colors
pub fn image_to_hex(path:String,k:usize) -> Vec<[u8;3]>{ 
    // open image
    let img = image::open(&path).unwrap();
    let pixels = img.pixels();

    // extract pixels to vector
    let mut data1 : Vec<Point> = Vec::new();
    for pixel in pixels.map(|i| (i.2)) {
        data1.push(Point {
            r : pixel[0] as f64,
            g : pixel[1] as f64,
            b : pixel[2] as f64
        })
    }

    // creates new k-means struct
    let mut kmeans = Kmeans::new(data1,k);

    // calculates the means
    for _i in 0..18 {
        let _diff = kmeans.new_means();
        // uncomment below to show the accuracy of the means
        //println!("{}",diff);
    }

    let mut colors = Vec::new();

    // turning the means to a vector of rgb
    for mean in kmeans.means {
        colors.push(mean.point_to_pixel());
    };

    // creates an image representing the colors
    rgb_to_image(&colors,return_short_file_name(&path));

    return colors
}