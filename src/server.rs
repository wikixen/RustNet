use std::{fs};
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use RustNet::ThreadPool;

pub fn init_server() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_conn(stream);
        });
    }

    println!("Shutting down.");
}

fn handle_conn(mut stream: TcpStream){
    let reader = BufReader::new(&mut stream);
    let http_req = reader.lines().next().unwrap().unwrap();

    let (status, file) = match &http_req[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK\r\n\r\n", "hello.html"),
        _ => ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };
    let content = fs::read_to_string(file).unwrap();
    let length = content.len();
    let response = format!("{status}\r\nContent-Length: {length}\r\n\r\n{content}");
    stream.write_all(response.as_bytes()).unwrap();
}


