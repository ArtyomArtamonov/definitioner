use crate::common::model::profile::Profile;
use crate::common::model::request;
use crate::common::service::definitioner::Definitioner;
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
        Update::filter_message()
            .branch(
                dptree::entry()
                    .filter_command::<SimpleCommand>()
                    .endpoint(handle_simple),
            )
            .branch(
                dptree::filter(|msg: Message| {
                    msg.text()
                        .map(|text| !text.starts_with('/'))
                        .unwrap_or_default()
                })
                .endpoint(handle_word_definition),
            ),
    )
    .dependencies(dptree::deps![service.clone()])
    .enable_ctrlc_handler()
    .build()
    .dispatch()
    .await;
}

async fn handle_simple(
    bot: Bot,
    msg: Message,
    command: SimpleCommand,
    service: Arc<Definitioner>,
) -> HandlerResult {
    match command {
        SimpleCommand::Start => handle_start(bot, msg, service).await,
        SimpleCommand::Help => handle_help(bot, msg, service).await,
    }
}

async fn handle_start(bot: Bot, msg: Message, service: Arc<Definitioner>) -> HandlerResult {
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

async fn handle_help(bot: Bot, msg: Message, service: Arc<Definitioner>) -> HandlerResult {
    service.help(request::Help {}).await.unwrap();

    bot.send_message(msg.chat.id, HELP_MSG).await?;

    Ok(())
}

async fn handle_word_definition(
    bot: Bot,
    msg: Message,
    service: Arc<Definitioner>,
) -> HandlerResult {
    let word = msg.text().unwrap().to_string();
    let resulted_text: String;

    if let Some(def) = service
        .word(request::WordDefinition { word: word })
        .await
        .unwrap()
        .description
    {
        resulted_text = def.fmt();
    } else {
        resulted_text = "No definition found".to_string();
    }

    bot.send_message(msg.chat.id, resulted_text).await?;

    Ok(())
}
