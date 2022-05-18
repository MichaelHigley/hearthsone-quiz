use anyhow::Result;
use axum::*;
use hs_quiz::populate_search_db;
use meilisearch_sdk::client::Client;

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::new("http://localhost:7700", "master-key");

    populate_search_db(&client).await?;

    // let app = Router::new().route("/", routing::get(|| async { "Hello World" }));

    // axum::Server::bind(&"0.0.0.0:3000".parse()?)
    //     .serve(app.into_make_service())
    //     .await?;

    Ok(())
}
