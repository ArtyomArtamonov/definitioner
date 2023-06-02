use crate::config::Config;
use crate::model::profile::Profile;
use crate::repository::Repository;
use crate::service::definition::Definitioner;
use std::error::Error;
use teloxide::prelude::*;

const HELLO_MSG: &str =
    "Hello! I'm a dictionary bot. Send me a word and I'll send you its definition.";
const HELP_MSG: &str = r#"
Available commands: 
/start - start bot
/help - show help
{word} - show word definition
"#;

pub struct Controller {
    repo: Repository,
    definitioner: Definitioner,
}

impl Controller {
    pub async fn new(config: &Config) -> Controller {
        let repo = Repository::new(&config).await;

        Controller {
            repo,
            definitioner: Definitioner::new(),
        }
    }

    pub async fn handle_start(&self, bot: Bot, msg: Message) -> Result<(), Box<dyn Error>> {
        let chat_id = msg.chat.id.0;
        let username = msg.chat.username();

        let profile = Profile {
            id: chat_id,
            name: username.unwrap_or("").to_string(),
        };

        self.repo.insert_profile(&profile).await?;

        bot.send_message(msg.chat.id, HELLO_MSG).await?;

        Ok(())
    }

    pub async fn handle_help(&self, bot: Bot, msg: Message) -> Result<(), Box<dyn Error>> {
        bot.send_message(msg.chat.id, HELP_MSG).await?;

        Ok(())
    }

    pub async fn handle_word_definition(
        &self,
        bot: Bot,
        msg: Message,
        word: &str,
    ) -> Result<(), Box<dyn Error>> {
        if let Some(word_description) = self.repo.get_word(word).await? {
            bot.send_message(msg.chat.id, word_description.fmt())
                .await?;
            return Ok(());
        }

        if let Some(word_description) = self.definitioner.get_word_description(word).await {
            self.repo.insert_word(&word_description).await?;
            bot.send_message(msg.chat.id, word_description.fmt())
                .await?;
            return Ok(());
        }

        bot.send_message(msg.chat.id, "word not found").await?;

        Ok(())
    }
}
