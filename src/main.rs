#[macro_use]
extern crate rocket;

use rocket_db_pools::{Database, Connection};
use rocket::{http::Status, serde::json::Json, serde::Deserialize, serde::Serialize};
use uuid::Uuid;

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

/*#[get("/todo", format = "json")]
async fn get_todos() {
    //todo GET -> todos
}

#[get("/todo/<id>", format = "json")]
async fn get_todo_item(_id: String) {
    //todo GET -> todo_item
}

#[put("/todo/<id>", data = "<task>", format = "json")]
async fn put_todo_item(_id: String, _task: Option<Json<Todo>>) {
    //todo PUT -> (EDIT)todo_item
}

#[delete("/todo/<id>", format = "json")]
async fn delete_todo_item(_id: String) {
//todo PUT -> (delete)todo_item
}*/

// mongodb+srv://admin:admin@cluster0.d5yn0.mongodb.net/myFirstDatabase // admin
#[rocket::main]
async fn main() {
    rocket::build()
        .mount("/api/v1", routes![post_todo, read])
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
}

#[derive(Debug, Serialize)]
struct Error {
    title: String,
    desc: String,
}

#[catch(500)]
fn internal_sever_error() -> Json<Error> {
    Json(Error {
        title: "Internal Server Error".to_string(),
        desc: "The server encountered an internal error while processing this request".to_string(),
    })
}

#[catch(400)]
fn bad_request() -> Json<Error> {
    Json(Error {
        title: "bad_request".to_string(),
        desc: "The server was unable to understand the request due to invalid syntax".to_string(),
    })
}

#[catch(403)]
fn forbidden() -> Json<Error> {
    Json(Error {
        title: "Forbidden".to_string(),
        desc: "You are denied access".to_string(),
    })
}

#[catch(404)]
fn not_found() -> Json<Error> {
    Json(Error {
        title: "Nof found".to_string(),
        desc: "Nof found".to_string(),
    })
}

#[catch(422)]
fn unprocessable_entity() -> Json<Error> {
    Json(Error {
        title: "Unprocessable Entity".to_string(),
        desc: "The request was well-formed but was unable to be followed due to semantic api."
            .to_string(),
    })
}
