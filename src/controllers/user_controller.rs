use crate::{
    helper::{generate_nonce, validate_signature},
    models::user_model::{UpdateUser, User},
    repository::mongodb_repo::{CreateUserRequest, MongoRepo},
};
use mongodb::results::InsertOneResult;
use rocket::{http::Status, serde::json::Json, State};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct NonceResponse {
    nonce: i32,
}

#[derive(Deserialize)]
pub struct VerifyRequest {
    pub address: String,
    pub signature: String,
}

#[get("/<address>")]
pub fn random_nonce(db: &State<MongoRepo>, address: String) -> Result<Json<NonceResponse>, Status> {
    let nonce = generate_nonce();
    let user = db.get_user_by_address(&address.to_owned());
    if user.unwrap().is_none() {
        let data = CreateUserRequest {
            address: address.to_owned(),
            nonce: nonce,
        };
        db.create_user_with_nonce(data);
    } else {
        db.update_nonce(&address, nonce);
    }
    Ok(Json(NonceResponse { nonce: nonce }))
}

#[post("/verifySignature", data = "<payload>")]
pub fn verify_signature(db: &State<MongoRepo>, payload: Json<VerifyRequest>) -> Result<Json<Option<User>>, Status> {
    let user_detail = db.get_user_by_address(&payload.address);
    match user_detail {
        Ok(user) => {
            let result = validate_signature(user.as_ref().unwrap().nonce, payload);
            println!("{}", result);
            Ok(Json(user))
        }
        Err(_) => Err(Status::BadRequest),
    }
    // send jwt token
}

#[post("/", data = "<new_user>")]
pub fn create_user(
    db: &State<MongoRepo>,
    new_user: Json<User>,
) -> Result<Json<InsertOneResult>, Status> {
    let data = User {
        id: None,
        username: None,
        profile: None,
        banner: None,
        address: new_user.address.to_owned(),
        nonce: new_user.nonce.to_owned(),
    };
    let user_detail = db.create_user(data);
    match user_detail {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/")]
pub fn get_all_users(db: &State<MongoRepo>) -> Result<Json<Vec<User>>, Status> {
    let users = db.get_all_users();
    match users {
        Ok(users) => Ok(Json(users)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/<path>")]
pub fn get_user(db: &State<MongoRepo>, path: String) -> Result<Json<User>, Status> {
    let id = path;
    if id.is_empty() {
        return Err(Status::BadRequest);
    };
    let user_detail = db.get_user_by_id(&id);
    match user_detail {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[put("/<path>", data = "<new_user>")]
pub fn update_user(
    db: &State<MongoRepo>,
    path: String,
    new_user: Json<UpdateUser>,
) -> Result<Json<User>, Status> {
    let id = path;
    if id.is_empty() {
        return Err(Status::BadRequest);
    };
    let data = UpdateUser {
        id: None,
        username: new_user.profile.to_owned(),
        profile: new_user.profile.to_owned(),
        banner: new_user.banner.to_owned(),
        address: new_user.address.to_owned(),
    };
    let update_result = db.update_user(&id, data);
    match update_result {
        Ok(update) => {
            if update.matched_count == 1 {
                let updated_user_info = db.get_user_by_id(&id);
                return match updated_user_info {
                    Ok(user) => Ok(Json(user)),
                    Err(_) => Err(Status::InternalServerError),
                };
            } else {
                return Err(Status::NotFound);
            }
        }
        Err(_) => Err(Status::InternalServerError),
    }
}

#[delete("/<path>")]
pub fn delete_user(db: &State<MongoRepo>, path: String) -> Result<Json<&str>, Status> {
    let id = path;
    if id.is_empty() {
        return Err(Status::BadRequest);
    };
    let result = db.delete_user(&id);
    match result {
        Ok(res) => {
            if res.deleted_count == 1 {
                return Ok(Json("User successfully deleted!"));
            } else {
                return Err(Status::NotFound);
            }
        }
        Err(_) => Err(Status::InternalServerError),
    }
}
