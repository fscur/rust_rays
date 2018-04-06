use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::vec::Vec;
const BITS_PER_PIXEL : usize = 3;

fn main() {
    let width = 640;
    let height = 480;
    let mut image = raytrace(width, height);
    save(width, height, image.as_mut_slice());
}

fn raytrace(width : usize, height : usize) -> Vec<f32> {
    let length : usize = width * height * BITS_PER_PIXEL;
    
    let mut data = vec![0.0; length];

    //let mut data = Vec::new();

    for y in 0..height {
        for x in 0..width {
            let i = BITS_PER_PIXEL * (y * width  + x);

            let r = x as f32/width as f32;
            let g = y as f32/height as f32;
            let b = 1.0;
            
            data[i + 0] = r as f32;
            data[i + 1] = g as f32;
            data[i + 2] = b as f32;        
        }
    }
    
    return data
}

fn save(width : usize, height : usize, data: &mut[f32]) {
    let path = Path::new("image.ppm");
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}",
                           display,
                           why.description()),
        Ok(file) => file,
    };

    let mut content = format!("P3 {} {} 255\n", width, height);

    for y in 0..height {
        for x in 0..width {
            let i = BITS_PER_PIXEL * (y * width  + x);
            
            let r = data[i + 0] * 255.0;
            let g = data[i + 1] * 255.0;
            let b = data[i + 2] * 255.0; 

            content.push_str(&format!("{} {} {}\n", r as i32, g as i32, b as i32));
        }
    }

    match file.write_all(content.as_bytes()) {
        Err(why) => {
            panic!("couldn't write to {}: {}", display,
                                               why.description())
        },
        Ok(_) => println!("successfully wrote to {}", display),
    }
}
