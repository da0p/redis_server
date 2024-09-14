use std::io::Cursor;

use tokio::io::AsyncReadExt;
use tokio::net::TcpListener;

use crate::{Error, BUFFER_SIZE};

pub mod frame;
use frame::Frame;

pub mod parser;

pub mod connection;
use connection::Connection;

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
            let mut connection = Connection::new(inbound_stream);
            tokio::spawn(async move {
                loop {
                    if let Err(err) = connection.read_frame().await {
                        println!("connection error");
                    }
                }
            });
        }
    }
}
