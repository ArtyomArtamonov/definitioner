use std::{error::Error, sync::Arc};

use config::Config;
use controller::Controller;
use teloxide::prelude::*;

mod config;
mod controller;
mod model;
mod repository;
mod service;

#[tokio::main]
async fn main() {
    let config_filepath = "./config.toml";

    let config = Config::from_toml(config_filepath);
    let controller = Arc::new(Controller::new(&config).await);

    let bot = Bot::new(&config.teloxide_token);

    teloxide::repl(bot, move |bot, msg| {
        handler(bot, msg, Arc::clone(&controller))
    })
    .await;
}

async fn handler(bot: Bot, msg: Message, controller: Arc<Controller>) -> ResponseResult<()> {
    let returned: Result<(), Box<dyn Error>> = match msg
        .text()
        .unwrap_or("/help ")
        .split(" ")
        .next()
        .unwrap_or("/help")
        .to_lowercase()
        .as_str()
    {
        "/start" => controller.handle_start(bot, msg).await,
        "/help" => controller.handle_help(bot, msg).await,
        word => controller.handle_word_definition(bot, msg, word).await,
    };

    match returned {
        Ok(_) => {}
        Err(e) => {
            eprintln!("error: {}", e);
        }
    }

    Ok(())
}
