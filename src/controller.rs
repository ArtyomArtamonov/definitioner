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
            definitioner: Definitioner::new(config),
        }
    }

    pub async fn handle_word_definition(&self, bot: Bot, msg: Message, word: &str) {
        let definition = self.definitioner.get_word_definition(word).await;
        bot.send_message(
            msg.chat.id,
            format!("{}: {}", definition.word, definition.definition),
        )
        .await
        .unwrap();
    }
}
