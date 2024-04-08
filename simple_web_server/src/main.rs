use std::{
    fs,
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

    let request_line = buf_reader.lines().next().unwrap().unwrap();
    let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "")
    } else {
        ("HTTP/1.1 404 Not Found", "404.html")
    };
    let response_contents = if filename == "" {
        format!("<!DOCTYPE html><html lang=\"en\"><head> <meta charset=\"UTF-8\"> <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\"><title>Document</title></head><body><h1>Hello</h1><p>This is the response to <mark>{request_line}<mark/></p></body></html>")
    } else {
        fs::read_to_string(filename).unwrap().replace("AOEU", &request_line)
    };
    let response_length = response_contents.len();
    let response =
        format!("{status_line}\r\nContent-Length: {response_length}\r\n\r\n{response_contents}");

    stream.write_all(response.as_bytes()).unwrap();

    // let http_request: Vec<_> = buf_reader
    //     .lines()
    //     .map(|result| result.unwrap())
    //     .take_while(|line| !line.is_empty())
    //     .collect();
    // println!("Request: {:#?}", http_request);
    // let truncated_one = &http_request[1..http_request.iter().len()];
    // let content=format!("<!DOCTYPE html><html lang=\"en\"><head> <meta charset=\"UTF-8\"> <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\"> <title>Document</title></head><body><h1>Hello</h1><p>{:#?}</p></body></html>",truncated_one);
    // let content_length = content.len();
    // let response = format!("HTTP/1.1 200 OK\r\nContent-Length: {content_length}\r\n\r\n{content}",);

    // stream.write_all(response.as_bytes()).unwrap();
}
