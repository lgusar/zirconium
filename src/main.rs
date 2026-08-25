use tokio::net::TcpStream;
use zirconium::{
    Connection,
    message::{Join, Message, Nick, User},
};

#[tokio::main]
async fn main() {
    let stream = TcpStream::connect("localhost:6667").await.unwrap();
    let mut connection = Connection::new(stream);

    connection
        .write_message(Message::from(Nick {
            nickname: "oberst".into(),
        }))
        .await
        .unwrap();

    connection
        .write_message(Message::from(User {
            username: "oberst".into(),
            realname: "oberst".into(),
        }))
        .await
        .unwrap();

    connection
        .write_message(Message::from(Join {
            params: zirconium::message::JoinParams::Channels {
                channels: vec!["#test".into()],
                keys: vec![],
            },
        }))
        .await
        .unwrap();

    loop {
        // let msg = connection
        //     .stream
        //     .read_buf(&mut connection.buffer)
        //     .await
        //     .unwrap();
        if let Some(msg) = connection.read_message().await.unwrap() {
            print!("{}", msg);
        }
    }
}
