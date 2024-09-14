use crate::server::frame::Frame;
use crate::Error;

use bytes::{Buf, BytesMut};
use tokio::io::{AsyncReadExt, BufWriter};
use tokio::net::TcpStream;

#[derive(Debug)]
pub struct Connection {
    /// TcpStream decorated with `BufWriter`, which provides write
    /// level buffering.
    stream: BufWriter<TcpStream>,

    /// The buffer for reading frames
    buffer: BytesMut,
}

impl Connection {
    pub fn new(socket: TcpStream) -> Connection {
        Connection {
            stream: BufWriter::new(socket),

            buffer: BytesMut::with_capacity(4 * 1024),
        }
    }

    pub(crate) async fn read_frame(&mut self) -> Result<Option<Frame>, Error> {
        loop {
            if let Some(frame) = self.parse_frame()? {
                return Ok(Some(frame));
            }

            if self.stream.read_buf(&mut self.buffer).await? == 0 {
                if self.buffer.is_empty() {
                    return Ok(None);
                } else {
                    return Err("connection reset by peers".into());
                }
            }
        }
    }

    fn parse_frame(&mut self) -> Result<Option<Frame>, Error> {
        use crate::server::frame::Error;
        let mut bytes = std::io::Cursor::new(&self.buffer[..]);
        match Frame::check(&mut bytes) {
            Ok(_) => {
                // Calculate the size to advance later
                let len = bytes.position() as usize;
                // return back to the beginning
                bytes.set_position(0);
                // parse the frame
                let frame = Frame::parse(&mut bytes)?;

                // we need to advance here to remove all the bytes
                self.buffer.advance(len);

                Ok(Some(frame))
            },
            Err(Error::Incomplete) => Ok(None),
            Err(err) => Err(err.into()),
        }
    }
}
