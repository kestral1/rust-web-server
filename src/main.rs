use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};
use web_server_rust_book::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    println!("Connection established");

    match &request_line[..] {
        "GET / HTTP/1.1" => {
            let status_line = "HTTP/1.1 200 OK";
            let contents = fs::read_to_string("hello.html").unwrap();
            let length = contents.len();

            let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

            stream.write_all(response.as_bytes()).unwrap();
        }
        "GET /foo HTTP/1.1" => {
            let status_line = "HTTP/1.1 200 OK";
            let contents = fs::read_to_string("foo.html").unwrap();
            let length = contents.len();

            let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

            stream.write_all(response.as_bytes()).unwrap();
        }
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            let status_line = "HTTP/1.1 200 OK";
            let contents = fs::read_to_string("foo.html").unwrap();
            let length = contents.len();

            let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

            stream.write_all(response.as_bytes()).unwrap();
        }
        _ => {}
    }
}
