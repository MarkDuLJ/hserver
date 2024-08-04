use std::{fs, io::{BufRead, BufReader, Write}, net::{TcpListener, TcpStream}};

fn main(){
    let listner = TcpListener::bind("127.0.0.1:8787").unwrap();

    for stream in listner.incoming() {
        let stream = stream.unwrap();
        handle_conn(stream);
    }
}

fn handle_conn(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);

    let http_req: Vec<_> = buf_reader
        .lines()
        .map(|res| res.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

        let status_line = "HTTP/1.1 200 OK";
        let contents = fs::read_to_string("hello.html").unwrap();
        let len = contents.len();

        let response = format!("{status_line}\r\nContent-Length: {len}\r\n\r\n{contents}");
    
    stream.write_all(response.as_bytes()).unwrap();

    println!("Request: {http_req:#?}");
}