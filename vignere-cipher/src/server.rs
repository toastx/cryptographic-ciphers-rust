use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn generate_key(plaintext: &str, key: &str) -> String {
    let mut extended_key = key.to_string();
    while extended_key.len() < plaintext.len() {
        extended_key.push_str(key);
    }
    extended_key.truncate(plaintext.len());
    extended_key
}

fn decrypt_vigenere(ciphertext: &str, key: &str) -> String {
    let key = generate_key(ciphertext, key);
    ciphertext
        .chars()
        .zip(key.chars())
        .map(|(c, k)| {
            if c.is_ascii_alphabetic() {
                let shift = k as u8 - if k.is_ascii_uppercase() { b'A' } else { b'a' };
                let base = if c.is_ascii_uppercase() { b'A' } else { b'a' };
                ((c as u8 + 26 - shift - base) % 26 + base) as char
            } else {
                c
            }
        })
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
                let key = "KEY";
                let correct_data = decrypt_vigenere(&received_data,&key);
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