use rocket::form::{Context, Contextual, Form};
use rocket::http::{Cookie, CookieJar};
use rocket::response::{Debug, Redirect};
use rocket::State;
use serde::{Deserialize, Serialize};

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