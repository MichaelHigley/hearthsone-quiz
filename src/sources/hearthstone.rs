use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

use anyhow::Result;
use regex::Regex;
use serde::{Deserialize, Serialize};

use super::ItemSource;

pub struct Hearthstone;

const MINION_TAG: &'static str = "MINION";
const JSON_CARD_COLLECTION: &'static str =
    "https://api.hearthstonejson.com/v1/latest/enUS/cards.collectible.json";

impl ItemSource<3> for Hearthstone {
    const STORAGE_INDEX: &'static str = "cards";
    const PRIMARY_KEY: &'static str = "id";
    const SEARCHABLE_ATTRIBUTES: [&'static str; 3] = ["text", "name", "id"];

    type Item = Card;

    fn fetch_items() -> Result<Vec<Self::Item>> {
        let collection_json = reqwest::blocking::get(JSON_CARD_COLLECTION)?.text()?;
        let collection_set: HashSet<CollectionItem> = serde_json::from_str(&collection_json)?;

        Ok(collection_set
            .into_iter()
            .filter(|card| card.card_type == MINION_TAG)
            .map(|mut card| {
                // strip the "CORE" off of standard cards
                card.id = card.id.replace("CORE_", "");
                card.into()
            })
            .collect())
    }
}

/// Shape of Card records converted from json
#[derive(Serialize)]
pub struct Card {
    pub id: String,
    pub name: String,
    pub text: String,
    pub image_url: String,
    pub sound_urls: HashMap<String, String>,
}

enum CardSound {
    Attack,
    Death,
    Play,
}

impl CollectionItem<'_> {
    fn get_image_url(&self) -> String {
        format!(
            "https://art.hearthstonejson.com/v1/render/latest/enUS/256x/{}.png",
            self.id.clone()
        )
    }

    fn get_sound_url(&self, card_sound: CardSound) -> String {
        format!(
            "https://hearthstonesounds.s3.amazonaws.com/{}_{}.wav",
            self.id,
            match card_sound {
                CardSound::Attack => "A",
                CardSound::Death => "D",
                CardSound::Play => "P",
            }
        )
    }
}

#[derive(Deserialize)]
struct CollectionItem<'a> {
    id: String,
    name: &'a str,
    text: Option<&'a str>,
    #[serde(rename = "type", skip_serializing)]
    card_type: &'a str,
}

thread_local! {
    /// regex used to replace all instances of html tags
    static CARD_TEXT_REGEX: Regex = Regex::new(r"<.*?>").unwrap()
}

impl From<CollectionItem<'_>> for Card {
    fn from(item: CollectionItem) -> Self {
        Self {
            id: item.id.to_string(),
            name: item.name.to_string(),
            text: CARD_TEXT_REGEX.with(|re| {
                re.replace_all(item.text.unwrap_or_default(), "")
                    .to_string()
            }),
            image_url: item.get_image_url(),
            sound_urls: HashMap::from_iter(
                [
                    ("attack".to_string(), item.get_sound_url(CardSound::Attack)),
                    ("death".to_string(), item.get_sound_url(CardSound::Death)),
                    ("play".to_string(), item.get_sound_url(CardSound::Play)),
                ]
                .into_iter(),
            ),
        }
    }
}

impl Hash for CollectionItem<'_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state)
    }
}

impl PartialEq for CollectionItem<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for CollectionItem<'_> {}
