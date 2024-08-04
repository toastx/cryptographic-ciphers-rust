use std::net::{TcpStream};
use std::io::{Read, Write};

fn vernam_encrypt(s1: &str, s2: &str) -> String {
    s1.bytes()
        .zip(s2.bytes())
        .map(|(b1, b2)| (b1 ^ b2) as char)
        .collect()
}

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:7878")?;
    println!("Connected to the server!");

    let unmsg = "Hello from client";
    println!("Data to send: {}", unmsg);
    let key = "Sjgnx euti slasia";
    let msg = vernam_encrypt(&unmsg, &key);
    stream.write(msg.as_bytes())?;

    let mut buffer = [0; 512];
    let n = stream.read(&mut buffer)?;
    println!("Received from server: {}", String::from_utf8_lossy(&buffer[..n]));

    Ok(())
}