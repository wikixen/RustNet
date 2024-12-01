use std::collections::HashMap;
use std::io::{BufReader, Read, Write};
use std::net::TcpStream;

// URI breaks provided URLs into 3 fields
pub struct URI {
    pub(crate) host: String,
    pub(crate) path: String,
    pub(crate) scheme: String,
}

impl URI {
    // parse_uri 
    pub fn parse_uri(&mut self, mut url: String) {
        let mut temp = url.split("://").collect::<Vec<&str>>();
        (self.scheme, url) = (temp[0].parse().unwrap(), temp[1].parse().unwrap());


        assert!(self.scheme == "http" || self.scheme == "https");

        if !url.contains("/")  {
            url += "/"
        }
        temp = url.splitn(2, "/").collect::<Vec<&str>>();
        (self.host, self.path) = (temp[0].parse().unwrap(), temp[1].parse().unwrap());
        println!("{}", self.host);
        println!("{}", self.scheme);
        println!("{}", self.path);
    }

    // init_server makes a single-threaded TCP server with the URL provided
    pub fn init_server(&self) {
        let mut stream = TcpStream::connect(self.host.as_str().to_owned()).unwrap();
        let request = format!("GET {} HTTP/1.1\r\nHost: {}\r\n\r\n", self.path, self.host);
        stream.write(request.as_bytes()).unwrap();

        let mut reader = BufReader::new(&stream);
        let mut response = String::new();
        reader.read_to_string(&mut response).unwrap();
        let status_line: String = response.lines().next().unwrap().parse().unwrap();
        let temp = status_line.splitn(2, " ").collect::<Vec<&str>>();
        let (version, content, explanation) = (temp[0], temp[1], temp[2]);

        println!("{}\n{}\n{}", version, content, explanation);
        let mut response_headers = HashMap::new();
        loop {
            let line = response.lines().next().unwrap();
            if line == "\r\n" { break }
            let (header, value) = line.split_at(line.find(':').unwrap());
            response_headers.insert(header.to_lowercase().to_owned(), value.trim().to_owned());
        }
        assert!(!response_headers.contains_key("transfer-encoding") && !response_headers.contains_key("content-encoding"));

        let content = response.as_bytes();
        println!("{:?}", content);
    }
}