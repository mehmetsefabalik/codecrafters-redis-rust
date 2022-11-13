use std::io::BufRead;
use std::io::{BufReader, Write};
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                println!("accepted new connection");
                let mut buf = BufReader::new(_stream.try_clone().unwrap());
                loop {
                    let mut line = String::new();
                    match buf.read_line(&mut line) {
                        Ok(size) => {
                            println!("Received data {} bytes {}", size, line);
                            if line.contains("ping") {
                                _stream.write("+PONG\r\n".as_bytes()).unwrap();
                            }
                        }
                        Err(e) => println!("error while receiving data: {}", e),
                    }
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
