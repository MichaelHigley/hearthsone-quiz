use hs_quiz::populate_search_db;
use meilisearch_sdk::client::*;

#[tokio::test]
async fn test_card_search() {
    let client = Client::new("http://localhost:7700", "master-key");
    populate_search_db(&client).await;

    // client.index("cards").execute_query()
}
