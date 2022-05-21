mod private;

use crate::database::private::DB;
use crate::model::Todo;
use crate::TodoDBO;
use mongodb::bson;
use mongodb::bson::oid::ObjectId;
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
    pub async fn add_todo(&self, todo: &mut TodoDBO) -> mongodb::error::Result<String> {
        let temp = Todo {
            title: todo.title.clone(),
            description: todo.description.clone(),
        };
        let collection = self.database.collection::<Todo>("todo");
        let insert: InsertOneResult = collection.insert_one(temp, None).await?;
        Ok(insert.inserted_id.to_string())
    }

    pub async fn get_one_todo(&self, id: ObjectId) -> mongodb::error::Result<Option<Todo>> {
        let collection = self.database.collection::<Todo>("todo");
        Ok(collection
            .find_one(bson::doc! { "_id": id }, None)
            .await?
            )
    }

    pub async fn delete_todo(&self, id: ObjectId) -> mongodb::error::Result<()> {
        let collection = self.database.collection::<Todo>("todo");
        collection
            .delete_one(bson::doc! { "_id": id }, None)
            .await?;
        Ok(())
    }

    pub async fn update_todo(
        &self,
        id: ObjectId,
        tododbo: &mut TodoDBO,
    ) -> mongodb::error::Result<String> {
        let collection = self.database.collection::<Todo>("todo");
        dbg!(
            collection
                .find_one_and_replace(
                    bson::doc! { "_id": id },
                    Todo {
                        title: tododbo.title.clone(),
                        description: tododbo.description.clone()
                    },
                    None
                )
                .await?
        );
        Ok("ok".to_string())
    }

    /*    pub async fn get_todos(&self) -> mongodb::error::Result<Collection<Todo>> {
        let collection = self.database.collection::<Todo>("todo");
        Ok(collection.find(bson::doc! {}, None).await?.unwrap())
    }

    pub async fn update_blog(&self, id:ObjectId, todo: TodoDBO) -> mongodb::error::Result<Todo> {
        let collection = self.database.collection::<Todo>("todo");
        Ok(collection
            .replace_one(bson::doc! { "_id": id }, todo, None)
            .await?
            .unwrap())
    }
    */
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
        .run_command(bson::doc! {"ping": 1}, None)
        .await?;

    println!("connected to DB");

    Ok(client.database("rust_blog_engine"))
}
