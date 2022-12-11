#![allow(non_snake_case)]

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() {
    println!("connecting");
    let mut socket = TcpStream::connect("192.168.108.101:6142")
        .await
        .expect("failed to connect"); // Connect to arduino
    println!("connected");
    let (mut rd, mut wr) = socket.split();

    let mut buf = vec![0; 128];

    loop {
        let mut text: String = String::new();
        std::io::stdin() // Take input
            .read_line(&mut text)
            .expect("Failed to read line");

        if text.trim() == "start" {
            // If start send 1 to arduino to start
            wr.write("1".as_bytes()).await.expect("failed to send");
        } else if text.trim() == "stop" {
            // If stop send 0 to arduino to stop
            wr.write("0".as_bytes()).await.expect("failed to send");
        }
        println!("sent");

        rd.read(&mut buf).await.expect("failed to read"); // Read response from arduino
        println!(
            "responce: {}",
            buf.iter().map(|x| *x as char).collect::<String>()
        );
    }
}
