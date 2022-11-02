use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub username: Option<String>,
    pub profile: Option<String>,
    pub banner: Option<String>,
    pub address: String,
    pub nonce: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUser {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub address: String,
    pub nonce: Option<i32>,
}
