use crate::config::Config;
use crate::controller::definition::Definitioner;
use teloxide::prelude::*;
use tokio_postgres::{Client, NoTls};

mod definition;

pub struct Controller {
    client: Client,
    definitioner: Definitioner,
}

impl Controller {
    pub async fn new(config: &Config) -> Controller {
        let (client, connection) =
            tokio_postgres::connect("host=localhost user=postgres password=postgres", NoTls)
                .await
                .unwrap();

        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });

        Controller {
            client,
            definitioner: Definitioner::new(),
        }
    }

    pub async fn handle_word_definition(&self, bot: Bot, msg: Message, word: &str) {
        if let Some(description) = self.definitioner.get_word_description(word).await {
            bot.send_message(msg.chat.id, description.fmt())
                .await
                .unwrap();
        } else {
            bot.send_message(msg.chat.id, "word not found")
                .await
                .unwrap();
        }
    }
}
