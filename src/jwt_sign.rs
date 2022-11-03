use jwt_simple::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;

#[derive(Serialize, Deserialize)]
pub struct CustomClaim {
    email: String,
}

pub fn create_jwt(email: String) -> String {
    let f = File::open("key").expect("error reading key file");
    let reader = BufReader::new(f);
    let buffer = Vec::new();
    let key = HS256Key::from_bytes(&buffer);
    let customclaim = CustomClaim { email: email };
    let time = Duration::from_hours(1u64);
    let claim = Claims::with_custom_claims(customclaim, time);
    let token = key.authenticate(claim).expect("fail to create token");
    token
}
