// ref: https://rust-lang.github.io/async-book/09_example/01_running_async_code.html
extern crate async_std;

// std和async_std都有TcpListener/TcpStream
// 不同的是async_std的版本需要调用await，函数才会执行, 也就是说两者的签名不太一样

use std::{
    fs,
    io::prelude::*,
    // net::{TcpListener, TcpStream},
    time::Duration,
};
use async_std::{
    task,
    prelude::*,
    net::{TcpListener, TcpStream},
};
use futures::stream::StreamExt;

/*
pub async fn http_async_server() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        // Warning: This is not concurrent!
        handle_connection(stream).await;
    }
}
*/

pub async fn http_async_server() {
    let listener = TcpListener::bind("127.0.0.1:7878").await.unwrap();
    listener
        .incoming()
        .for_each_concurrent(/* limit */ None, |tcpstream| async move {
            let tcpstream = tcpstream.unwrap();
            handle_connection(tcpstream).await;
        })
        .await;
    }

async fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).await;

    let get = b"GET /index.html HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "./src/day8/hello.html")
    } else if buffer.starts_with(sleep) {
        task::sleep(Duration::from_secs(5)).await;
        ("HTTP/1.1 200 OK\r\n\r\n", "./src/day8/hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "./src/day8/404.html")
    };
    let contents = fs::read_to_string(filename).unwrap();

    let response = format!("{status_line}{contents}");
    stream.write(response.as_bytes()).await;
    stream.flush().await;
}