use crate::common::config::Config;
use std::any::Any;
use tokio_postgres::{Client, Error};

mod profile;
mod util;
mod word;

pub struct Repository {
    client: Client,
}

impl Repository {
    pub async fn new(config: &Config) -> Self {
        let (client, connection) = tokio_postgres::connect(
            &format!(
                "host={} user={} password={} dbname={}",
                &config.postgres_host,
                &config.postgres_user,
                &config.postgres_password,
                &config.postgres_dbname,
            ),
            tokio_postgres::NoTls,
        )
        .await
        .unwrap();

        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });

        Self { client }
    }
}
