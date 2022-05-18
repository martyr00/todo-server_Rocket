#[macro_use]
extern crate rocket;

mod database;
mod model;
mod routes;

use rocket::{http::Status, serde::Deserialize, serde::json::Json, serde::Serialize};
use uuid::Uuid;
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Todo {
    title: String,
    description: String,
    id: String,
}

#[post("/todo", data = "<task>", format = "json")]
async fn post_todo(task: Option<Json<Todo>>) -> Option<Status> {
    return match task {
        Some(task) => {
            if !task.title.is_empty() && !task.description.is_empty() && task.id.is_empty() {
                Todo {
                    title: task.title.to_string(),
                    description: task.description.to_string(),
                    id: Uuid::new_v4().to_string(),
                };
                Some(Status::Ok)
            } else {
                Some(Status::BadRequest)
            }
        }
        None => Some(Status::InternalServerError),
    };
}

#[rocket::main]
async fn main() -> Result<(), Box<dyn Error>>  {
    rocket::build()
        .attach(database::init().await)
        .mount("/api/v1", routes![post_todo])
        .register(
            "/",
            catchers![
                not_found,
                forbidden,
                unprocessable_entity,
                bad_request,
                internal_sever_error
            ],
        )
        .launch()
        .await;

    Ok(())
}

#[derive(Debug, Serialize)]
struct ServerError {
    title: String,
    desc: String,
}

#[catch(500)]
fn internal_sever_error() -> Json<ServerError> {
    Json(ServerError {
        title: "Internal Server Error".to_string(),
        desc: "The server encountered an internal error while processing this request".to_string(),
    })
}

#[catch(400)]
fn bad_request() -> Json<ServerError> {
    Json(ServerError {
        title: "bad_request".to_string(),
        desc: "The server was unable to understand the request due to invalid syntax".to_string(),
    })
}

#[catch(403)]
fn forbidden() -> Json<ServerError> {
    Json(ServerError {
        title: "Forbidden".to_string(),
        desc: "You are denied access".to_string(),
    })
}

#[catch(404)]
fn not_found() -> Json<ServerError> {
    Json(ServerError {
        title: "Nof found".to_string(),
        desc: "Nof found".to_string(),
    })
}

#[catch(422)]
fn unprocessable_entity() -> Json<ServerError> {
    Json(ServerError {
        title: "Unprocessable Entity".to_string(),
        desc: "The request was well-formed but was unable to be followed due to semantic api."
            .to_string(),
    })
}
