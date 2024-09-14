use std::io::Cursor;

use tokio::io::AsyncReadExt;
use tokio::net::TcpListener;

use crate::{Error, BUFFER_SIZE};

pub mod frame;
use frame::Frame;

pub mod parser;

pub struct RedisServer {
    binding_socket: TcpListener,
}

impl RedisServer {
    pub async fn new(port: u16) -> Result<RedisServer, Error> {
        let address = format!("127.0.0.1:{}", port);
        let listener = TcpListener::bind(address).await?;

        Ok(RedisServer {
            binding_socket: listener,
        })
    }

    pub async fn run(&self) -> Result<(), Error> {
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
                    if Frame::check(&mut bytes).is_ok() {
                        let frame = Frame::parse(&mut bytes).unwrap();
                        match frame {
                            Frame::Array(_) => println!("hehehehe"),
                            _ => println!("error"),
                        }
                    }
                }
            });
        }
    }
}
