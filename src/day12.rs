// ref: https://rust-lang.github.io/async-book/09_example/01_running_async_code.html
extern crate async_std;

// std & async_std have both TcpListener/TcpStream, but they are not the same.
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

// ref: https://rust-lang.github.io/async-book/01_getting_started/04_async_await_primer.html
// block_on: turn async fn to regular fn, and it blocks
// use async+await to keep it unblocking - 两者并行执行
use futures::executor::block_on;

struct Song {}
async fn learn_song() -> Song { return Song {}; }
async fn sing_song(song: Song) { /* ... */ }
async fn dance() { /* ... */ }

fn test_block_on() {
    let song = block_on(learn_song());
    block_on(sing_song(song));
    block_on(dance());
}

// learn_song & sing_song are executed in series
// 注意
// - await函数必须在async函数中
// - await函数本身也是block的，也就是await会block直到async函数执行完
// - 如果不call await这个函数不会被执行
// - await返回的是一个Future
async fn learn_and_sing() {
    // Wait until the song has been learned before singing it.
    // We use `.await` here rather than `block_on` to prevent blocking the
    // thread, which makes it possible to `dance` at the same time.
    let song = learn_song().await;
    sing_song(song).await;
}

// learn_and_sing & dance are executed in parallel
// they won't be executed until join is called.
// 如果要并行执行，需要用到join或者其他方法
// join是个宏，可以支持多个异步函数
async fn test_async_main() {
    let f1 = learn_and_sing();
    let f2 = dance();

    // `join!` is like `.await` but can wait for multiple futures concurrently.
    // If we're temporarily blocked in the `learn_and_sing` future, the `dance`
    // future will take over the current thread. If `dance` becomes blocked,
    // `learn_and_sing` can take back over. If both futures are blocked, then
    // `async_main` is blocked and will yield to the executor.
    futures::join!(f1, f2);
}

fn test_main() {
    block_on(test_async_main());
}
