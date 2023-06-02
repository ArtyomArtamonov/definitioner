use crate::common::api;

pub struct Start {}

pub struct Help {}

pub struct WordDefinition {
    pub description: Option<api::dictionary::WordDescription>,
}
