use rocket::form::FromForm;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, FromForm, Deserialize, Clone)]
pub struct Todo {
    #[field(validate = len(2..))]
    pub title: String,
    #[field(validate = len(10..))]
    pub description: String,
}
