// ref: https://doc.rust-lang.org/book/ch20-01-single-threaded.html
use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

pub fn http_server_v1() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        // handle_connection_v1(stream);
        handle_connection_v11(stream);
    }
}

/*
ref: https://developer.mozilla.org/en-US/docs/Web/HTTP/Overview
* http request,
GET /echo HTTP/1.1
Host: reqbin.com
Accept: text/html

* http response, 
HTTP/1.1 200 OK
Date: Sat, 09 Oct 2010 14:28:02 GMT
Server: Apache
Last-Modified: Tue, 01 Dec 2009 20:18:22 GMT
ETag: "51142bc1-7449-479b075b2891b"
Accept-Ranges: bytes
Content-Length: 29769
Content-Type: text/html

<!DOCTYPE html>… (here come the 29769 bytes of the requested web page)
 */
 // get the 1st line of request.
fn handle_connection_v11(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = if request_line == "GET /index.html HTTP/1.1" {
        ("HTTP/1.1 200 OK", "./src/day8/hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "./src/day8/404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}


fn handle_connection_v1(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {:#?}", http_request);
}