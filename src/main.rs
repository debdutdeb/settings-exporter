mod lib;
use futures::TryStreamExt;
use lib::RocketChatSettings::RocketChatMongo;

use tokio;

#[tokio::main]
async fn main() {
    let settings = RocketChatMongo::New(Some("mongodb://localhost:27017".to_string())).await;
    let mut cursor = settings.find_settings().await.unwrap();
    while let Some(setting) = cursor.try_next().await.unwrap() {
        println!("{:?}", setting)
    }
}
