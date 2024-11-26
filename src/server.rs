use std::{env, fs};
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use RustNet::ThreadPool;

// URI breaks provided URLs into 3 fields
pub struct URI {
    pub(crate) host: String,
    pub(crate) path: String,
    pub(crate) scheme: String,
}

impl URI {
    // init_server makes a multithreaded TCP server
    pub fn init_server(&mut self, mut url: String) {
        let mut temp = url.splitn(2,"://").collect::<Vec<_>>();
        self.scheme = temp[0].parse().unwrap();
        url = temp[1].parse().unwrap();

        assert!(self.scheme == "http" || self.scheme == "https");

        if url.contains("/")  {
            url += "/"
        }
        temp = url.splitn(2, "/").collect::<Vec<_>>();
        self.host = temp[0].parse().unwrap();
        self.path = '/'.to_string() + temp[1].to_string().as_str();

        let listener = TcpListener::bind(self.host.as_str()).unwrap();
        let pool = ThreadPool::new(4);

        for stream in listener.incoming() {
            let mut stream = stream.unwrap();

            pool.execute(move || {
                let reader = BufReader::new(&stream);
                let http_request = reader.lines().next().unwrap().unwrap();

                // FILE PATH
                // Code should be 200 regardless of whether file is found or not
                // Three slashes are there because the hostname is empty
                let (status, file) = match &http_request[..] {
                    "GET / HTTP/1.1" => ("HTTP/1.1 200 OK\r\n\r\n", "hello.html"),
                    _ => ("HTTP/1. 1 404 NOT FOUND\r\n\r\n", "404.html")
                };

                let user_agent = format!("Wikixen/0.1 ({} Photon/0 RustNet/0.01)", env::consts::OS);
                let content = fs::read_to_string(file).unwrap();
                let length = content.len();
                let response = format!("{status}\r\nContent-Length: {length}\r\n\r\n{content}\r\n\r\nUser-Agent: {user_agent}\r\n\r\n");
                stream.write_all(response.as_bytes()).unwrap();
            })
        }

        println!("Shutting down.");
    }
}