use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for value in listener.incoming() {
        let stream = value.unwrap();
        println!("Connection established!.\n{:#?}", stream);
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    println!("Request: {:#?}", http_request);
    // if let truncatedOne= http_request
    let content=format!("<!DOCTYPE html><html lang=\"en\"><head> <meta charset=\"UTF-8\"> <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\"> <title>Document</title></head><body>{:?}</body></html>",http_request);
    let content_length = content.len();
    let response =
        format!("HTTP/1.1 299 OK,but no rea\r\nContent-Length: {content_length}\r\n\r\n{content}",);

    stream.write_all(response.as_bytes()).unwrap();
}
