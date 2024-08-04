use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;


fn rail_fense_decrypt(ciphertext: &str, num_rails: usize) -> String {
    if num_rails == 1 {
        return ciphertext.to_string();
    }

    let mut rails: Vec<Vec<char>> = vec![Vec::new(); num_rails];
    let mut rail_lens = vec![0; num_rails];
    let mut rail = 0;
    let mut direction = 1;

    for _ in ciphertext.chars() {
        rail_lens[rail] += 1;
        rail = (rail as isize + direction) as usize;

        if rail == 0 || rail == num_rails - 1 {
            direction = -direction;
        }
    }

    let mut index = 0;
    for (i, len) in rail_lens.iter().enumerate() {
        for _ in 0..*len {
            rails[i].push(ciphertext.chars().nth(index).unwrap());
            index += 1;
        }
    }

    let mut result = String::new();
    rail = 0;
    direction = 1;

    for _ in ciphertext.chars() {
        result.push(rails[rail].remove(0));
        rail = (rail as isize + direction) as usize;

        if rail == 0 || rail == num_rails - 1 {
            direction = -direction;
        }
    }

    result
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    loop {
        match stream.read(&mut buffer) {
            Ok(0) => break, 
            Ok(n) => {
                let received_data = String::from_utf8_lossy(&buffer[..n]);
                println!("Received from client: {}", received_data);
                let correct_data = rail_fense_decrypt(&received_data,3);
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