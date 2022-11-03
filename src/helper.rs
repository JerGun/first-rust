use crate::controllers::user_controller::VerifyRequest;
use hex;
use rand::Rng;
use rocket::serde::json::Json;
use web3::signing::recover;

pub fn generate_nonce() -> i32 {
    let mut rng = rand::thread_rng();
    return rng.gen_range(0..9999999);
}

pub fn validate_signature(nonce: i32, data: Json<VerifyRequest>) -> bool {
    let message = format!("Nonce: {}", nonce);
    let message_hex = hex::encode(message);

    let signature = hex::decode(data.signature.to_owned()).unwrap();
    let pubkey = recover(&message_hex.as_bytes(), &signature[..64], 0);
    assert!(pubkey.is_ok());
    let pubkey = pubkey.unwrap();
    let pubkey = format!("{:02X?}", pubkey);
    assert_eq!(data.address, pubkey);
    return data.address == pubkey;
}
