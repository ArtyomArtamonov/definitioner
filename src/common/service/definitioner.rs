use std::error::Error;

use crate::common::{api::dictionary, repository::Repository};

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

    pub async fn start(&self, request: request::Start) -> Result<response::Start, Box<dyn Error>> {
        self.repo.insert_profile(&request.profile).await?;

        Ok(response::Start {})
    }

    pub async fn help(&self, _request: request::Help) -> Result<response::Help, Box<dyn Error>> {
        Ok(response::Help {})
    }

    pub async fn word(
        &self,
        request: request::WordDefinition,
    ) -> Result<response::WordDefinition, Box<dyn Error>> {
        if let Some(word_description) = self.repo.get_word(&request.word).await? {
            return Ok(response::WordDefinition {
                description: Some(word_description),
            });
        }

        if let Some(word_description) = self
            .dictionary_api
            .get_word_description(&request.word)
            .await
        {
            self.repo.insert_word(&word_description).await?;

            return Ok(response::WordDefinition {
                description: Some(word_description),
            });
        }

        Ok(response::WordDefinition { description: None })
    }
}
