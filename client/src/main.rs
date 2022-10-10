use std::{
    io::{stdin, stdout, Read, Result, Write},
    net::TcpStream,
};

fn main() -> Result<()> {
    let mut stream = TcpStream::connect("0.0.0.0:8080").unwrap();
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();

    stream.write(buf.trim().as_bytes()).unwrap();
    let mut read_buf: [u8; 1024] = [0; 1024];
    let bytes_read = stream.read(&mut read_buf).unwrap();
    if bytes_read > 0 {
        stdout().write(&mut read_buf[0..bytes_read]).unwrap();
    }
    Ok(())
}
