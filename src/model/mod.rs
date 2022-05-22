use mongodb::bson::oid::ObjectId;
use rocket::form::FromForm;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, FromForm, Deserialize, Clone)]
pub struct Todo {
    #[field(validate = len(2..))]
    pub title: String,
    #[field(validate = len(10..))]
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TodoId {
    pub _id: ObjectId,
    pub title: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TodoGET {
    pub _id: String,
    pub title: String,
    pub description: String,
}
