use std::{
    env,
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    let port = env::var("MRC_PORT").expect("$MRC_PORT is not set");

    let localhost = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(localhost.clone()).unwrap();
    println!("Listening at http://{}", localhost);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established!");

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

    let request_line = &http_request.first();

    let contents;

    if request_line.unwrap() == "GET /favicon.ico HTTP/1.1" {
        contents = fs::read("static/icons/favicon.ico").unwrap();

    } else {
        contents = fs::read("content/index.html").unwrap();
    }

    let length = contents.len();

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n",
        length
    );

    let _ = stream.write_all(response.as_bytes());
    let _ = stream.write_all(&contents);
}