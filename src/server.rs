use std::{env, fs};
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use RustNet::ThreadPool;

// init_server makes a multithreaded TCP server
pub fn init_server() {
    let listener = TcpListener::bind(format!("127.0.0.1:{}",handle_input())).unwrap();
    let pool = ThreadPool::new(4);

    // .take() can be added to the end of listener.incoming() to change how many requests can be made
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_conn(stream);
        });
    }

    println!("Shutting down.");
}

// handle_input takes user input & returns a string to TcpListener in init_server
fn handle_input() -> String{
    use std::io::BufRead;
    let mut line = String::new();

    line
}

// handle_conn handles the responses to users from the TCP server made in init_server
fn handle_conn(mut stream: TcpStream){
    let reader = BufReader::new(&mut stream);
    let http_req = reader.lines().next().unwrap().unwrap();

    // FILE PATH
    // Code should be 200 regardless of whether file is found or not
    // Three slashes are there because the hostname is empty
    let (status, file) = match &http_req[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK\r\n\r\n", "hello.html"),
        _ => ("HTTP/1. 1 404 NOT FOUND\r\n\r\n", "404.html")
    };
    const USER_AGENT: String = format!("Wikixen/0.1 ({} Photon/0 RustNet/0.01)", env::consts::OS);
    let content = fs::read_to_string(file).unwrap();
    let length = content.len();
    let response = format!("{status}\r\nContent-Length: {length}\r\n\r\n{content}\r\n\r\nUser-Agent: {USER_AGENT}\r\n\r\n");
    stream.write_all(response.as_bytes()).unwrap();
}