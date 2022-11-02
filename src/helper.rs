use rand::Rng;

pub fn generate_nonce() -> i32 {
    let mut rng = rand::thread_rng();
    return rng.gen_range(0..9999999);
}
