use crate::config::Config;
use reqwest::Client;

pub struct Definition {
    pub word: String,
    pub definition: String,
}

pub struct Definitioner {
    client: Client,
}

impl Definitioner {
    pub fn new(config: &Config) -> Definitioner {
        Definitioner {
            client: Client::new(),
        }
    }

    pub async fn get_word_definition(&self, word: &str) -> Definition {
        let url = format!("https://api.dictionaryapi.dev/api/v2/entries/en/{}", word);
        let response = reqwest::get(&url).await.unwrap();
        let json = response.json::<serde_json::Value>().await.unwrap();
        let definition = json[0]["meanings"][0]["definitions"][0]["definition"]
            .as_str()
            .unwrap_or_else(|| "No definition found");
        // let example = json[0]["meanings"][0]["definitions"][0]["example"]
        //     .as_str()
        //     .unwrap();

        Definition {
            word: word.to_string(),
            definition: definition.to_string(),
        }
    }
}
