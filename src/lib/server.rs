use std::io::Cursor;

use tokio::io::AsyncReadExt;
use tokio::net::TcpListener;

use crate::{Error, BUFFER_SIZE};

pub mod frame;

pub struct RedisServer {
    binding_socket: TcpListener,
}

impl RedisServer {
    pub async fn new(port: u16) -> Result<RedisServer, Error> {
        let address = "127.0.0.1:".to_string() + &port.to_string();
        let listener = TcpListener::bind(address).await?;

        Ok(RedisServer {
            binding_socket: listener,
        })
    }

    pub async fn execute(&self) -> Result<(), Error> {
        loop {
            let (mut inbound_stream, _) = self.binding_socket.accept().await?;

            tokio::spawn(async move {
                let mut buffer = vec![0; BUFFER_SIZE];

                loop {
                    let number_of_bytes = inbound_stream
                        .read(&mut buffer)
                        .await
                        .expect("failed to read data from socket");

                    if number_of_bytes == 0 {
                        return;
                    }

                    let mut bytes = Cursor::new(buffer.as_ref());
                    let frame = frame::Frame::parse(&mut bytes).unwrap();
                    match frame {
                        frame::Frame::Integer(integer) => println!("integer = {}", integer),
                        frame::Frame::Null => println!("nil"),
                        frame::Frame::Bulk(_) => println!("bulk string"),
                        frame::Frame::Simple(simple) => println!("simple string = {}", simple),
                        frame::Frame::Array(_) => println!("array"),
                        _ => println!("error"),
                    }
                }
            });
        }
    }
}
