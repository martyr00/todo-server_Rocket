mod private;

use crate::database::private::DB;
use crate::model::{Todo, TodoGET, TodoWithIdNotDesc};
use crate::TodoDBO;
use chrono::Utc;
use mongodb::{
    bson, bson::oid::ObjectId, options::ClientOptions, results::InsertOneResult, Client, Database,
};
use rocket::{fairing::AdHoc, futures::TryStreamExt};

pub struct MongoDB {
    database: Database,
}

impl MongoDB {
    fn new(database: Database) -> Self {
        MongoDB { database }
    }

    pub async fn add_item(&self, todo: &mut TodoDBO) -> mongodb::error::Result<String> {
        let collection = self.database.collection::<Todo>("todo");
        let insert: InsertOneResult = collection
            .insert_one(
                Todo {
                    title: todo.title.clone(),
                    description: todo.description.clone(),
                    time: Utc::now().time().to_string(),
                },
                None,
            )
            .await?;
        Ok(insert.inserted_id.to_string())
    }

    pub async fn get_all_items(&self) -> mongodb::error::Result<Vec<TodoGET>> {
        let collection = self.database.collection::<TodoWithIdNotDesc>("todo");

        let mut cursor = collection.find(None, None).await?;

        let mut todos: Vec<TodoGET> = vec![];
        while let Some(result) = cursor.try_next().await? {
            let _id = result._id;
            let title = result.title;
            let time = result.time;

            let customer_json = TodoGET {
                _id: _id.to_string(),
                title: title.to_string(),
                time,
            };
            todos.push(customer_json);
        }

        Ok(todos)
    }

    pub async fn get_one_item(&self, id: ObjectId) -> mongodb::error::Result<Option<Todo>> {
        let collection = self.database.collection::<Todo>("todo");
        Ok(collection.find_one(bson::doc! { "_id": id }, None).await?)
    }

    pub async fn delete_item(&self, id: ObjectId) -> mongodb::error::Result<()> {
        let collection = self.database.collection::<Todo>("todo");
        collection
            .delete_one(bson::doc! { "_id": id }, None)
            .await?;
        Ok(())
    }

    pub async fn update_item(
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
                        description: tododbo.description.clone(),
                        time: Utc::now().time().to_string()
                    },
                    None
                )
                .await?
        );
        Ok("ok".to_string())
    }
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
