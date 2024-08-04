use std::net::{TcpStream};
use std::io::{Read, Write};

fn rail_fense_encrypt(plaintext: &str, num_rails: usize) -> String {
    if num_rails == 1{
        return plaintext.to_string();
    }

    let mut rails = vec![String::new(); num_rails];
    let mut rail = 0;
    let mut direction:i32 = 1;

    for char in plaintext.chars() {
        rails[rail].push(char);
        rail = (rail as i32 + direction) as usize;

        if rail == 0 || num_rails - rail == 1{
            direction = -direction;
        }
    }
    rails.concat()
}


fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:7878")?;
    println!("Connected to the server!");

    let unmsg = "Hello from client";
    println!("Data to send: {}", unmsg);
    let msg = rail_fense_encrypt(&unmsg, 3);
    stream.write(msg.as_bytes())?;

    let mut buffer = [0; 512];
    let n = stream.read(&mut buffer)?;
    println!("Received from server: {}", String::from_utf8_lossy(&buffer[..n]));

    Ok(())
}