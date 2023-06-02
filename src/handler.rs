use std::error::Error;

pub mod telegram;

#[async_trait::async_trait]
pub trait ResponseHandler {
    async fn handle_start(&self) -> Result<(), Box<dyn Error>>;
    async fn handle_help(&self) -> Result<(), Box<dyn Error>>;
    async fn handle_word_definition(&self) -> Result<(), Box<dyn Error>>;
}
