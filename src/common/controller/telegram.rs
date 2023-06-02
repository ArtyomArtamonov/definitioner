use crate::common::model::profile::Profile;
use crate::common::model::request;
use crate::common::service::definitioner::Definitioner;
use crate::common::service::DefinitionerService;
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
    service: Definitioner,
}

impl Controller {
    pub async fn new(service: Definitioner) -> Controller {
        Controller { service }
    }

    pub async fn handle_start(&self, bot: Bot, msg: Message) -> Result<(), Box<dyn Error>> {
        let chat_id = msg.chat.id.0;
        let username = msg.chat.username();

        let profile = Profile {
            id: chat_id,
            name: username.unwrap_or("").to_string(),
        };

        self.service.start(request::Start { profile }).await?;

        bot.send_message(msg.chat.id, HELLO_MSG).await?;

        Ok(())
    }

    pub async fn handle_help(&self, bot: Bot, msg: Message) -> Result<(), Box<dyn Error>> {
        self.service.help(request::Help {}).await?;

        bot.send_message(msg.chat.id, HELP_MSG).await?;

        Ok(())
    }

    pub async fn handle_word_definition(
        &self,
        bot: Bot,
        msg: Message,
        word: &str,
    ) -> Result<(), Box<dyn Error>> {
        if let Some(def) = self
            .service
            .word(request::WordDefinition {
                word: word.to_owned(),
            })
            .await?
            .description
        {
            bot.send_message(msg.chat.id, def.fmt()).await?;
            return Ok(());
        }

        bot.send_message(msg.chat.id, "Word not found").await?;

        Ok(())
    }
}
