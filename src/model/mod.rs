use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Todo {
    pub title: String,
    pub description: String,
    pub time: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TodoWithIdNotDesc {
    pub _id: ObjectId,
    pub title: String,
    pub time: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TodoGET {
    pub _id: String,
    pub title: String,
    pub time: String
}