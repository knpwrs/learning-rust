use std::io::prelude::*;
use std::net::{ TcpListener, TcpStream };
use std::fs::File;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        // When stream goes out of scope and is dropped at the end of the loop, the connection is
        // closed as part of the drop implementation.
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "index.html")
    } else {
        ("HTTP/1.1 404 Not Found\r\n\r\n", "404.html")
    };

    let mut contents = String::new();
    let mut file = File::open(filename).unwrap();
    file.read_to_string(&mut contents).unwrap();

    let res = format!("{}{}", status_line, contents);

    stream.write(res.as_bytes()).unwrap();
    // flush will wait and prevent the program from continuing until all the bytes are written to
    // the connection; TcpStream contains an internal buffer to minimize calls to the underlying
    // operating system.
    stream.flush().unwrap();
}
