use rocket::form::{FromForm, FromFormField};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, FromForm, Deserialize, Clone)]
pub struct BlogEntry {
    pub id: String,
    #[field(validate = len(2..))]
    pub title: String,
    #[field(validate = len(10..))]
    pub description: String,
}
