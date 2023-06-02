use crate::common::api::dictionary::WordDescription;

use super::Repository;

impl Repository {
    pub async fn insert_word(&self, word: &WordDescription) -> Result<(), tokio_postgres::Error> {
        // TODO: insert word in one transaction
        Ok(())
    }

    pub async fn get_word(
        &self,
        word: &str,
    ) -> Result<Option<WordDescription>, tokio_postgres::Error> {
        // TODO: try get word from db
        Ok(None)
    }
}
