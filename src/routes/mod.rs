use rocket::State;

use crate::database;
use crate::model::Todo;
use rocket::{http::Status, serde::json::Json};

#[post("/todo", data = "<form>", format = "json")]
pub async fn post_new_todo(
    mut form: Option<Json<Todo>>,
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

/*#[get("/todo", format = "json")]
pub async fn get_todos(database: &State<database::MongoDB>) -> Result<Json<Collection<Todo>>, Status> {
    let temp = database.get_todos().await.ok();
    match temp {
        Some(temp) => {

        },
        None => Err(Status::InternalServerError),
    }

}
#[put("/todo/<id>", data = "<task>", format = "json")]
async fn put_todo_item(_id: String, _task: Option<Json<Todo>>) {
    //todo PUT -> (EDIT)todo_item
}

#[delete("/todo/<id>", format = "json")]
async fn delete_todo_item(_id: String) {
//todo PUT -> (delete)todo_item
}*/
