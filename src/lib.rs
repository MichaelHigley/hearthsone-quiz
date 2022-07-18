use std::{collections::HashMap, hash::Hash};

use serde::{Deserialize, Serialize};

pub const SEARCH_INDEX: &str = "cards";
pub const PRIMARY_KEY: &str = "id";
pub const SEARCHABLE_ATTRIBUTES: [&str; 3] = ["text", "name", "id"];

#[derive(Deserialize, Serialize, Hash, PartialEq, Eq)]
pub enum CardSound {
    Attack,
    Death,
    Play,
}

/// Shape of Card records from collections json
#[derive(Deserialize, Serialize)]
pub struct Card {
    pub id: String,
    pub name: String,
    pub text: String,
    pub image_url: String,
    pub sound_urls: HashMap<CardSound, String>,
}
