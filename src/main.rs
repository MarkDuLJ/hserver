use std::{io::{BufRead, BufReader}, net::{TcpListener, TcpStream}};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8787").unwrap();

    for stream in listener.incoming(){
        let stream = stream.unwrap();

        handle_conn(stream);
    }

}

fn handle_conn(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_req: Vec<_> = buf_reader
            .lines()
            .map(|res| res.unwrap())
            .collect();

    println!("request: {:#?}", http_req);
}
