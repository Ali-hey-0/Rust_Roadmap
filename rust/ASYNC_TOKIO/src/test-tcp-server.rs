#[warn(unused)]
use ::std::net::TcpListener;
use std::{io::Write, net::TcpStream, thread::JoinHandle, time::Duration};

fn main() {
    //socket ,setsocketopt(REUSEPORT),bind,listen,accept -> socket,close
    //unicast , Multicast , Broadcast
    // ip -br a
    //local -> 127.0.0.1 (self)
    //iface ip -> 192.168.1.2 -8.9.10.11
    //0.0.0.0 -> Any face
    //port 1-1024 -> privilege (root)
    //port 1024-65535 (1024-32765   32765-65535)
    //ip :192.168.50.60    5647    5FC6   ip to decimal and hex
    // ipython for linux explain
    // cargo build ... and cargo build --bin ...
    // cargo run ... and cargo run --bin ...
    // explain netstat and usage for this case and  nc explain for this case and tmux
    // nc explain for this case

    let server: TcpListener = TcpListener::bind("127.0.0.1:9876").unwrap();

    let mut workers: Vec<JoinHandle<()>> = vec![];

    // accept
    for conn in server.incoming() {
        let mut conn: TcpStream = conn.unwrap();

        workers.push(std::thread::spawn(move || {
            println!("Got new connection ({:?})", conn.peer_addr());
            conn.write_all("Hello Client\n".as_bytes()).unwrap();
            std::thread::sleep(std::time::Duration::from_secs(10));
            conn.write_all("ok , going to next\n".as_bytes()).unwrap();
            conn.shutdown(std::net::Shutdown::Write).unwrap();
        }));
    }

    for t in workers {
        t.join().unwrap();
    }
}
