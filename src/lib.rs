use anyhow::Result;
use futures::future::try_join_all;
use meilisearch_sdk::client::Client;
use reqwest;
use serde::{Deserialize, Serialize};

pub async fn populate_search_db(client: &Client) -> Result<()> {
    let card_store = client.index("cards");

    let card_collection: Vec<StoredCard> =
        try_join_all(Card::get_cards().await?.into_iter().map(|card| async move {
            StoredCard::from_card(
                &card,
                card.get_image_url().await.unwrap(),
                card.get_play_sound_url().await.unwrap(),
            )
        }))
        .await?;

    card_store
        .add_documents(&card_collection, Some("id"))
        .await?;

    Ok(())
}

#[derive(Deserialize)]
struct Card {
    id: String,
    name: String,
    #[serde(rename = "type", skip_serializing)]
    card_type: String,
}

#[derive(Serialize)]
struct StoredCard {
    id: String,
    name: String,
    image_url: String,
    sound_url: String,
}
impl StoredCard {
    fn from_card(card: &Card, image_url: String, sound_url: String) -> Result<Self> {
        Ok(Self {
            id: card.id.clone(),
            name: card.name.clone(),
            image_url,
            sound_url,
        })
    }
}

impl Card {
    const MINION_TAG: &'static str = "MINION";
    const JSON_CARD_COLLECTION: &'static str =
        "https://api.hearthstonejson.com/v1/latest/enUS/cards.collectible.json";

    pub async fn get_cards() -> Result<Vec<Self>> {
        Ok(
            serde_json::from_str::<Vec<Card>>(&Self::fetch_json_collection().await?)?
                .into_iter()
                .filter(|card| card.card_type == Card::MINION_TAG)
                .collect(),
        )
    }

    async fn fetch_json_collection() -> Result<String> {
        Ok(reqwest::get(Self::JSON_CARD_COLLECTION)
            .await?
            .text()
            .await?)
    }

    pub async fn get_image_url(&self) -> Result<String> {
        Ok(format!(
            "https://art.hearthstonejson.com/v1/render/latest/enUS/256x/{}.png",
            self.id
        ))
    }

    pub async fn get_play_sound_url(&self) -> Result<String> {
        Ok(format!(
            // "https://hearthstonesounds.s3.amazonaws.com/{}_(P,A,D).wav",
            "https://hearthstonesounds.s3.amazonaws.com/{}_P.wav",
            self.id
        ))
    }
}
