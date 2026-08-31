use std::{error::Error, fmt::Display};

use log::info;
use tokio::net::TcpStream;

use crate::{
    Connection,
    message::{Message, Nick, Source, User},
};

#[derive(Debug)]
pub struct ChannelMessage {
    pub source: Source,
    pub data: String,
}

#[derive(Debug)]
pub struct Channel {
    pub name: String,
    // mode: Mode TODO: handle channel mode
    pub messages: Vec<ChannelMessage>,
}

#[derive(Debug)]
pub struct Server {
    pub name: String,
    pub connection: Connection,
    pub channels: Vec<Channel>,
}

#[derive(Debug)]
pub struct App {
    pub servers: Vec<Server>,
}

#[derive(Debug)]
pub enum AppError {
    RegisterError(String),
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::RegisterError(addr) => {
                write!(f, "Failed to register to server on address {}", addr)
            }
        }
    }
}

impl Error for AppError {}

impl App {
    pub fn new() -> App {
        App { servers: vec![] }
    }

    pub async fn register(&mut self, nickname: &str, address: &str) -> Result<(), Box<dyn Error>> {
        info!("Registering to {} with nickname {}", address, nickname);

        // TODO: do I need to create TcpStream or should connection create it?
        let stream = TcpStream::connect(address).await?;
        let mut connection = Connection::new(stream);

        connection
            .write_message(Message::from(Nick {
                nickname: nickname.into(),
            }))
            .await?;

        connection
            .write_message(Message::from(User {
                username: nickname.into(),
                realname: nickname.into(),
            }))
            .await?;

        // TODO: registration process

        let server = Server {
            name: "".into(),
            connection,
            channels: vec![],
        };

        self.servers.push(server);
        info!("Registered to {} with nickname {}", address, nickname); // TODO: change address to server.name

        Ok(())
    }
}

impl Default for App {
    fn default() -> Self {
        App::new()
    }
}
