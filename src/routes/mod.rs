use mongodb::bson::oid::ObjectId;
use rocket::State;

use crate::database;
use crate::model::Todo;
use rocket::{http::Status, serde::json::Json};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TodoDBO {
    pub(crate) title: String,
    pub(crate) description: String,
}

#[post("/todo", data = "<form>", format = "json")]
pub async fn post_new_todo(
    mut form: Option<Json<TodoDBO>>,
    database: &State<database::MongoDB>,
) -> Result<Status, Status> {
    return match form {
        Some(ref mut form) => {
            if !form.title.is_empty() && !form.description.is_empty() {
                database.add_todo(form).await.ok();
                Ok(Status::Ok)
            } else {
                Err(Status::BadRequest)
            }
        }
        None => Err(Status::InternalServerError),
    };
}

#[get("/todo/<_id>")]
pub async fn get_one_todo(
    _id: String,
    database: &State<database::MongoDB>,
) -> Result<Json<TodoDBO>, Status> {
    match ObjectId::parse_str(&_id) {
        Ok(id) => match database.get_one_todo(id).await {
            Ok(option_todo) => {
                return match option_todo {
                    Some(todo) => Ok(Json(TodoDBO {
                        title: todo.title,
                        description: todo.description,
                    })),
                    None => Err(Status::NotFound),
                }
            }
            Err(error) => {
                println!("{:?}", error);
                Err(Status::NotFound)
            }
        },
        Err(error) => {
            println!("{}", error);
            Err(Status::InternalServerError)
        }
    }
}

#[delete("/todo/<_id>")]
pub async fn delete_one_todo(
    _id: String,
    database: &State<database::MongoDB>,
) -> Result<Status, Status> {
    match ObjectId::parse_str(&_id) {
        Ok(id) => match database.delete_todo(id).await {
            Ok(()) => Ok(Status::Ok),
            Err(error) => {
                println!("{:?}", error);
                Err(Status::NotFound)
            }
        },
        Err(error) => {
            println!("{}", error);
            Err(Status::InternalServerError)
        }
    }
}

#[patch("/todo/<_id>", data = "<form>", format = "json")]
pub async fn patch_todo(
    _id: String,
    database: &State<database::MongoDB>,
    mut form: Option<Json<TodoDBO>>,
) -> Result<Json<String>, Status> {
    match ObjectId::parse_str(&_id) {
        Ok(id) => match form {
            Some(ref mut form) => {
                if !form.title.is_empty() && !form.description.is_empty() {
                    return match database.update_todo(id, form).await.ok() {
                        Some(ok) => Ok(Json(ok)),
                        None => Err(Status::InternalServerError),
                    };
                } else {
                    Err(Status::BadRequest)
                }
            }
            None => Err(Status::InternalServerError),
        },
        Err(error) => {
            println!("----------------");
            println!("error: {:?}", error);
            println!("_id: {:?}", _id);
            println!("form: {:?}", form);
            println!("----------------");
            Err(Status::InternalServerError)
        }
    }
}

#[get("/todo")]
pub async fn get_all_todos(database: &State<database::MongoDB>) -> Result<Json<Vec<Todo>>, Status> {
    return match database.get_all_todos().await {
        Ok(vec_todo) => Ok(Json(vec_todo)),
        Err(error) => {
            println!("----------------");
            println!("error: {:?}", error);
            println!("----------------");
            Err(Status::InternalServerError)
        }
    };
}
