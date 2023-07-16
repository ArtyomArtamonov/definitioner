use crate::common::model::profile::Profile;
use crate::common::model::request;
use crate::common::service::definitioner::Definitioner;
use crate::common::service::DefinitionerService;
use std::error::Error;
use std::sync::Arc;

use teloxide::prelude::*;
use teloxide::utils::command::BotCommands;

const HELLO_MSG: &str =
    "Hello! I'm a dictionary bot. Send me a word and I'll send you its definition.";
const HELP_MSG: &str = r#"
Available commands: 
/start - start bot
/help - show help
{word} - show word definition
"#;

type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Default commands")]
enum SimpleCommand {
    #[command(description = "shows start message")]
    Start,
    #[command(description = "shows help message")]
    Help,
}

pub async fn run(bot: Bot, service: Arc<Definitioner>) {
    Dispatcher::builder(
        bot,
        Update::filter_message().branch(
            dptree::entry()
                .filter_command::<SimpleCommand>()
                .endpoint(handle_start),
        ),
    )
    .dependencies(dptree::deps![service.clone()])
    .enable_ctrlc_handler()
    .build()
    .dispatch()
    .await;
}

pub async fn handle_start(bot: Bot, msg: Message, service: Arc<Definitioner>) -> HandlerResult {
    let chat_id = msg.chat.id.0;
    let username = msg.chat.username();

    let profile = Profile {
        id: chat_id,
        name: username.unwrap_or("").to_string(),
    };

    service.start(request::Start { profile }).await.unwrap();

    bot.send_message(msg.chat.id, HELLO_MSG).await?;

    Ok(())
}

pub async fn handle_help(bot: Bot, msg: Message, service: Arc<Definitioner>) -> HandlerResult {
    service.help(request::Help {}).await.unwrap();

    bot.send_message(msg.chat.id, HELP_MSG).await?;

    Ok(())
}

pub async fn handle_word_definition(
    bot: Bot,
    msg: Message,
    service: Arc<Definitioner>,
    word: String,
) -> HandlerResult {
    if let Some(def) = service
        .word(request::WordDefinition { word: word })
        .await
        .unwrap()
        .description
    {
        bot.send_message(msg.chat.id, def.fmt()).await?;
        return Ok(());
    }

    bot.send_message(msg.chat.id, "Word not found").await?;

    Ok(())
}
