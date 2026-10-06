use diesel_async::AsyncPgConnection;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::pooled_connection::deadpool::{Object, Pool};
use dioxus::prelude::ServerFnError;
use std::sync::LazyLock;

type DatabasePool = Pool<AsyncPgConnection>;
type DatabaseConnection = Object<AsyncPgConnection>;

static DATABASE: LazyLock<DatabasePool> = LazyLock::new(|| {
  let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set for the server");
  Pool::builder(AsyncDieselConnectionManager::<AsyncPgConnection>::new(database_url)).build().expect("database pool must initialize")
});

pub async fn connection() -> Result<DatabaseConnection, ServerFnError> {
  DATABASE.get().await.map_err(ServerFnError::new)
}
