// ref: https://docs.rs/tokio/latest/tokio/
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::io::{Cursor, Write};

// How to test?
// "curl telnet://127.0.0.1:7777 <<< helo!"
// "echo "foo" | nc 127.0.0.1 7777"
// 
// 修改版本 - 添加一个"[rec: ]"引起的问题
// 遇到一个回车的问题 - 输入自动带一个停止字符
// 当输出的时候，如果不自动带一个回车\n，client端不会输出

// cast: usize as u64
// ref: https://doc.rust-lang.org/rust-by-example/types/cast.html
// float: to_int_unchecked::<T>

pub async fn tcp_server() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:7777").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = [0; 1024];

            // In a loop, read data from the socket and write the data back.
            loop {
                let n = match socket.read(&mut buf).await {
                    // socket closed
                    Ok(n) if n == 0 => return,
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("failed to read from socket; err = {:?}", e);
                        return;
                    }
                };

                let n = n - 1; // to remove the return character
                println!("received: {}", String::from_utf8_lossy(&buf[..n]));

                let mut cursor = Cursor::new(&mut buf[..]);
                cursor.set_position(n as u64);
                write!(cursor, "[rev: {}]\n", n).unwrap(); // add \n at the end

                // Write the data back
                let len = cursor.position() as usize;
                if let Err(e) = socket.write_all(&buf[0..len]).await {
                    eprintln!("failed to write to socket; err = {:?}", e);
                    return;
                }
            }
        });
    }
}