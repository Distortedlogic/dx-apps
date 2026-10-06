use dioxus::prelude::*;

#[get("/api/status")]
pub async fn server_status() -> Result<String> {
  Ok("Dioxus server is ready.".to_string())
}
