use std::error::Error;

use crate::common::{api::dictionary, repository::Repository};

use super::DefinitionerService;
use crate::common::model::{request, response};

pub struct Definitioner {
    repo: Repository,
    dictionary_api: dictionary::Dictionary,
}

impl Definitioner {
    pub fn new(repo: Repository, dictionary_api: dictionary::Dictionary) -> Self {
        Self {
            repo: repo,
            dictionary_api,
        }
    }
}

#[async_trait::async_trait]
impl DefinitionerService for Definitioner {
    async fn start(&self, request: request::Start) -> Result<response::Start, Box<dyn Error>> {
        self.repo.insert_profile(&request.profile).await?;

        Ok(response::Start {})
    }

    async fn help(&self, _request: request::Help) -> Result<response::Help, Box<dyn Error>> {
        Ok(response::Help {})
    }

    async fn word(
        &self,
        request: request::WordDefinition,
    ) -> Result<response::WordDefinition, Box<dyn Error>> {
        if let Some(word_description) = self.repo.get_word(&request.word).await? {
            todo!("return word definition")
        }

        if let Some(word_description) = self
            .dictionary_api
            .get_word_description(&request.word)
            .await
        {
            self.repo.insert_word(&word_description).await?;
        }

        Ok(response::WordDefinition { description: None })
    }
}
