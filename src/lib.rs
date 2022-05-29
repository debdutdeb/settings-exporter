pub mod RocketChatSettings {
    use mongodb::{
        bson::{doc, Bson},
        error::Error,
        options::{ClientOptions, FindOptions},
        Client, Collection, Cursor, Database,
    };
    use serde::{Deserialize, Serialize};

    const SETTINGS_DB: &str = &"rocketchat";
    const SETTINGS_COL: &str = &"rocketchat_settings";

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ISetting {}

    pub struct RocketChatMongo {
        client: Client,
        db: Database,
        col: Collection<ISetting>,
    }

    impl RocketChatMongo {
        pub async fn New(connection_string: Option<String>) -> Box<Self> {
            let mut connection_options = ClientOptions::parse(
                connection_string.unwrap_or(String::from("mongodb://mongodb:27017")),
            )
            .await
            .unwrap();

            connection_options.repl_set_name = Some("rs0".to_string());

            let client = Client::with_options(connection_options).unwrap();
            let db = client.database(self::SETTINGS_DB);
            let col = db.collection::<ISetting>(self::SETTINGS_COL);

            Box::new(Self { client, db, col })
        }

        pub async fn find_settings(&self) -> Result<Cursor<ISetting>, Error> {
            let filter = doc! {"type": {"$ne": "group"}, "valueSource": {"$ne": "packageValue"}};
            let sort_options = FindOptions::builder()
                .sort(doc! {"_id": Bson::Int32(1)})
                .build();
            self.col.find(doc! {}, sort_options).await
        }
    }
}
