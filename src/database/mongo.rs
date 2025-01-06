use mongodb::{ options::{ ClientOptions, ServerApi, ServerApiVersion }, Client, Collection, Database };

pub struct MongoDB {
    pub db: Database,
}

impl MongoDB {
    pub async fn init() -> Self {
        let uri = std::env::var("MONGODB_URI").expect("MONGODB_URI not found in environment variables");
        let database = std::env::var("MONGODB_DATABASE").expect("MONGODB_DATABASE not found in environment variables");
        let mut client_options = ClientOptions::parse(uri).await.unwrap();

        let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
        client_options.server_api = Some(server_api);

        let client = Client::with_options(client_options).unwrap();
        let db = client.database(&database);

        MongoDB { db }
    }
}
