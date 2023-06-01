use crate::config::Config;
use std::any::Any;
use tokio_postgres::{Client, Error};

mod profile;
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

    fn handle_already_exist_state<T>(&self, e: Error, default: T) -> T
    where
        T: Any,
    {
        match e.code() {
            Some(code) => match code.code() {
                "23505" => {
                    eprintln!("user already exists");
                }
                _ => {
                    eprintln!("error: {}", e);
                }
            },
            None => {
                eprintln!("error: {}", e);
            }
        }

        // eprintln!("error: {}", e);
        // match e {
        //     SqlState(e) => match e.code() {
        //         "23505" => {
        //             eprintln!("user already exists");
        //         }
        //         _ => {
        //             eprintln!("error: {}", e);
        //         }
        //     },
        //     _ => {
        //         eprintln!("error: {}", e);
        //     }
        // }

        default
    }
}
