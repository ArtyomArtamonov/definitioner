use async_trait::async_trait;
use teloxide::prelude::*;

use super::ResponseHandler;

pub struct Telegram {
    bot: Bot,
}

impl Telegram {
    pub fn new(token: &str) -> Telegram {
        Telegram {
            bot: Bot::new(token),
        }
    }
}

#[async_trait]
impl ResponseHandler for Telegram {
    async fn handle_start(&self) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    async fn handle_help(&self) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    async fn handle_word_definition(&self) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }
}
