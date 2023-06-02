pub struct WordDescription {
    pub word: String,
    pub meanings: Vec<Meaning>,
}

impl WordDescription {
    pub fn fmt(&self) -> String {
        let mut result = format!("{}:\n\n", self.word);
        for meaning in &self.meanings {
            result.push_str(&format!("{}:\n", meaning.part_of_speech));
            for (i, def) in meaning.definitions.iter().enumerate() {
                result.push_str(&format!("{}. {}\n", i + 1, def));
            }
            result.push_str("\n")
        }
        result
    }
}

pub struct Meaning {
    pub part_of_speech: String,
    pub definitions: Vec<String>,
}

pub struct Dictionary {}

impl Dictionary {
    pub fn new() -> Dictionary {
        Dictionary {}
    }

    pub async fn get_word_description(&self, word: &str) -> Option<WordDescription> {
        let url = format!("https://api.dictionaryapi.dev/api/v2/entries/en/{}", word);
        let response = reqwest::get(&url).await.unwrap();
        if response.status() == 404 {
            return None;
        }

        let mut desc = WordDescription {
            word: word.to_owned(),
            meanings: vec![],
        };

        let json = response.json::<serde_json::Value>().await.unwrap();
        let meanings_json = json[0]["meanings"].as_array().unwrap();
        for meaning_json in meanings_json {
            let mut meaning = Meaning {
                part_of_speech: meaning_json["partOfSpeech"].as_str().unwrap().to_string(),
                definitions: vec![],
            };
            let definitions_json = meaning_json["definitions"].as_array().unwrap();
            for def in definitions_json {
                let def = def["definition"].as_str().unwrap();
                meaning.definitions.push(def.to_owned());
            }
            desc.meanings.push(meaning);
        }

        Some(desc)
    }
}
