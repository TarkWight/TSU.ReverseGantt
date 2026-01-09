use rand::Rng;
use crate::auth::{hash_password, verify_password};
use crate::infra::errors::AppResult;

pub fn generate_reset_code() -> String {
    let mut rng = rand::thread_rng();
    format!("{:06}", rng.gen_range(0..1_000_000))
}

pub fn hash_reset_code(code: &str) -> AppResult<String> {
    hash_password(code)
}

pub fn verify_reset_code(hash: &str, code: &str) -> AppResult<bool> {
    verify_password(hash, code)
}
