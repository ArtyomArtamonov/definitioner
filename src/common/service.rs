pub mod definitioner;

use std::error::Error;

use crate::common::model::{request, response};

#[async_trait::async_trait]
pub trait DefinitionerService {
    async fn start(&self, request: request::Start) -> Result<response::Start, Box<dyn Error>>;
    async fn help(&self, request: request::Help) -> Result<response::Help, Box<dyn Error>>;
    async fn word(
        &self,
        request: request::WordDefinition,
    ) -> Result<response::WordDefinition, Box<dyn Error>>;
}
