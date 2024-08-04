use std::net::{TcpStream};
use std::io::{Read, Write};

fn generate_key(plaintext: &str, key: &str) -> String {
    let mut extended_key = key.to_string();
    while extended_key.len() < plaintext.len() {
        extended_key.push_str(key);
    }
    extended_key.truncate(plaintext.len());
    extended_key
}

fn encrypt_vigenere(plaintext: &str, key: &str) -> String {
    let key = generate_key(plaintext, key);
    plaintext
        .chars()
        .zip(key.chars())
        .map(|(p, k)| {
            if p.is_ascii_alphabetic() {
                let shift = k as u8 - if k.is_ascii_uppercase() { b'A' } else { b'a' };
                let base = if p.is_ascii_uppercase() { b'A' } else { b'a' };
                ((p as u8 - base + shift) % 26 + base) as char
            } else {
                p
            }
        })
        .collect()
}

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:7878")?;
    println!("Connected to the server!");

    let unmsg = "Hello from client";
    println!("Data to send: {}", unmsg);
    let key = "KEY";
    let msg = encrypt_vigenere(&unmsg, &key);
    stream.write(msg.as_bytes())?;

    let mut buffer = [0; 512];
    let n = stream.read(&mut buffer)?;
    println!("Received from server: {}", String::from_utf8_lossy(&buffer[..n]));

    Ok(())
}
