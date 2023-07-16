use crate::common::api::dictionary::WordDescription;

use super::{util::handle_already_exist_state, Repository};

impl Repository {
    pub async fn insert_word(&self, word: &WordDescription) -> Result<(), tokio_postgres::Error> {
        let tx = self.client.transaction().await?;

        let res = tx
            .query(
                "INSERT INTO word (word) VALUES ($1) RETURNING id",
                &[&word.word],
            )
            .await
            .unwrap_or_else(|e| handle_already_exist_state(e, vec![]));
        if res.len() == 0 || res.len() > 1 {
            tx.rollback().await?;
            return Ok(());
        }

        let word_id = res[0].get::<_, i32>(0);
        for meaning in &word.meanings {
            let res = tx
                .query(
                    "INSERT INTO meaning (word_id, part_of_speech) VALUES ($1, $2) RETURNING id",
                    &[&word_id, &meaning.part_of_speech],
                )
                .await
                .unwrap_or_else(|e| handle_already_exist_state(e, vec![]));
            if res.len() == 0 || res.len() > 1 {
                tx.rollback().await?;
                return Ok(());
            }
            let meaning_id = res[0].get::<_, i32>(0);

            for def in &meaning.definitions {
                let res = tx
                    .execute(
                        "INSERT INTO definition (meaning_id, definition) VALUES ($1, $2)",
                        &[&meaning_id, def],
                    )
                    .await
                    .unwrap_or_else(|e| handle_already_exist_state(e, 0));
                if res == 0 {
                    tx.rollback().await?;
                    return Ok(());
                }
            }
        }

        tx.commit().await?;
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
