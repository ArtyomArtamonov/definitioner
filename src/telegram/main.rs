use std::{process::exit, sync::Arc};

use definitioner_bot::common::controller::telegram;
use definitioner_bot::common::{api, config::Config, repository, service::definitioner};
use teloxide::prelude::*;

#[derive(Clone, Default)]
pub enum State {
    #[default]
    Start,
}

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
    let service = Arc::new(definitioner::Definitioner::new(repo, dictionary));

    let bot = Bot::new(&config.teloxide_token);

    telegram::run(bot, service.clone()).await;
}
