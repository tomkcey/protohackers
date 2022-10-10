use std::{
    io::{Read, Result, Write},
    net::{TcpListener, TcpStream},
    thread,
};

fn main() -> Result<()> {
    println!("opening socket...");
    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();
    println!("listening on {}...", listener.local_addr().unwrap());

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("{} connected...", stream.peer_addr().unwrap());
        thread::spawn(|| {
            handle_connection(stream);
        });
    }
    Ok(())
}

fn handle_connection(mut stream: TcpStream) {
    loop {
        let mut buf: [u8; 1024] = [0; 1024];
        let bytes_read = stream.read(&mut buf).unwrap();
        if bytes_read == 0 {
            break;
        }
        stream.write(&mut buf[0..bytes_read]).unwrap();
    }
    println!("{} disconnected...", stream.peer_addr().unwrap());
    stream.shutdown(std::net::Shutdown::Both).unwrap();
}
