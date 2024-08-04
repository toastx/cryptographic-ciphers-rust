use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;


fn vernam_decrypt(s1: &str, s2: &str) -> String {
    s1.bytes()
        .zip(s2.bytes())
        .map(|(b1, b2)| (b1 ^ b2) as char)
        .collect()
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    loop {
        match stream.read(&mut buffer) {
            Ok(0) => break, 
            Ok(n) => {
                let received_data = String::from_utf8_lossy(&buffer[..n]);
                println!("Received from client: {}", received_data);
                let key = "Sjgnx euti slasia";
                let correct_data = vernam_decrypt(&received_data,&key);
                println!("Decrypted data {}",correct_data);
            }
            Err(_) => {
                println!("Error reading from stream.");
                break;
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    println!("Server listening on port 7878");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(e) => {
                println!("Connection failed: {}", e);
            }
        }
    }
    Ok(())
}