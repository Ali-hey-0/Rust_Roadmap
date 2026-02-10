#[allow(unused)]
use async_std::io::WriteExt;
use async_std::net::TcpListener;
use async_std::{io::Write, net::TcpStream, task::block_on};
use async_std::stream::StreamExt;




async fn handler(mut c: TcpStream) {
    println!("Got new connection ({:?})", c.peer_addr());
    c.write_all("Hello client\n".as_bytes()).await.unwrap();
    async_std::task::sleep(std::time::Duration::from_secs(10)).await;
    c.write_all("ok , going to next\n".as_bytes())
        .await
        .unwrap();
}

async fn run_server() {
    let server: TcpListener = TcpListener::bind("127.0.0.1:9876").await.unwrap();

    let mut connection = server.incoming();
    while let Some(c) = connection.next().await {
        let c = c.unwrap();
        async_std::task::spawn(handler(c));
    }
}

fn main() {
    block_on(run_server());
}