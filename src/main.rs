mod controllers;
mod cors;
mod helper;
mod jwt_sign;
mod models;
mod repository;

use crate::cors::CORS;
use jwt_sign::create_jwt;
use jwt_simple::prelude::*;
use rocket::{http::Status, serde::json::Json};
use std::fs::File;
use std::io::{BufWriter, Write};

#[macro_use]
extern crate rocket;
use controllers::user_controller::{
    create_user, delete_user, get_all_users, get_user, random_nonce, update_user, verify_signature,
};
use repository::mongodb_repo::MongoRepo;

#[get("/")]
fn index() -> Result<Json<String>, Status> {
    Ok(Json(String::from("Hello from rust and mongoDB")))
}

#[launch]
fn rocket() -> _ {
    let db = MongoRepo::init();
    rocket::build()
        .attach(CORS)
        .manage(db)
        .mount("/", routes![index])
        .mount(
            "/user",
            routes![create_user, get_user, update_user, delete_user],
        )
        .mount("/users", routes![get_all_users])
        .mount("/users/nonce", routes![random_nonce])
        .mount("/verifySignature", routes![verify_signature])
}

fn login() {
    let key = HS256Key::generate();
    let byte_data = key.to_bytes();

    let f = File::create("key").expect("Unable to create  file");
    let mut f = BufWriter::new(f);
    f.write_all(&byte_data).expect("Unable to write data");

    print!("{}", create_jwt("someone@gmail.com".to_string()));
}
