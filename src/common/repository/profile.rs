use super::{util::handle_already_exist_state, Repository};
use crate::common::model::profile::Profile;

use tokio_postgres::Error;

impl Repository {
    pub async fn insert_profile(&self, profile: &Profile) -> Result<(), Error> {
        self.client
            .execute(
                "INSERT INTO profile(id, name) VALUES($1, $2)",
                &[&profile.id, &profile.name],
            )
            .await
            .unwrap_or_else(|e| handle_already_exist_state(e, 0));

        Ok(())
    }
}
