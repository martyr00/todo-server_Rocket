mod private;

use crate::database::private::DB;
use crate::model::Todo;
use mongodb::bson::doc;
use mongodb::options::ClientOptions;
use mongodb::results::InsertOneResult;
use mongodb::{Client, Database};
use rocket::fairing::AdHoc;

pub struct MongoDB {
    database: Database,
}

impl MongoDB {
    fn new(database: Database) -> Self {
        MongoDB { database }
    }
    pub async fn add_todo(&self, todo: &mut Todo) -> mongodb::error::Result<String> {
        let collection = self.database.collection::<Todo>("todo");
        let insert: InsertOneResult = collection.insert_one(todo, None).await?;
        Ok(insert.inserted_id.to_string())
    }
    /*    pub async fn get_todos(&self) -> mongodb::error::Result<Collection<Todo>> {
        let collection = self.database.collection::<Todo>("todo");
        Ok(collection.drop(Todo))
    }

    pub async fn get_one_todo(&self, id: String) -> mongodb::error::Result<Todo> {
        let collection = self.database.collection::<Todo>("todo");
        Ok(collection.find_one(doc! { "_id": id }, None).await?.unwrap())
    }

    pub async fn delete_blog(&self, id: String) -> mongodb::error::Result<()> {
        let collection = self.database.collection::<BlogEntry>("blogs");
        collection.delete_one(doc! { "_id": id }, None).await?;
        Ok(())
    }*/
}

pub async fn init() -> AdHoc {
    AdHoc::on_ignite("Connect to MongoDB cluster", |rocket| async {
        match connect().await {
            Ok(database) => rocket.manage(MongoDB::new(database)),
            Err(error) => {
                panic!("Cannot connect to MDB instance:: {:?}", error)
            }
        }
    })
}

async fn connect() -> mongodb::error::Result<Database> {
    let client_options = ClientOptions::parse(DB).await?;
    let client = Client::with_options(client_options)?;
    // Ping the server to see if you can connect to the cluster
    client
        .database("admin")
        .run_command(doc! {"ping": 1}, None)
        .await?;

    println!("connected to DB");

    Ok(client.database("rust_blog_engine"))
}
