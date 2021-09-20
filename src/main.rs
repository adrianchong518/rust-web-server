use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::time::Duration;
use std::{fs, process, thread};

use rust_web_server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let pool = ThreadPool::new(4).unwrap_or_else(|err| {
        eprintln!("Error creating web server thread pool: {}", err);
        process::exit(1);
    });

    for stream in listener.incoming().take(4) {
        match stream {
            Ok(stream) => {
                pool.execute(|| {
                    handle_connection(stream);
                });
            }
            Err(err) => {
                eprintln!("Connection Error: {}", err);
            }
        }
    }

    println!("Shutting down");
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let index = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(index) {
        ("HTTP/1.1 200 OK", "public/index.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "public/index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "public/404.html")
    };

    let content = fs::read_to_string(filename).unwrap();

    let response = format!(
        "{}\r\n Content-Length: {}\r\n\r\n{}",
        status_line,
        content.len(),
        content
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
