mod api;
mod jwt_sign;
mod models;
mod repository;

use jwt_sign::create_jwt;
use jwt_simple::prelude::*;
use std::fs::File;
use std::io::{BufWriter, Write};

#[macro_use]
extern crate rocket;
use api::user_api::{create_user, delete_user, get_user, update_user};
use repository::mongodb_repo::MongoRepo;
use rocket::{get, http::Status, serde::json::Json};

#[get("/")]
fn index() -> Result<Json<String>, Status> {
    Ok(Json(String::from("Hello from rust and mongoDB")))
}

#[launch]
fn rocket() -> _ {
    let db = MongoRepo::init();
    rocket::build().manage(db).mount(
        "/user",
        routes![create_user, get_user, update_user, delete_user],
    )
}

fn login() {
    let key = HS256Key::generate();
    let byte_data = key.to_bytes();

    let f = File::create("key").expect("Unable to create  file");
    let mut f = BufWriter::new(f);
    f.write_all(&byte_data).expect("Unable to write data");

    print!("{}", create_jwt("someone@gmail.com".to_string()));
}
