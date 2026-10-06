use argon2::Argon2;
use argon2::password_hash::{PasswordHash, PasswordHasher, PasswordVerifier};
use dioxus::prelude::ServerFnError;

pub fn hash(password: &str) -> Result<String, ServerFnError> {
  Argon2::default().hash_password(password.as_bytes()).map(|hash| hash.to_string()).map_err(ServerFnError::new)
}

pub fn verify(password: &str, password_hash: &str) -> Result<bool, ServerFnError> {
  let password_hash = PasswordHash::new(password_hash).map_err(ServerFnError::new)?;
  Ok(Argon2::default().verify_password(password.as_bytes(), &password_hash).is_ok())
}
