///! initial process to fetch all of the HeathStone cards from APIs,
///! then bootstrap the search database used by the frontend to perform lookups
///!
///! *currently using [meilisearch](https://github.com/meilisearch/meilisearch), but could be implemented for any backend*
///!
use std::{
    collections::{HashMap, HashSet},
    env,
    hash::Hash,
};

use anyhow::Result;
use hs_quiz::{Card, CardSound, PRIMARY_KEY, SEARCHABLE_ATTRIBUTES, SEARCH_INDEX};
use meilisearch_sdk::client::Client;
use serde::Deserialize;

#[derive(Deserialize)]
struct CollectionItem {
    id: String,
    name: String,
    text: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    card_type: String,
}

macro_rules! regex {
    ($re:literal $(,)?) => {{
        static RE: once_cell::sync::OnceCell<regex::Regex> = once_cell::sync::OnceCell::new();
        RE.get_or_init(|| regex::Regex::new($re).unwrap())
    }};
}

impl From<CollectionItem> for Card {
    fn from(item: CollectionItem) -> Self {
        Self {
            id: item.id.clone(),
            name: item.name.clone(),
            text: regex!(r"<.*?>")
                .replace_all(&item.text.as_ref().unwrap_or(&String::default()), "")
                .to_string(),
            image_url: item.get_image_url(),
            sound_urls: item.get_sound_urls(),
        }
    }
}

impl Hash for CollectionItem {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state)
    }
}

impl PartialEq for CollectionItem {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for CollectionItem {}

impl CollectionItem {
    const MINION_TAG: &'static str = "MINION";
    const JSON_CARD_COLLECTION: &'static str =
        "https://api.hearthstonejson.com/v1/latest/enUS/cards.collectible.json";

    async fn get_cards() -> Result<HashSet<Self>> {
        let json_collection_request = &Self::fetch_json_collection().await?;
        let collection_set: HashSet<Self> = serde_json::from_str(&json_collection_request)?;
        let only_minion_cards = collection_set
            .into_iter()
            .filter(|card| card.card_type == Self::MINION_TAG)
            .map(|mut card| {
                // strip the "CORE" off of standard cards
                card.id = card.id.replace("CORE_", "");
                card
            })
            .collect();
        Ok(only_minion_cards)
    }

    async fn fetch_json_collection() -> Result<String> {
        Ok(reqwest::get(Self::JSON_CARD_COLLECTION)
            .await?
            .text()
            .await?)
    }

    fn get_image_url(&self) -> String {
        format!(
            "https://art.hearthstonejson.com/v1/render/latest/enUS/256x/{}.png",
            self.id
        )
    }

    fn get_sound_urls(&self) -> HashMap<CardSound, String> {
        let mut sounds = HashMap::with_capacity(3);
        sounds.insert(
            CardSound::Attack,
            format!(
                "https://hearthstonesounds.s3.amazonaws.com/{}_A.wav",
                self.id
            ),
        );
        sounds.insert(
            CardSound::Death,
            format!(
                "https://hearthstonesounds.s3.amazonaws.com/{}_D.wav",
                self.id
            ),
        );
        sounds.insert(
            CardSound::Play,
            format!(
                "https://hearthstonesounds.s3.amazonaws.com/{}_P.wav",
                self.id
            ),
        );
        sounds
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let host_addr = env::var("DB_HOST").unwrap_or_else(|_| "http://localhost:7700".to_string());
    let api_key = env::var("DB_KEY").unwrap_or_else(|_| "masterKey".to_string());
    let client = Client::new(host_addr, api_key);
    let card_store = client.index(SEARCH_INDEX);

    card_store
        .set_searchable_attributes(SEARCHABLE_ATTRIBUTES)
        .await?
        .wait_for_completion(&client, None, None)
        .await?;

    card_store
        .delete_all_documents()
        .await?
        .wait_for_completion(&client, None, None)
        .await?;

    let card_collection: Vec<Card> = CollectionItem::get_cards()
        .await?
        .into_iter()
        .map(Card::from)
        .collect();

    card_store
        .add_documents(&card_collection, Some(PRIMARY_KEY))
        .await?
        .wait_for_completion(&client, None, None)
        .await?;

    Ok(())
}
