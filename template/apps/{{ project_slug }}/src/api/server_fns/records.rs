use crate::api::db::models::AppRecord;
use dioxus::prelude::*;

#[get("/api/records")]
pub async fn list_records() -> Result<Vec<AppRecord>> {
  crate::api::db::queries::list_records().await
}

#[post("/api/records")]
pub async fn create_record(name: String) -> Result<AppRecord> {
  crate::api::db::queries::create_record(&name).await
}
