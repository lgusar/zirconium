use simple_logger::SimpleLogger;
use zirconium::app::App;

const NICKNAME: &str = "test-user";

#[tokio::main]
async fn main() {
    SimpleLogger::new().env().init().unwrap();

    let mut app = App::new();
    app.register(NICKNAME, "localhost:6667").await.unwrap();

    loop {
        todo!()

        // tokio::select! {
        //     app.handle_messages();
        //     handle_user_input() {
        //         app.send_message(server_name, channel_name, data),
        //         app.quit(),
        //         app.join_channel(server_name, channel_name),
        //         app.exit_channel(server_name, channel_name)
        //     }
        // }
    }
}
