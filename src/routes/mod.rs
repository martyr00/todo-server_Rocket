use mongodb::bson::oid::ObjectId;
use rocket::State;

use crate::database;
use crate::model::TodoGET;
use rocket::{http::Status, serde::json::Json};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TodoDBO {
    pub(crate) title: String,
    pub(crate) description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TodoDBOWithTime {
    pub(crate) title: String,
    pub(crate) description: String,
    pub(crate) time: String,
}

#[post("/todo", data = "<form>", format = "json")]
pub async fn post_new_item(
    mut form: Option<Json<TodoDBO>>,
    database: &State<database::MongoDB>,
) -> Result<Status, Status> {
    return match form {
        Some(ref mut form) => {
            if get_is_valid_item(form) {
                database.add_item(form).await.ok();
                Ok(Status::Ok)
            } else {
                Err(Status::BadRequest)
            }
        }
        None => Err(Status::InternalServerError),
    };
}

#[get("/todo/<_id>")]
pub async fn get_one_item(
    _id: String,
    database: &State<database::MongoDB>,
) -> Result<Json<TodoDBOWithTime>, Status> {
    match ObjectId::parse_str(&_id) {
        Ok(id) => match database.get_one_item(id).await {
            Ok(option_todo) => {
                return match option_todo {
                    Some(todo) => Ok(Json(TodoDBOWithTime {
                        title: todo.title,
                        description: todo.description,
                        time: todo.time,
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
pub async fn delete_one_item(
    _id: String,
    database: &State<database::MongoDB>,
) -> Result<Status, Status> {
    match ObjectId::parse_str(&_id) {
        Ok(id) => match database.delete_item(id).await {
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
pub async fn patch_item(
    _id: String,
    database: &State<database::MongoDB>,
    mut form: Option<Json<TodoDBO>>,
) -> Result<Json<String>, Status> {
    match ObjectId::parse_str(&_id) {
        Ok(id) => match form {
            Some(ref mut form) => {
                if get_is_valid_item(form) {
                    return match database.update_item(id, form).await.ok() {
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
pub async fn get_all_item(
    database: &State<database::MongoDB>,
) -> Result<Json<Vec<TodoGET>>, Status> {
    return match database.get_all_items().await {
        Ok(vec_todo) => Ok(Json(vec_todo)),
        Err(error) => {
            println!("----------------");
            println!("error: {:?}", error);
            println!("----------------");
            Err(Status::InternalServerError)
        }
    };
}

fn get_is_valid_item(todo: &TodoDBO) -> bool {
    let title = &todo.title;
    let description = &todo.description;

    return if !title.is_empty() && !description.is_empty() {
        if title.len() < 10 && description.len() < 20 {
            true
        } else {
            false
        }
    } else {
        false
    };
}
