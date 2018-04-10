use std::net::{ TcpListener, TcpStream };
use std::io::{ Write, Read };
use std::io;
use std::str;

fn main() {

    println!("[ Server ] Started.");

    let ip = "0.0.0.0";
    let port = "1500";
    let address = format!("{}:{}", ip, port);
    let listener = TcpListener::bind(address).unwrap();
    
    for stream in listener.incoming() {
         match stream {
            Ok(stream) => {
                handle_client(stream); 
            }
            Err(e) => { 
                println!("{}", e)
             }
        }
    }

    println!("[ Server ] Closed.");
}

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

fn handle_client(mut stream : TcpStream) {
    println!(
        "[ Server ] Connection accepted from {}", 
        stream.peer_addr().unwrap());

    let mut has_job = false;

    while !has_job {
        println!("[ Server ] Has Job? (yes/no)");
        
        let mut result = String::new();
        
        io::stdin().read_line(&mut result)
            .expect("[ Server ] Failed to read input.");

        has_job = result.eq("yes\r\n");
    }

    let mut id_count = u32_to_byte_array(5);
    stream.write(&id_count)
        .expect("[ Server ] Failed to send ids.");

    println!("[ Server ] Sent scene IDs count.");

    let mut ids = Vec::new();
    ids.append(&mut u32_to_byte_array(10).to_vec());
    ids.append(&mut u32_to_byte_array(21).to_vec());
    ids.append(&mut u32_to_byte_array(35).to_vec());
    ids.append(&mut u32_to_byte_array(7).to_vec());
    ids.append(&mut u32_to_byte_array(29).to_vec());
    stream.write(&ids.as_slice());
    println!("[ Server ] Sent scene IDs.");

    let mut buffer = [0;4];
    stream.read(&mut buffer)
        .expect("[ Server ] Failed to receive missing IDs count.");
    let mut id_count = byte_array_to_u32(&buffer);
    println!("[ Server ] Client reported {:?} missing IDs.", id_count);

    while id_count > 0
    {
        stream.read(&mut buffer).unwrap();

        println!(
            "[ Server ] Missing ID received: {:?}", byte_array_to_u32(&buffer));

        id_count -= 1;
    }

    println!("[ Server ] Received all missing IDs.");
}