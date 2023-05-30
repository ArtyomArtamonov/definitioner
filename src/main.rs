use std::sync::Arc;

use config::Config;
use controller::Controller;
use teloxide::prelude::*;

mod config;
mod controller;

#[tokio::main]
async fn main() {
    let config_filepath = "./config.toml";

    let config = Config::from_toml(config_filepath);
    let controller = Arc::new(Controller::new(&config).await);

    let bot = Bot::new(&config.teloxide_token);

    teloxide::repl(bot, move |bot, msg| {
        selector(bot, msg, Arc::clone(&controller))
    })
    .await;
}

async fn selector(bot: Bot, msg: Message, controller: Arc<Controller>) -> ResponseResult<()> {
    match msg
        .text()
        .unwrap_or("/help ")
        .split(" ")
        .next()
        .unwrap_or("/help")
        .to_lowercase()
        .as_str()
    {
        "/start" => {
            bot.send_message(msg.chat.id, "hello there!").await?;
        }
        word => {
            controller.handle_word_definition(bot, msg, word).await;
        }
    };

    Ok(())
}
