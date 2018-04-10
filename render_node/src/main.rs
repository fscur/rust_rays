// use std::error::Error;
// use std::fs::File;
// use std::path::Path;
// use std::vec::Vec;
use std::io::prelude::*;
use std::net::{ TcpStream };
//use std::io::Write;
use std::io::Read;

//const BITS_PER_PIXEL : usize = 3;

fn u32_to_byte_array(x:u32) -> [u8;4] {
    let b1 : u8 = ((x >> 24) & 0xff) as u8;
    let b2 : u8 = ((x >> 16) & 0xff) as u8;
    let b3 : u8 = ((x >> 8) & 0xff) as u8;
    let b4 : u8 = (x & 0xff) as u8;
    return [b1, b2, b3, b4]
}

fn byte_array_to_u32(x:&[u8]) -> u32 {
    let mut value : u32 = 0;
    value |= (x[0] as u32) << 24;
    value |= (x[1] as u32) << 16;
    value |= (x[2] as u32) << 8;
    value |= x[3] as u32;
    return value
}

fn main() {
    let mut stream = TcpStream::connect("localhost:1500").unwrap();

    let mut buffer = [0;4];
    
    stream.read(&mut buffer);

    let mut id_count = byte_array_to_u32(&buffer);
    println!("[ Client ] Received scene IDs count: {:?}", id_count);

    while id_count > 0
    {
        stream.read(&mut buffer).unwrap();

        println!(
            "[ Client ] Received scene ID: {:?}", byte_array_to_u32(&buffer));

        id_count-=1;
    }

    println!("[ Client ] Received all IDs.");

    let mut missing_ids_count = u32_to_byte_array(2);
    stream.write(&missing_ids_count)
        .expect("[ Client ] Failed to send missing IDs count.");

    println!("[ Client ] Sent missing IDs count.");

    let mut ids = Vec::new();
    ids.append(&mut u32_to_byte_array(10).to_vec());
    ids.append(&mut u32_to_byte_array(21).to_vec());
    stream.write(&ids.as_slice());
    println!("[ Client ] Sent missing IDs.");
}

//fn main() {
    // let width = 640;
    // let height = 480;
    // let mut image = raytrace(width, height);
    // save(width, height, image.as_mut_slice());
//}

// fn raytrace(width : usize, height : usize) -> Vec<f32> {
//     let length : usize = width * height * BITS_PER_PIXEL;
    
//     let mut data = vec![0.0; length];

//     //let mut data = Vec::new();

//     for y in 0..height {
//         for x in 0..width {
//             let i = BITS_PER_PIXEL * (y * width  + x);

//             let r = x as f32/width as f32;
//             let g = y as f32/height as f32;
//             let b = 1.0;
            
//             data[i + 0] = r as f32;
//             data[i + 1] = g as f32;
//             data[i + 2] = b as f32;        
//         }
//     }
    
//     return data
// }

// fn save(width : usize, height : usize, data: &mut[f32]) {
//     let path = Path::new("image.ppm");
//     let display = path.display();

//     // Open a file in write-only mode, returns `io::Result<File>`
//     let mut file = match File::create(&path) {
//         Err(why) => panic!("couldn't create {}: {}",
//                            display,
//                            why.description()),
//         Ok(file) => file,
//     };

//     let mut content = format!("P3 {} {} 255\n", width, height);

//     for y in 0..height {
//         for x in 0..width {
//             let i = BITS_PER_PIXEL * (y * width  + x);
            
//             let r = data[i + 0] * 255.0;
//             let g = data[i + 1] * 255.0;
//             let b = data[i + 2] * 255.0; 

//             content.push_str(&format!("{} {} {}\n", r as i32, g as i32, b as i32));
//         }
//     }

//     match file.write_all(content.as_bytes()) {
//         Err(why) => {
//             panic!("couldn't write to {}: {}", display,
//                                                why.description())
//         },
//         Ok(_) => println!("successfully wrote to {}", display),
//     }
// }
