use crate::api::auth::models::{AuthenticatedUser, Credentials};
use dioxus::prelude::*;

#[post("/api/auth/register")]
pub async fn register(credentials: Credentials) -> Result<AuthenticatedUser> {
  let password_hash = crate::api::auth::password::hash(&credentials.password)?;
  let user = crate::api::db::queries::create_user(&credentials.email, &password_hash).await?;
  Ok(AuthenticatedUser { id: user.id, email: user.email })
}

#[post("/api/auth/login")]
pub async fn login(credentials: Credentials) -> Result<AuthenticatedUser> {
  let user = crate::api::db::queries::find_user_by_email(&credentials.email).await?;
  if !crate::api::auth::password::verify(&credentials.password, &user.password_hash)? {
    return Err(ServerFnError::new("invalid credentials"));
  }
  Ok(AuthenticatedUser { id: user.id, email: user.email })
}
