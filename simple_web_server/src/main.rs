use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use simple_web_server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for value in listener.incoming() {
        let stream = value.unwrap();
        println!("Connection established!.\n{:#?}", stream);
        thread::spawn(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    let pool = ThreadPool::new(4);

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", ""),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "sleep.html")
        }
        _ => ("HTTP/1.1 404 Not Found", "404.html"),
    };
    let response_contents = match &filename[..] {
      ""=> format!("<!DOCTYPE html><html lang=\"en\"><head> <meta charset=\"UTF-8\"> <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\"><title>Document</title></head><body><h1>Hello</h1><p>This is the response to <mark>{request_line}<mark/></p></body></html>"),
      "sleep.html"=>  format!("<!DOCTYPE html><html lang=\"en\"><head> <meta charset=\"UTF-8\"> <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\"><title>Document</title></head><body><h1>Just slept</h1><p>This is the response to <mark>{request_line}<mark/></p></body></html>"),
      _=> fs::read_to_string(filename).unwrap().replace("AOEU", &request_line),
    };
    let response_length = response_contents.len();
    let response =
        format!("{status_line}\r\nContent-Length: {response_length}\r\n\r\n{response_contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
