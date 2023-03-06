///! initial process to fetch all of the cards from APIs,
///! then bootstrap the search database used by the frontend to perform lookups
///!
///! *currently using [meilisearch](https://github.com/meilisearch/meilisearch), but could be implemented for any backend*
use anyhow::Result;
use meilisearch_sdk::client::Client;

mod sources;

#[tokio::main]
async fn main() -> Result<()> {
    let host_addr = std::env::var("DB_HOST").unwrap_or_else(|_| "http://localhost:7700".into());
    let api_key = std::env::var("DB_KEY").unwrap_or_else(|_| "masterKey".into());
    let client = Client::new(host_addr, api_key);

    macro_rules! load_source {
        // expand each line of the Source and index name pair into meilisearch load procedures
        [$($source:ty : $index:expr),*] => {
            use crate::sources::ItemSource;

            $(
                let store = client.index($index);

                store
                    .set_searchable_attributes(<$source>::SEARCHABLE_ATTRIBUTES)
                    .await?
                    .wait_for_completion(&client, None, None)
                    .await?;

                store
                    .delete_all_documents()
                    .await?
                    .wait_for_completion(&client, None, None)
                    .await?;

                store
                    .add_documents(&<$source>::fetch_items()?, Some(<$source>::PRIMARY_KEY))
                    .await?
                    .wait_for_completion(&client, None, None)
                    .await?;
            )*
        };
    }

    Ok({
        load_source![sources::Hearthstone : "hearthstone"];
    })
}
