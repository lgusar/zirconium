use std::{error::Error, io::Cursor};

use bytes::{Buf, Bytes, BytesMut};
use log::{debug, error};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, BufWriter},
    net::TcpStream,
};

use crate::message::Message;

pub struct Connection {
    stream: BufWriter<TcpStream>,
    buffer: BytesMut,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Connection {
        Connection {
            stream: BufWriter::new(stream),
            buffer: BytesMut::with_capacity(1024),
        }
    }

    fn parse_message(&mut self) -> Result<Option<String>, Box<dyn Error>> {
        let mut buf = Cursor::new(&self.buffer[..]);

        while buf.has_remaining() {
            let byte = buf.get_u8();
            if byte != b'\r' {
                continue;
            }

            if !buf.has_remaining() {
                return Ok(None);
            }
            let byte = buf.get_u8();
            if byte != b'\n' {
                continue;
            }

            let len = buf.position() as usize;
            buf.set_position(0);
            let msg = Bytes::copy_from_slice(&buf.chunk()[..len]);
            self.buffer.advance(len);

            return Ok(Some(String::from_utf8(msg.to_vec())?));
        }

        Ok(None)
    }

    pub async fn read_message(&mut self) -> Result<Option<Message>, Box<dyn Error>> {
        loop {
            if let Some(msg) = self.parse_message()? {
                debug!("received: {}", msg.trim());
                match Message::try_from(msg.clone()) {
                    Ok(msg) => {
                        debug!("parsed message: {}", msg);
                        return Ok(Some(msg));
                    }
                    Err(_) => {
                        error!("could not parse message: {}", msg.trim());
                        // return Err(Box::new(e));
                        return Ok(None);
                    }
                }
            }

            if 0 == self.stream.read_buf(&mut self.buffer).await? {
                if self.buffer.is_empty() {
                    return Ok(None);
                } else {
                    return Err("connection reset by peer".into());
                }
            }
        }
    }

    pub async fn write_message(&mut self, message: Message) -> Result<(), Box<dyn Error>> {
        self.stream
            .write_all(message.to_string().as_bytes())
            .await?;
        self.stream.flush().await?;

        Ok(())
    }
}
