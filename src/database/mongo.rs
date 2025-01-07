use async_trait::async_trait;
use mongodb::{ options::{ ClientOptions, ServerApi, ServerApiVersion }, Client, Collection, Database };

pub struct MongoDB {
    pub db: Database
}

#[async_trait]
pub trait MongoDBTrait {
    async fn init() -> Self;
    async fn new_collection<T: Send + Sync>(&self, collection_name: &str) -> Collection<T>;
}

#[async_trait]
impl MongoDBTrait for MongoDB {
    async fn init() -> Self {
        let uri = std::env::var("MONGODB_URI").expect("MONGODB_URI not found in environment variables");
        let database = std::env::var("MONGODB_DATABASE").expect("MONGODB_DATABASE not found in environment variables");
        let mut client_options = ClientOptions::parse(uri).await.unwrap();

        let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
        client_options.server_api = Some(server_api);

        let client = Client::with_options(client_options).unwrap();
        let db = client.database(&database);

        MongoDB { db }
    }

    async fn new_collection<T: Send + Sync>(&self, collection_name: &str) -> Collection<T> {
        self.db.collection(collection_name)
    }
}
