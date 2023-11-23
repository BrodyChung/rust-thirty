// ref: https://docs.rs/tokio/latest/tokio/net/struct.UdpSocket.html
use std::io::{Cursor, Write};
use tokio::{net::UdpSocket, sync::mpsc};
use std::{io, net::SocketAddr, sync::Arc};

// use netcat: echo "hi" | nc -cu localhost 7777
// 比较有意思的是nc先发送数据，接着发送停止符 - CRLR-[13 10],也就是说发送一次会接收到两个数据

pub async fn udp_server_1() -> io::Result<()> {
    let sock = UdpSocket::bind("0.0.0.0:7777").await?;
    let mut buf = [0; 1024];
    loop {
        let (len, addr) = sock.recv_from(&mut buf).await?;
        println!("{:?} bytes received from {:?}: {:?}", len, addr, &buf[..len]);

        let mut cursor = Cursor::new(&mut buf[..]);
        cursor.set_position(len as u64);
        write!(cursor, "[rec: {}]", len).unwrap();
        let len = cursor.position() as usize;

        let len = sock.send_to(&buf[..len], addr).await?;
        println!("{:?} bytes sent", len);
    }
    Ok(())
}

// ref: https://docs.rs/tokio/latest/tokio/net/struct.UdpSocket.html
// Arc的使用
// mpsc-多生产单消费 -  let (sender, receiver) = mpsc::channel();
// tx-sender, rx-receiver
// 如下的代码-
// 创建一个udp 端，每接收到一个数据就通过tx发送一个数据，然后另一端rx接收数据并发送出去
pub async fn udp_server() -> io::Result<()> {
    let sock = UdpSocket::bind("0.0.0.0:7777".parse::<SocketAddr>().unwrap()).await?;
    let r = Arc::new(sock);
    let s = r.clone();
    let (tx, mut rx) = mpsc::channel::<(Vec<u8>, SocketAddr)>(1_000);

    tokio::spawn(async move {
        while let Some((bytes, addr)) = rx.recv().await {
            let len = s.send_to(&bytes, &addr).await.unwrap();
            println!("{:?} bytes sent", len);
        }
    });

    let mut buf = [0; 1024];
    loop {
        let (len, addr) = r.recv_from(&mut buf).await?;
        println!("{:?} bytes received from {:?}", len, addr);
        tx.send((buf[..len].to_vec(), addr)).await.unwrap();
    }
}