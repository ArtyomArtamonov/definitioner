use std::{error::Error, process::exit, sync::Arc};

use definitioner_bot::common::{
    api, config::Config, controller::telegram::Controller, repository, service::definitioner,
};
use teloxide::prelude::*;

#[tokio::main]
async fn main() {
    ctrlc::set_handler(move || {
        exit(130);
    })
    .unwrap();

    let config_filepath = "./config.toml";

    let config = Config::from_toml(config_filepath);
    let dictionary = api::dictionary::Dictionary::new();

    let repo = repository::Repository::new(&config).await;
    let service = definitioner::Definitioner::new(repo, dictionary);
    let controller = Arc::new(Controller::new(service).await);

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
