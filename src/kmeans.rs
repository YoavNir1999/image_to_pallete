use hex::encode;


#[derive(Copy,Clone,Debug)]
pub struct Point {
    pub r:f64,
    pub g:f64,
    pub b:f64
}

impl Point {
    fn distance(&self,other:&Point) -> f64 {
        return (
            (self.r-other.r).powf(2.0)+
            (self.g-other.b).powf(2.0)+
            (self.b-other.b).powf(2.0)
        ).sqrt()
    }

    pub fn point_to_pixel(&self) -> [u8;3] {
        return [self.r as u8, self.g as u8, self.b as u8]
    }
}

#[derive(Debug)]
pub struct Kmeans {
    k:usize,
    data:Vec<Point>,
    pub means:Vec<Point>
}

impl PartialEq for Point {
    fn eq(&self,other:&Point) -> bool {
        return self.r==other.r &&
        self.g==other.g &&
        self.b==other.b
    }
}

impl Kmeans {
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

    pub fn new_means(&mut self) -> f64 {
        let mut diff = 0.0;
        let mut partitions = vec!([0.0,0.0,0.0,0.0];self.k);
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

pub fn rgb_to_hex(point:&[u8]) -> String {
    return format!("#{}",encode(point))
}

pub fn rgb_to_image(colors:Vec<[u8;3]>) {
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
    buffer.save("colors.jpg").unwrap()
}