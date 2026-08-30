use std::error::Error;

use log::info;
use simple_logger::SimpleLogger;
use tokio::net::TcpStream;
use zirconium::{
    Connection,
    message::{Command, Join, Message, Nick, PrivMsg, User},
};

const NICKNAME: &str = "test-user";

#[tokio::main]
async fn main() {
    SimpleLogger::new().env().init().unwrap();

    let mut conn_ergo1 = register(NICKNAME.into(), "localhost:6667").await.unwrap();

    conn_ergo1
        .write_message(Message::from(Join {
            params: zirconium::message::JoinParams::Channels {
                channels: vec!["#test".into()],
                keys: vec![],
            },
        }))
        .await
        .unwrap();

    conn_ergo1
        .write_message(Message::from(PrivMsg {
            targets: vec!["#test".into()],
            payload: "test".into(),
        }))
        .await
        .unwrap();

    loop {
        if let Some(msg) = tokio::select! {
            msg = conn_ergo1.read_message() => msg.unwrap(),
            // msg = conn_broski.read_message() => msg.unwrap(),
        } {
            match msg.command {
                Command::PrivMsg(PrivMsg {
                    targets: _,
                    payload,
                }) => {
                    if let Some(source) = msg.source {
                        println!("{}: {}", source.name, payload);
                    } else {
                        println!("{}", payload);
                    }
                }
                _ => {
                    print!("{}", msg);
                }
            }
        }
    }
}

async fn register(nickname: String, addr: &str) -> Result<Connection, Box<dyn Error>> {
    info!("Registering to {} with nickname {}", addr, nickname);

    let stream = TcpStream::connect(addr).await?;
    let mut connection = Connection::new(stream);

    connection
        .write_message(Message::from(Nick {
            nickname: nickname.clone(),
        }))
        .await?;

    connection
        .write_message(Message::from(User {
            username: nickname.clone(),
            realname: nickname.clone(),
        }))
        .await?;

    Ok(connection)
}
